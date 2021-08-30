use criterion::{criterion_group, Criterion};

criterion_group!(
    diophantine_equation,
    bench_diophantine_equation_1,
);

use project_euler::m66::*;

fn bench_diophantine_equation_1(c: &mut Criterion) {
    c.bench_function("diophantine_equation_1", |b| {
        b.iter(diophantine_equation_1)
    });
}
