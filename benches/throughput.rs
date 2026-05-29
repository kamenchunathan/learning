mod shared;

use std::time::Duration;

use criterion::{
    BenchmarkId, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use criterion_perf_events::Perf;
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

// ---------------------------------------------------------------------------
// LeetCode 3121 – Count the Number of Special Characters II
// ---------------------------------------------------------------------------

use learning::solutions::leetcode::s3121::Solution as S3121;
use perfcnt::linux::{HardwareEventType, PerfCounterBuilderLinux};
use rand::Rng;

fn bench_s3121<M: Measurement>(c: &mut Criterion<M>, metric: &str) {
    let small_input = String::from("aaAbcBC");

    let medium_input = shared::inputs::generate_random_string(10_000);

    let large_input = shared::inputs::generate_random_string(1_000_000);

    // 1. Benchmark small input
    let mut group = c.benchmark_group(format!("leetcode/s3121_{metric}/small"));
    group.bench_with_input("v1_array", &small_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v1(input.clone()));
    });
    group.bench_with_input("v2_bitwise_match", &small_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v2(input.clone()));
    });
    group.bench_with_input("v3_bitwise_ascii", &small_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v3(input.clone()));
    });
    group.bench_with_input("v4_portable_simd", &small_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v4(input.clone()));
    });
    group.bench_with_input("v5_avx2_bmi1", &small_input, |b, input| {
        b.iter(|| unsafe { S3121::number_of_special_chars_v5(input.clone()) });
    });
    group.finish();

    // 2. Benchmark medium input
    let mut group = c.benchmark_group(format!("leetcode/s3121_{metric}/medium"));
    group.bench_with_input("v1_array", &medium_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v1(input.clone()));
    });
    group.bench_with_input("v2_bitwise_match", &medium_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v2(input.clone()));
    });
    group.bench_with_input("v3_bitwise_ascii", &medium_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v3(input.clone()));
    });
    group.bench_with_input("v4_portable_simd", &medium_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v4(input.clone()));
    });
    group.bench_with_input("v5_avx2_bmi1", &medium_input, |b, input| {
        b.iter(|| unsafe { S3121::number_of_special_chars_v5(input.clone()) });
    });
    group.finish();

    // 3. Benchmark large input
    let mut group = c.benchmark_group(format!("leetcode/s3121_{metric}/large"));
    group.sample_size(50);
    group.bench_with_input("v1_array", &large_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v1(input.clone()));
    });
    group.bench_with_input("v2_bitwise_match", &large_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v2(input.clone()));
    });
    group.bench_with_input("v3_bitwise_ascii", &large_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v3(input.clone()));
    });
    group.bench_with_input("v4_portable_simd", &large_input, |b, input| {
        b.iter(|| S3121::number_of_special_chars_v4(input.clone()));
    });
    group.bench_with_input("v5_avx2_bmi1", &large_input, |b, input| {
        b.iter(|| unsafe { S3121::number_of_special_chars_v5(input.clone()) });
    });
    group.finish();
}

fn bench_s3121_time(c: &mut Criterion) {
    bench_s3121(c, "time");
}

fn bench_s3121_cache(c: &mut Criterion<Perf>) {
    bench_s3121(c, "cache_misses");
}

fn bench_s3121_branch(c: &mut Criterion<Perf>) {
    bench_s3121(c, "branch_misses");
}

// ---------------------------------------------------------------------------
// Leetcode 3093. Longest Common Suffix Queries
// ---------------------------------------------------------------------------

mod s3093_bench {
    use criterion::{
        BenchmarkId, Criterion, criterion_group, criterion_main, measurement::Measurement,
    };
    use learning::solutions::leetcode::s3093::{self, generate_worst_case};

    pub fn bench_time(c: &mut Criterion) {
        let b = generate_worst_case(10_000, 10_000, 20);
        let mut group = c.benchmark_group("leetcode/s3093");
        group.bench_with_input("wow", &b, |bench, inp| {
            bench.iter(|| {
                s3093::Solution::string_indices(inp.0.clone(), inp.1.clone());
            })
        });
        group.finish();
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(15));
    targets = bench_s3661, bench_s3121_time, s3093_bench::bench_time
);

criterion_group!(
    name = cache_misses;
    config = Criterion::default()
        .with_measurement(
            Perf::new(PerfCounterBuilderLinux::from_hardware_event(HardwareEventType::CacheMisses))
        );
    targets = bench_s3121_cache,
);

criterion_group!(
    name = branch_mispredictions;
    config = Criterion::default()
        .with_measurement(
            Perf::new(PerfCounterBuilderLinux::from_hardware_event(HardwareEventType::BranchMisses))
        );
    targets = bench_s3121_branch
);

criterion_main!(benches, cache_misses, branch_mispredictions);
