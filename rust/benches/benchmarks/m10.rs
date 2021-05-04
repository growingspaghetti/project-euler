use criterion::{criterion_group, Criterion};

criterion_group!(
    eratosthenes_modified,
    bench_sum_of_all_the_primes_below_two_million,
    bench_sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len,
    bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well,
    bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut,
    bench_sum_of_all_the_primes_below_two_million_bit_vec,
    bench_sum_of_all_the_primes_below_two_million_sieve,
    bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter,
    bench_sum_of_all_the_primes_below_two_million_sieve_step,
    bench_sum_of_all_the_primes_below_two_million_sieve_step_23,
    bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter_2,
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

fn bench_sum_of_all_the_primes_below_two_million_sieve(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million_sieve", |b| {
        b.iter(sum_of_all_the_primes_below_two_million_sieve)
    });
}

fn bench_sum_of_all_the_primes_below_two_million_sieve_step(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million_sieve_step", |b| {
        b.iter(sum_of_all_the_primes_below_two_million_sieve_step)
    });
}

fn bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter(
    c: &mut Criterion,
) {
    c.bench_function(
        "sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter",
        |b| {
            b.iter(sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter)
        },
    );
}


fn bench_sum_of_all_the_primes_below_two_million_sieve_step_23(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million_sieve_step_23", |b| {
        b.iter(sum_of_all_the_primes_below_two_million_sieve_step_23)
    });
}

fn bench_sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter_2(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter_2", |b| {
        b.iter(sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut_iter_2)
    });
}

