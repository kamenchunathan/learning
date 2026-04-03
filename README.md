# learning

A Rust workspace for competitive programming practice across multiple platforms.

## Structure

```
src/
  utils/
    log.rs          # dlog! macro — zero-cost in release builds
  solutions/
    leetcode/       # LeetCode problems (sXXXX.rs)
    codeforces/     # Codeforces problems
    rosalind/       # Rosalind bioinformatics problems
benches/
  solutions.rs      # Criterion benchmark suite
scripts/
  bench_report.sh   # Run benchmarks → bench_results.json
  gen_readme.py     # bench_results.json → README performance table
```

## Debug logging

Use `dlog!` anywhere you would use `eprintln!`. It compiles to nothing in
release mode, so you can leave calls in place when submitting to an online
judge and simply build with `--release`:

```rust
use learning::dlog;

dlog!("robot {} fires left, range {}", i, range);
```

| Build mode | `dlog!` behaviour |
|------------|------------------|
| `cargo run` / `cargo test` (debug) | prints to stderr |
| `cargo run --release` / judge submission | no-op, zero overhead |

## Benchmarking

Run benchmarks for solutions changed since the last commit:

```bash
./scripts/bench_report.sh
```

Run all benchmarks regardless of changes:

```bash
./scripts/bench_report.sh --all
```

Then regenerate this README's performance section:

```bash
python3 scripts/gen_readme.py
```

Both steps combined (handy for CI):

```bash
./scripts/bench_report.sh --all && python3 scripts/gen_readme.py
```

Criterion HTML reports are written to `target/criterion/`.

## Performance

<!-- BENCH_START -->
*No benchmark data yet — run `./scripts/bench_report.sh --all && python3 scripts/gen_readme.py`.*
<!-- BENCH_END -->

## Adding a new solution

1. Create `src/solutions/<platform>/<id>.rs` and add `pub mod <id>;` to the
   platform's `mod.rs`.
2. Add a benchmark group to `benches/solutions.rs`.
3. Run `./scripts/bench_report.sh` and `python3 scripts/gen_readme.py` to
   record the baseline performance.
