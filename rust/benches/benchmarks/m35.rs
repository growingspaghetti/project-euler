use criterion::{criterion_group, Criterion};

criterion_group!(
    circular_primes,
    bench_circular_primes_are_there_below_one_million
);

use project_euler::m35::*;

fn bench_circular_primes_are_there_below_one_million(c: &mut Criterion) {
    c.bench_function("circular_primes_are_there_below_one_million", |b| {
        b.iter(circular_primes_are_there_below_one_million)
    });
}
