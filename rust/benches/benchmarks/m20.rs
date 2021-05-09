use criterion::{criterion_group, Criterion};

criterion_group!(
    factorial_100,
    bench_sum_of_the_digits_in_the_number_factorial_100_2,
);

use project_euler::m20::*;

fn bench_sum_of_the_digits_in_the_number_factorial_100_2(c: &mut Criterion) {
    c.bench_function("sum_of_the_digits_in_the_number_factorial_100_2", |b| {
        b.iter(sum_of_the_digits_in_the_number_factorial_100_2)
    });
}
