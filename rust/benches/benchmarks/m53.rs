use criterion::{criterion_group, Criterion};

criterion_group!(
    binom_one_million,
    bench_binom_greater_than_one_million,
);

use project_euler::m53::*;

fn bench_binom_greater_than_one_million(c: &mut Criterion) {
    c.bench_function("binom_greater_than_one_million", |b| {
        b.iter(binom_greater_than_one_million)
    });
}
