#!/usr/bin/env python3
"""gen_readme.py

Reads bench_results.json produced by bench_report.sh and rewrites the
<!-- BENCH_START --> … <!-- BENCH_END --> section of README.md with an
up-to-date Markdown performance table.

Usage:
    python3 scripts/gen_readme.py
    python3 scripts/gen_readme.py --readme path/to/README.md --results bench_results.json
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys
from datetime import datetime, timezone

# ── sentinel tags used in README.md ────────────────────────────────────────
START_TAG = "<!-- BENCH_START -->"
END_TAG   = "<!-- BENCH_END -->"


def format_time(ns: float) -> str:
    """Return a human-readable duration string."""
    if ns < 1_000:
        return f"{ns:.1f} ns"
    if ns < 1_000_000:
        return f"{ns / 1_000:.2f} µs"
    return f"{ns / 1_000_000:.2f} ms"


def build_table(results: dict) -> str:
    """Convert the results dict into a Markdown table string."""
    lines: list[str] = []
    ts = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    lines.append(f"*Last updated: {ts}*\n")

    for group, benchmarks in sorted(results.items()):
        # Group header derived from Criterion group name, e.g. "leetcode/s3661"
        lines.append(f"### {group}\n")
        lines.append("| Benchmark | Mean | Std Dev |")
        lines.append("|-----------|-----:|--------:|")
        for bench_id, data in sorted(benchmarks.items()):
            mean = format_time(data["mean_ns"])
            std  = format_time(data["std_ns"])
            lines.append(f"| `{bench_id}` | {mean} | ±{std} |")
        lines.append("")  # blank line between groups

    return "\n".join(lines)


def update_readme(readme_path: pathlib.Path, results: dict) -> None:
    text = readme_path.read_text()

    if START_TAG not in text or END_TAG not in text:
        sys.exit(
            f"Could not find {START_TAG!r} / {END_TAG!r} sentinels in {readme_path}.\n"
            "Add them to README.md where you want the performance table."
        )

    table = build_table(results)
    replacement = f"{START_TAG}\n{table}\n{END_TAG}"
    new_text = re.sub(
        re.escape(START_TAG) + ".*?" + re.escape(END_TAG),
        replacement,
        text,
        flags=re.DOTALL,
    )

    readme_path.write_text(new_text)
    print(f"[gen_readme] Updated {readme_path}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--readme",  default="README.md",          help="Path to README.md")
    parser.add_argument("--results", default="bench_results.json", help="Path to bench_results.json")
    args = parser.parse_args()

    results_path = pathlib.Path(args.results)
    readme_path  = pathlib.Path(args.readme)

    if not results_path.exists():
        sys.exit(f"[gen_readme] {results_path} not found. Run bench_report.sh first.")

    results = json.loads(results_path.read_text())
    if not results:
        print("[gen_readme] bench_results.json is empty — nothing to update.")
        return

    update_readme(readme_path, results)


if __name__ == "__main__":
    main()
