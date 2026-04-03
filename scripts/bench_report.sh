#!/usr/bin/env bash
# bench_report.sh
#
# Runs Criterion benchmarks for solutions that have changed since the last
# commit (or all solutions when run with --all), then writes bench_results.json
# which gen_readme.py uses to update the README performance table.
#
# Usage:
#   ./scripts/bench_report.sh          # only changed solutions
#   ./scripts/bench_report.sh --all    # every solution

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RESULTS_FILE="${REPO_ROOT}/bench_results.json"
CRITERION_DIR="${REPO_ROOT}/target/criterion"

# ── helpers ────────────────────────────────────────────────────────────────

log() { echo "[bench_report] $*"; }

# Returns the list of solution modules that have changed vs HEAD.
changed_solutions() {
    git -C "${REPO_ROOT}" diff --name-only HEAD -- 'src/solutions/**/*.rs' \
        | sed 's|src/solutions/||;s|/mod\.rs||;s|\.rs||' \
        | sort -u
}

# ── argument handling ───────────────────────────────────────────────────────

RUN_ALL=false
[[ "${1:-}" == "--all" ]] && RUN_ALL=true

# ── decide which benchmarks to run ─────────────────────────────────────────

if $RUN_ALL; then
    log "Running ALL benchmarks…"
    FILTER=""
else
    CHANGED=$(changed_solutions)
    if [[ -z "$CHANGED" ]]; then
        log "No solution files changed. Pass --all to run everything."
        exit 0
    fi
    log "Changed solutions: $(echo "$CHANGED" | tr '\n' ' ')"
    # Criterion filter: match any benchmark ID that contains a changed module name.
    FILTER=$(echo "$CHANGED" | tr '\n' '|' | sed 's/|$//')
fi

# ── build & run benchmarks ──────────────────────────────────────────────────

cd "${REPO_ROOT}"
log "Compiling…"
cargo build --release -q

log "Running benchmarks…"
if [[ -n "${FILTER:-}" ]]; then
    cargo bench --bench solutions -- "${FILTER}" 2>&1 | tee /tmp/bench_stdout.txt
else
    cargo bench --bench solutions 2>&1 | tee /tmp/bench_stdout.txt
fi

# ── parse Criterion JSON estimates into bench_results.json ─────────────────
#
# Criterion writes one estimates.json per benchmark under:
#   target/criterion/<group>/<id>/new/estimates.json
#
# We collect mean ± std_dev for every benchmark that was just run.

log "Collecting results…"

python3 - <<'PYEOF'
import json, os, pathlib, sys

criterion_dir = pathlib.Path(os.environ.get("CRITERION_DIR", "target/criterion"))
results = {}

for estimates_path in criterion_dir.rglob("new/estimates.json"):
    # path: target/criterion/<group>/<bench_id>/new/estimates.json
    parts = estimates_path.parts
    # find the index of the part after "criterion"
    try:
        idx = parts.index("criterion")
    except ValueError:
        continue
    group   = parts[idx + 1]
    bench_id = parts[idx + 2]

    with open(estimates_path) as f:
        est = json.load(f)

    mean_ns  = est["mean"]["point_estimate"]
    std_ns   = est["std_dev"]["point_estimate"]

    results.setdefault(group, {})[bench_id] = {
        "mean_ns":  mean_ns,
        "std_ns":   std_ns,
        "mean_us":  round(mean_ns / 1_000, 3),
        "std_us":   round(std_ns  / 1_000, 3),
    }

with open("bench_results.json", "w") as f:
    json.dump(results, f, indent=2)

print(f"[bench_report] Wrote bench_results.json ({len(results)} groups).")
PYEOF
