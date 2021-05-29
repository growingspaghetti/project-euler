use criterion::{criterion_group, Criterion};

criterion_group!(
    double_based_palindromes,
    bench_double_based_palindromes,
    bench_double_based_palindromes_gen
);

use project_euler::m36::*;

fn bench_double_based_palindromes(c: &mut Criterion) {
    c.bench_function("double_based_palindromes_sum", |b| {
        b.iter(double_based_palindromes_sum)
    });
}

fn bench_double_based_palindromes_gen(c: &mut Criterion) {
    c.bench_function("double_based_palindromes_sum_gen", |b| {
        b.iter(double_based_palindromes_sum_gen)
    });
}
