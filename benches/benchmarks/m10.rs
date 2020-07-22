use criterion::{criterion_group, Criterion};

criterion_group!(
    eratosthenes_modified,
    bench_sum_of_all_the_primes_below_two_million,
    bench_sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len,
    bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well,
    bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut,
    bench_sum_of_all_the_primes_below_two_million_bit_vec,
);

use project_euler::m10::*;

fn bench_sum_of_all_the_primes_below_two_million(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million", |b| {
        b.iter(sum_of_all_the_primes_below_two_million)
    });
}

fn bench_sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len(c: &mut Criterion) {
    c.bench_function(
        "sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len",
        |b| b.iter(sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len),
    );
}

fn bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well(c: &mut Criterion) {
    c.bench_function(
        "sum_of_all_the_primes_below_two_million_modified_skip3_as_well",
        |b| b.iter(sum_of_all_the_primes_below_two_million_modified_skip3_as_well),
    );
}

fn bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut(
    c: &mut Criterion,
) {
    c.bench_function(
        "sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut",
        |b| b.iter(sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut),
    );
}

fn bench_sum_of_all_the_primes_below_two_million_bit_vec(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million_bit_vec", |b| {
        b.iter(sum_of_all_the_primes_below_two_million_bit_vec)
    });
}
