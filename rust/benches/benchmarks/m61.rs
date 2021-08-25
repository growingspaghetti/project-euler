use criterion::{criterion_group, Criterion};

criterion_group!(
    polygonal_cyclical_numbers,
    bench_polygonal_cyclical_1,
);

use project_euler::m61::*;

fn bench_polygonal_cyclical_1(c: &mut Criterion) {
    c.bench_function("polygonal_cyclical_numbers_1", |b| {
        b.iter(polygonal_cyclical_numbers_1)
    });
}
