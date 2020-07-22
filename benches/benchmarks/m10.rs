use criterion::{criterion_group, Criterion};

criterion_group!(
    eratosthenes_modified,
    bench_sum_of_all_the_primes_below_two_million,
    bench_sum_of_all_the_primes_below_two_million_modified,
    bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well,
);

use project_euler::m10::*;

fn bench_sum_of_all_the_primes_below_two_million(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million", |b| {
        b.iter(sum_of_all_the_primes_below_two_million)
    });
}

fn bench_sum_of_all_the_primes_below_two_million_modified(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million_modified", |b| {
        b.iter(sum_of_all_the_primes_below_two_million_modified)
    });
}

fn bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well(c: &mut Criterion) {
    c.bench_function(
        "sum_of_all_the_primes_below_two_million_modified_skip3_as_well",
        |b| b.iter(sum_of_all_the_primes_below_two_million_modified_skip3_as_well),
    );
}
