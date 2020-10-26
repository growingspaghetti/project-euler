use criterion::{black_box, criterion_group, Criterion};

criterion_group!(
    pythagorean_triples,
    bench_pythagorean_triplet_and_1000,
    bench_pythagorean_triplet_and_1000_triangle_means_500,
    bench_pythagorean_triplet_and_1000_triangle_euclid,
);

use project_euler::m9::*;

fn bench_pythagorean_triplet_and_1000(c: &mut Criterion) {
    c.bench_function("pythagorean_triplet_and_1000", |b| {
        b.iter(|| pythagorean_triplet_and_1000(black_box(1000)))
    });
}

fn bench_pythagorean_triplet_and_1000_triangle_means_500(c: &mut Criterion) {
    c.bench_function("pythagorean_triplet_and_1000_triangle_means_500", |b| {
        b.iter(|| pythagorean_triplet_and_1000_triangle_means_500(black_box(1000)))
    });
}

fn bench_pythagorean_triplet_and_1000_triangle_euclid(c: &mut Criterion) {
    c.bench_function("pythagorean_triplet_and_1000_triangle_euclid", |b| {
        b.iter(|| pythagorean_triplet_and_1000_triangle_euclid(black_box(1000)))
    });
}
