use criterion::{criterion_group, Criterion};

criterion_group!(
    penta_hexa_identical,
    bench_tri_penta_and_hexa,
);

use project_euler::m45::*;

fn bench_tri_penta_and_hexa(c: &mut Criterion) {
    c.bench_function("tri_penta_and_hexa", |b| {
        b.iter(tri_penta_and_hexa)
    });
}
