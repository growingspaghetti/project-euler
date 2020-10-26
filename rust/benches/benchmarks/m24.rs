use criterion::{criterion_group, Criterion};

criterion_group!(
    permutation_range,
    bench_millionth_lexicographic_permutation_brute,
    bench_millionth_lexicographic_permutation_factorial,
);

use project_euler::m24::*;

fn bench_millionth_lexicographic_permutation_brute(c: &mut Criterion) {
    c.bench_function("millionth_lexicographic_permutation_brute", |b| {
        b.iter(millionth_lexicographic_permutation_brute)
    });
}

fn bench_millionth_lexicographic_permutation_factorial(c: &mut Criterion) {
    c.bench_function("millionth_lexicographic_permutation_factorial", |b| {
        b.iter(millionth_lexicographic_permutation_factorial)
    });
}
