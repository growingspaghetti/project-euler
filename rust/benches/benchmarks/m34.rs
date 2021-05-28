use criterion::{criterion_group, Criterion};

criterion_group!(
    digit_factorials,
    bench_digit_factorials
);

use project_euler::m34::*;

fn bench_digit_factorials(c: &mut Criterion) {
    c.bench_function("digit_factorials_abc", |b| {
        b.iter(digit_factorials_abc)
    });
}
