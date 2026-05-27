mod shared;

use std::hint::black_box;

use gungraun::prelude::*;
use learning::solutions::leetcode::s3121::Solution as S3121;
use rand::Rng;

use shared::inputs::generate_random_string;

#[library_benchmark]
#[bench::small(generate_random_string(7))]
#[bench::medium(generate_random_string(10_000))]
#[bench::large(generate_random_string(1_000_000))]
fn bench_v1_array(input: String) -> i32 {
    black_box(S3121::number_of_special_chars_v1(black_box(input)))
}

#[library_benchmark]
#[bench::small(generate_random_string(7))]
#[bench::medium(generate_random_string(10_000))]
#[bench::large(generate_random_string(1_000_000))]
fn bench_v2_bitwise_match(input: String) -> i32 {
    black_box(S3121::number_of_special_chars_v2(black_box(input)))
}

#[library_benchmark]
#[bench::small(generate_random_string(7))]
#[bench::medium(generate_random_string(10_000))]
#[bench::large(generate_random_string(1_000_000))]
fn bench_v3_bitwise_ascii(input: String) -> i32 {
    black_box(S3121::number_of_special_chars_v3(black_box(input)))
}

library_benchmark_group!(
    name = s3121_group,
    benchmarks = [
        bench_v1_array,
        bench_v2_bitwise_match,
        bench_v3_bitwise_ascii
    ]
);

main!(library_benchmark_groups = s3121_group);
