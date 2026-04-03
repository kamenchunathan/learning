// There is an endless straight line populated with some robots and walls. You are given integer arrays robots, distance, and walls:
//
//     robots[i] is the position of the ith robot.
//     distance[i] is the maximum distance the ith robot's bullet can travel.
//     walls[j] is the position of the jth wall.
//
// Every robot has one bullet that can either fire to the left or the right at most distance[i] meters.
//
// A bullet destroys every wall in its path that lies within its range. Robots are fixed obstacles: if a bullet hits another robot before reaching a wall, it immediately stops at that robot and cannot continue.
//
// Return the maximum number of unique walls that can be destroyed by the robots.
//
// Notes:
//
//     A wall and a robot may share the same position; the wall can be destroyed by the robot at that position.
//     Robots are not destroyed by bullets.
//
//
//
// Example 1:
//
// Input: robots = [4], distance = [3], walls = [1,10]
//
// Output: 1
//
// Explanation:
//
//     robots[0] = 4 fires left with distance[0] = 3, covering [1, 4] and destroys walls[0] = 1.
//     Thus, the answer is 1.
//
// Example 2:
//
// Input: robots = [10,2], distance = [5,1], walls = [5,2,7]
//
// Output: 3
//
// Explanation:
//
//     robots[0] = 10 fires left with distance[0] = 5, covering [5, 10] and destroys walls[0] = 5 and walls[2] = 7.
//     robots[1] = 2 fires left with distance[1] = 1, covering [1, 2] and destroys walls[1] = 2.
//     Thus, the answer is 3.
//
// Example 3:
//
// Input: robots = [1,2], distance = [100,1], walls = [10]
//
// Output: 0
//
// Explanation:
//
// In this example, only robots[0] can reach the wall, but its shot to the right is blocked by robots[1]; thus the answer is 0.
//
//
//
// Constraints:
//
//     1 <= robots.length == distance.length <= 105
//     1 <= walls.length <= 105
//     1 <= robots[i], walls[j] <= 109
//     1 <= distance[i] <= 105
//     All values in robots are unique
//     All values in walls are unique
//

pub struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let mut destroyed_walls: Vec<(i32, i32)> = vec![];
        for i in 0..robots.len() {
            // Left
            let closes_robot_dist_left = if i > 1 {
                robots[i] - robots[i - 1]
            } else {
                i32::MAX
            };
            let destroyable_range_left = i32::min(closes_robot_dist_left, distance[i]);
            let destroyed_left = walls.iter().fold(0, |acc, wall| {
                if *wall <= robots[i] && robots[i] - wall <= destroyable_range_left {
                    acc + 1
                } else {
                    acc
                }
            });

            crate::dlog!(
                "Robot: {i}, robot left {closes_robot_dist_left}, range {destroyable_range_left}, destroyed {destroyed_left}"
            );

            // Right
            let closes_robot_dist_right = if i < robots.len() - 1 {
                robots[i + 1] - robots[i]
            } else {
                i32::MAX
            };

            let destroyable_range_right = i32::min(closes_robot_dist_right, distance[i]);
            let destroyed_right = walls.iter().fold(0, |acc, wall| {
                if *wall >= robots[i] && wall - robots[i] <= destroyable_range_right {
                    acc + 1
                } else {
                    acc
                }
            });

            crate::dlog!(
                "Robot: {i}, robot right {closes_robot_dist_right}, range {destroyable_range_right}, destroyed {destroyed_right}"
            );

            destroyed_walls.push((destroyed_left, destroyed_right));
        }

        crate::dlog!("Destroyed walls {:?}", destroyed_walls);
        maximize_total_walls(destroyed_walls)
    }
}

fn maximize_total_walls(destroyed_walls: Vec<(i32, i32)>) -> i32 {
    let mut destroyed_walls = destroyed_walls;
    let mut chosen: Vec<bool> = [true].repeat(destroyed_walls.len());
    for i in 0..destroyed_walls.len() {
        let (l, r) = destroyed_walls[i];
        if l > r {
            propagate_left_preference(&mut destroyed_walls[..=i], &mut chosen[..=i]);
        } else {
            chosen[i] = true;
        }
    }

    crate::dlog!("Right wall {:?}", chosen);
    calc_score(&destroyed_walls, &chosen)
}

