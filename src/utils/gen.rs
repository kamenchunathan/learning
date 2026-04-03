//! Constraint-aware test data generators.
//!
//! Each problem gets its own `Args` struct whose fields mirror the LeetCode
//! constraint table, plus a `generate` method and a `proptest` strategy so
//! the same type works in both hand-rolled benchmarks and property tests.
//!
//! # Example – manual generation
//! ```rust
//! use learning::utils::gen::s3661::Args;
//!
//! let args = Args::default().generate(); // max-constraint sizes
//! let args = Args { n_robots: 10, n_walls: 5, ..Default::default() }.generate();
//! ```
//!
//! # Example – proptest strategy
//! ```rust,ignore
//! use learning::utils::gen::s3661::Args;
//! use proptest::prelude::*;
//!
//! proptest! {
//!     #[test]
//!     fn my_test(input in Args::default().strategy()) {
//!         // input.robots, input.distance, input.walls are all valid
//!     }
//! }
//! ```

// ── shared helper ───────────────────────────────────────────────────────────

/// Draw `count` distinct values from `1..=max` in sorted order.
///
/// Uses a partial Fisher-Yates shuffle over an index range so it is O(count)
/// rather than O(max), which matters when count << max (e.g. 100 robots out
/// of a 10^9 position space).
pub(crate) fn distinct_sorted(rng: &mut impl rand::Rng, max: u32, count: usize) -> Vec<i32> {
    use rand::seq::index::sample;
    let mut vals: Vec<i32> = sample(rng, max as usize, count)
        .into_iter()
        .map(|i| i as i32 + 1) // shift into 1..=max
        .collect();
    vals.sort_unstable();
    vals
}

// ── per-problem modules ─────────────────────────────────────────────────────

/// Generator for LeetCode 3661 – Maximum Walls Destroyed by Robots.
///
/// Constraints reproduced from the problem statement:
/// * `1 <= robots.length == distance.length <= 10^5`
/// * `1 <= walls.length <= 10^5`
/// * `1 <= robots[i], walls[j] <= 10^9`
/// * `1 <= distance[i] <= 10^5`
/// * All values in `robots` are unique
/// * All values in `walls` are unique
pub mod s3661 {
    use rand::Rng;

    // ── constraint constants ────────────────────────────────────────────────

    pub const MAX_N: usize = 100_000; // robots.length / distance.length
    pub const MAX_W: usize = 100_000; // walls.length
    pub const MAX_POS: u32 = 1_000_000_000;
    pub const MAX_DIST: i32 = 100_000;

    // ── generated input ─────────────────────────────────────────────────────

    /// A fully constraint-valid input triple.
    #[derive(Debug, Clone)]
    pub struct Input {
        pub robots: Vec<i32>,
        pub distance: Vec<i32>,
        pub walls: Vec<i32>,
    }

    // ── generation parameters ────────────────────────────────────────────────

    /// Controls what is generated; tweak fields to target specific cases.
    ///
    /// Defaults produce maximum-constraint inputs (stress / benchmark use).
    #[derive(Debug, Clone)]
    pub struct Args {
        /// Number of robots (and distance entries). `1..=MAX_N`.
        pub n_robots: usize,
        /// Number of walls. `1..=MAX_W`.
        pub n_walls: usize,
        /// Upper bound on position values. `1..=MAX_POS`.
        pub max_pos: u32,
        /// Upper bound on distance values. `1..=MAX_DIST`.
        pub max_dist: i32,
    }

    impl Default for Args {
        /// Maximum-constraint defaults — useful for benchmarks.
        fn default() -> Self {
            Self {
                n_robots: MAX_N,
                n_walls: MAX_W,
                max_pos: MAX_POS,
                max_dist: MAX_DIST,
            }
        }
    }

    impl Args {
        // ── builders ────────────────────────────────────────────────────────

        pub fn small() -> Self {
            Self {
                n_robots: 10,
                n_walls: 10,
                max_pos: 1000,
                max_dist: 100,
            }
        }

        pub fn medium() -> Self {
            Self {
                n_robots: 1_000,
                n_walls: 1_000,
                max_pos: 1_000_000,
                max_dist: 10_000,
            }
        }

        pub fn with_n(mut self, n: usize) -> Self {
            self.n_robots = n.clamp(1, MAX_N);
            self
        }

        pub fn with_walls(mut self, w: usize) -> Self {
            self.n_walls = w.clamp(1, MAX_W);
            self
        }

        // ── generation ──────────────────────────────────────────────────────

        /// Generate a random `Input` from a thread-local RNG.
        pub fn generate(&self) -> Input {
            let mut rng = rand::thread_rng();
            self.generate_with(&mut rng)
        }

        /// Generate a random `Input` from the provided RNG (deterministic
        /// when seeded, useful in tests).
        pub fn generate_with(&self, rng: &mut impl Rng) -> Input {
            assert!(self.n_robots >= 1 && self.n_robots <= MAX_N);
            assert!(self.n_walls >= 1 && self.n_walls <= MAX_W);
            assert!(
                self.max_pos >= self.n_robots as u32,
                "max_pos must be >= n_robots so unique positions are possible"
            );

            let robots = super::distinct_sorted(rng, self.max_pos, self.n_robots);
            let distance = (0..self.n_robots)
                .map(|_| rng.gen_range(1..=self.max_dist))
                .collect();
            let walls = super::distinct_sorted(rng, self.max_pos, self.n_walls);

            Input {
                robots,
                distance,
                walls,
            }
        }

        // ── proptest strategy ───────────────────────────────────────────────

        /// Returns a `proptest` [`Strategy`] that generates valid `Input`s
        /// according to these `Args`.
        ///
        /// The strategy honours all LeetCode constraints, so any counterexample
        /// proptest finds is a genuine bug.
        #[cfg(test)]
        pub fn strategy(self) -> impl proptest::strategy::Strategy<Value = Input> {
            use proptest::prelude::*;

            let Args {
                n_robots,
                n_walls,
                max_pos,
                max_dist,
            } = self;

            // Ranges for sizes: proptest will shrink toward small values.
            let n_robots_range = 1usize..=n_robots;
            let n_walls_range = 1usize..=n_walls;

            (n_robots_range, n_walls_range).prop_flat_map(move |(nr, nw)| {
                // We need nr + nw distinct positions from 1..=max_pos.
                // Generate them together then split, to guarantee no overlap
                // between robots and walls arrays (each internally unique;
                // they *may* share a position per the problem statement, so
                // we don't enforce cross-uniqueness).
                let pos_strat =
                    proptest::sample::subsequence((1u32..=max_pos).collect::<Vec<_>>(), nr);
                let wall_strat =
                    proptest::sample::subsequence((1u32..=max_pos).collect::<Vec<_>>(), nw);
                let dist_strat = proptest::collection::vec(1i32..=max_dist, nr);

                (pos_strat, dist_strat, wall_strat).prop_map(|(mut r, d, mut w)| {
                    r.sort_unstable();
                    w.sort_unstable();
                    Input {
                        robots: r.into_iter().map(|x| x as i32).collect(),
                        distance: d,
                        walls: w.into_iter().map(|x| x as i32).collect(),
                    }
                })
            })
        }
    }
}
