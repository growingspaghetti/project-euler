use criterion::{criterion_group, Criterion};

criterion_group!(
    recurring_digit_6,
    bench_exactly_the_same_digits_6x,
);

use project_euler::m52::*;

fn bench_exactly_the_same_digits_6x(c: &mut Criterion) {
    c.bench_function("exactly_the_same_digits_6x", |b| {
        b.iter(exactly_the_same_digits_6x)
    });
}