/// Starting at the right of the slice, Set the right most preference to left (chosen - false)
/// and propagate the results of this choice backwards
///
/// The maximum number of *unique* walls  is required.
/// But a opposing points can face the same direction i.e. right_wall[i - 1] = true and
/// `right_wall[i] = false`. However when a left direction is chosen, we flip the previous
/// direction (ith -  1) to check if this increases the overall score
fn propagate_left_preference(destroyed_walls: &[(i32, i32)], right_wall: &mut [bool]) {
    assert_eq!(destroyed_walls.len(), right_wall.len());

    right_wall[right_wall.len() - 1] = false;
    if destroyed_walls.len() <= 1 {
        return;
    }

    let mut flipped = right_wall.to_owned();
    flipped[right_wall.len() - 2] = false;
    if calc_score(destroyed_walls, &flipped) > calc_score(destroyed_walls, right_wall) {
        right_wall[right_wall.len() - 2] = false;
        let l = right_wall.len();
        propagate_left_preference(
            &destroyed_walls[..destroyed_walls.len() - 1],
            &mut right_wall[..l - 2],
        );
    }
}

fn calc_score(destroyed_walls: &[(i32, i32)], right_wall: &[bool]) -> i32 {
    right_wall
        .iter()
        .zip(destroyed_walls)
        .map(|(&is_right, walls)| if is_right { walls.1 } else { walls.0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_walls(vec![4], vec![3], vec![1, 10]), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]),
            3
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]), 0);
    }
}

// ── property-based tests ────────────────────────────────────────────────────
//
// These use the constraint-aware generator to verify invariants that must hold
// for *any* valid input, not just the hand-picked examples above.
//
// Run with:  cargo test prop_
// Shrinking: proptest automatically minimises counterexamples.
#[cfg(test)]
mod prop_tests {
    use super::*;
    use crate::utils::r#gen::s3661::Args;
    use proptest::prelude::*;

    proptest! {
        /// The result is always non-negative and never exceeds the total
        /// number of walls — a basic sanity check on the return type.
        #[test]
        fn result_within_wall_count(
            input in Args { n_robots: 50, n_walls: 50, max_pos: 10_000, max_dist: 1_000 }
                .strategy()
        ) {
            let n_walls = input.walls.len() as i32;
            let result  = Solution::max_walls(
                input.robots.clone(),
                input.distance.clone(),
                input.walls.clone(),
            );
            prop_assert!(result >= 0,        "result must be non-negative, got {result}");
            prop_assert!(result <= n_walls,  "result {result} exceeds wall count {n_walls}");
        }

        /// A single robot with infinite range and no neighbours should be
        /// able to fire either direction and reach every wall on that side.
        /// Concretely: result > 0 whenever at least one wall is reachable.
        #[test]
        fn single_robot_destroys_reachable_walls(
            input in Args { n_robots: 1, n_walls: 20, max_pos: 10_000, max_dist: 100_000 }
                .strategy()
        ) {
            let result = Solution::max_walls(
                input.robots.clone(),
                input.distance.clone(),
                input.walls.clone(),
            );
            // With one robot and max_dist = MAX_DIST (100 000) vs max_pos = 10 000,
            // the robot can always reach every wall regardless of direction.
            prop_assert!(result == input.walls.len() as i32,
                "single robot with unbounded range should hit all {0} walls, got {result}",
                input.walls.len()
            );
        }

        /// Increasing distance can only weakly increase the result —
        /// a doubled distance array should never produce a lower score.
        #[test]
        fn more_range_never_decreases_score(
            input in Args::small().strategy()
        ) {
            let base = Solution::max_walls(
                input.robots.clone(),
                input.distance.clone(),
                input.walls.clone(),
            );
            let boosted_dist: Vec<i32> = input.distance.iter()
                .map(|&d| (d * 2).min(100_000))
                .collect();
            let boosted = Solution::max_walls(
                input.robots.clone(),
                boosted_dist,
                input.walls.clone(),
            );
            prop_assert!(
                boosted >= base,
                "doubling distance reduced score from {base} to {boosted}"
            );
        }
    }
}
