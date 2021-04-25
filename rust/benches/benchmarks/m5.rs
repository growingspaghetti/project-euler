use criterion::{criterion_group, Criterion};

criterion_group!(
    least_common_multiple,
    bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_brute,
    bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes,
    bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd,
    bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_brute_syntax,
    bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct,
    bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_format,
);

use project_euler::m5::*;

fn bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_brute(c: &mut Criterion) {
    c.bench_function(
        "smallest_positive_number_that_is_evenly_divisible_by_each_20_brute",
        |b| b.iter(smallest_positive_number_that_is_evenly_divisible_by_each_20_brute),
    );
}

fn bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes(
    c: &mut Criterion,
) {
    c.bench_function(
        "smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes",
        |b| b.iter(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes),
    );
}

fn bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd(c: &mut Criterion) {
    c.bench_function(
        "smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd",
        |b| b.iter(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd),
    );
}
fn bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_brute_syntax(c: &mut Criterion) {
    c.bench_function(
        "smallest_positive_number_that_is_evenly_divisible_by_each_20_brute_syntax",
        |b| b.iter(smallest_positive_number_that_is_evenly_divisible_by_each_20_brute_syntax),
    );
}

fn bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct(c: &mut Criterion) {
    c.bench_function(
        "smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct",
        |b| b.iter(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct),
    );
}


fn bench_smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_format(c: &mut Criterion) {
    c.bench_function(
        "smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_format",
        |b| b.iter(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_format),
    );
}
