use criterion::{criterion_group, Criterion};

criterion_group!(
    pandigital_divisible,
    bench_pandigital_numbers_divisible_substrings,
    bench_pandigital_numbers_divisible_substrings_level,
);

use project_euler::m43::*;

fn bench_pandigital_numbers_divisible_substrings(c: &mut Criterion) {
    c.bench_function("pandigital_numbers_divisible_substrings", |b| {
        b.iter(pandigital_numbers_divisible_substrings)
    });
}

fn bench_pandigital_numbers_divisible_substrings_level(c: &mut Criterion) {
    c.bench_function("pandigital_numbers_divisible_substrings_level", |b| {
        b.iter(pandigital_numbers_divisible_substrings_level)
    });
}
