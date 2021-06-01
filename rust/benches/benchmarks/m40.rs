use criterion::{criterion_group, Criterion};

criterion_group!(
    champernowne,
    bench_champernownes_constant,
);

use project_euler::m40::*;

fn bench_champernownes_constant(c: &mut Criterion) {
    c.bench_function("champernownes_constant", |b| {
        b.iter(champernownes_constant)
    });
}
