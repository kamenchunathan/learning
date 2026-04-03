use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use learning::solutions::leetcode::s3661::Solution as S3661;
use learning::utils::r#gen::s3661::Args;

// ---------------------------------------------------------------------------
// LeetCode 3661 – Maximum Walls Destroyed by Robots
// ---------------------------------------------------------------------------

fn bench_s3661(c: &mut Criterion) {
    // ── fixed examples from the problem statement ───────────────────────────
    let fixed: &[(&str, Vec<i32>, Vec<i32>, Vec<i32>)] = &[
        ("example_1", vec![4], vec![3], vec![1, 10]),
        ("example_2", vec![10, 2], vec![5, 1], vec![5, 2, 7]),
        ("example_3", vec![1, 2], vec![100, 1], vec![10]),
    ];

    let mut group = c.benchmark_group("leetcode/s3661/fixed");
    for (name, robots, distance, walls) in fixed {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &(robots.clone(), distance.clone(), walls.clone()),
            |b, (r, d, w)| {
                b.iter(|| S3661::max_walls(r.clone(), d.clone(), w.clone()));
            },
        );
    }
    group.finish();

    // ── generated inputs at increasing scale ────────────────────────────────
    //
    // Each fixture is produced once before the timing loop so generation
    // cost is not measured — only the solution itself.
    let scaled: &[(&str, Args)] = &[
        ("n=10", Args::small()),
        ("n=1_000", Args::medium()),
        (
            "n=10_000",
            Args::default().with_n(10_000).with_walls(10_000),
        ),
        ("n=100_000", Args::default()), // max-constraint stress test
    ];

    let mut group = c.benchmark_group("leetcode/s3661/generated");
    // Reduce sample count for the large cases so the suite stays fast.
    group.sample_size(20);

    for (label, args) in scaled {
        let input = args.generate();
        group.bench_with_input(BenchmarkId::from_parameter(label), &input, |b, inp| {
            b.iter(|| {
                S3661::max_walls(inp.robots.clone(), inp.distance.clone(), inp.walls.clone())
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_s3661);
criterion_main!(benches);
