use criterion::{criterion_group, Criterion};

criterion_group!(
    primes_for_consecutive_values_of_n,
    bench_of_primes_for_consecutive_values_of_n_brute,
    bench_of_primes_for_consecutive_values_of_n_eratosthenes,
    bench_of_primes_for_consecutive_values_of_n_optimization,
    bench_of_primes_for_consecutive_values_of_n_optimization_2,
);

use project_euler::m27::*;

fn bench_of_primes_for_consecutive_values_of_n_brute(c: &mut Criterion) {
    c.bench_function("of_primes_for_consecutive_values_of_n_brute", |b| {
        b.iter(of_primes_for_consecutive_values_of_n_brute)
    });
}

fn bench_of_primes_for_consecutive_values_of_n_eratosthenes(c: &mut Criterion) {
    c.bench_function("of_primes_for_consecutive_values_of_n_eratosthenes", |b| {
        b.iter(of_primes_for_consecutive_values_of_n_eratosthenes)
    });
}

fn bench_of_primes_for_consecutive_values_of_n_optimization(c: &mut Criterion) {
    c.bench_function("of_primes_for_consecutive_values_of_n_optimization", |b| {
        b.iter(of_primes_for_consecutive_values_of_n_optimization)
    });
}



fn bench_of_primes_for_consecutive_values_of_n_optimization_2(c: &mut Criterion) {
    c.bench_function("of_primes_for_consecutive_values_of_n_optimization_2", |b| {
        b.iter(of_primes_for_consecutive_values_of_n_optimization_2)
    });
}
