use criterion::{criterion_group, Criterion};

criterion_group!(
    pandigital_prime,
    bench_largest_pandigital_prime,
);

use project_euler::m41::*;

fn bench_largest_pandigital_prime(c: &mut Criterion) {
    c.bench_function("largest_pandigital_prime", |b| {
        b.iter(largest_pandigital_prime)
    });
}
