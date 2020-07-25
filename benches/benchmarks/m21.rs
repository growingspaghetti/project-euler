use criterion::{criterion_group, Criterion};

criterion_group!(
    sigma_function,
    bench_sum_of_all_the_amicable_numbers_under_10000_brute,
    bench_sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache,
    bench_sum_of_all_the_amicable_numbers_under_10000_prime_factor
);

use project_euler::m21::*;

fn bench_sum_of_all_the_amicable_numbers_under_10000_brute(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_amicable_numbers_under_10000_brute", |b| {
        b.iter(sum_of_all_the_amicable_numbers_under_10000_brute)
    });
}

fn bench_sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache(c: &mut Criterion) {
    c.bench_function(
        "sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache",
        |b| b.iter(sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache),
    );
}

fn bench_sum_of_all_the_amicable_numbers_under_10000_prime_factor(c: &mut Criterion) {
    c.bench_function(
        "sum_of_all_the_amicable_numbers_under_10000_prime_factor",
        |b| b.iter(sum_of_all_the_amicable_numbers_under_10000_prime_factor),
    );
}
