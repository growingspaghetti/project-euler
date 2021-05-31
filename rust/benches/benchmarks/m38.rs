use criterion::{criterion_group, Criterion};

criterion_group!(
    pandigital_9_digit_number_concatenated_products,
    bench_pandigital_9_digit_number_concatenated_product,
    bench_pandigital_9_digit_number_concatenated_product_97,
);

use project_euler::m38::*;

fn bench_pandigital_9_digit_number_concatenated_product(c: &mut Criterion) {
    c.bench_function("pandigital_9_digit_number_concatenated_product", |b| {
        b.iter(pandigital_9_digit_number_concatenated_product)
    });
}


fn bench_pandigital_9_digit_number_concatenated_product_97(c: &mut Criterion) {
    c.bench_function("pandigital_9_digit_number_concatenated_product_97", |b| {
        b.iter(pandigital_9_digit_number_concatenated_product_97)
    });
}

