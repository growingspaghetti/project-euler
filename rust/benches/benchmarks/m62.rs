use criterion::{criterion_group, Criterion};

criterion_group!(
    cubic_permutations,
    bench_cubic_permutations_1,
);

use project_euler::m62::*;

fn bench_cubic_permutations_1(c: &mut Criterion) {
    c.bench_function("cubic_permutations_1", |b| {
        b.iter(cubic_permutations_1)
    });
}
