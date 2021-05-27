use criterion::{criterion_group, Criterion};

criterion_group!(
    pandigital_nine,
    bench_sum_of_all_products_nine_digit_pandigital
);

use project_euler::m32::*;

fn bench_sum_of_all_products_nine_digit_pandigital(c: &mut Criterion) {
    c.bench_function("sum_of_all_products_nine_digit_pandigital", |b| {
        b.iter(sum_of_all_products_nine_digit_pandigital)
    });
}
