use criterion::{criterion_group, Criterion};

criterion_group!(
    coded_triangle_numbers,
    bench_how_many_triangle_words,
);

use project_euler::m42::*;

fn bench_how_many_triangle_words(c: &mut Criterion) {
    c.bench_function("how_many_triangle_words", |b| {
        b.iter(how_many_triangle_words)
    });
}
