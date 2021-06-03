use criterion::{criterion_group, Criterion};

criterion_group!(
    consective_prime_million,
    bench_consecutive_prime_one_million,
    bench_consective_prime_million_fermat,
);

use project_euler::m50::*;

fn bench_consecutive_prime_one_million(c: &mut Criterion) {
    c.bench_function("consecutive_prime_one_million", |b| {
        b.iter(consecutive_prime_one_million)
    });
}

fn bench_consective_prime_million_fermat(c: &mut Criterion) {
    c.bench_function("consective_prime_million_fermat", |b| {
        b.iter(consective_prime_million_fermat)
    });
}
