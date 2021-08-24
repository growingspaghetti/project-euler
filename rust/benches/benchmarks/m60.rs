use criterion::{criterion_group, Criterion};

criterion_group!(
    prime_pair_sets,
    bench_prime_pair_sets_1,
);

use project_euler::m60::*;

fn bench_prime_pair_sets_1(c: &mut Criterion) {
    c.bench_function("prime_set_1", |b| {
        b.iter(prime_set_1)
    });
}
