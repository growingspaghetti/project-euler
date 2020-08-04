use criterion::{black_box, criterion_group, Criterion};

criterion_group!(
    sum_of_dth_powers,
    bench_sum_of_fifth_powers,
    bench_sum_of_fifth_powers_precise,
);

use project_euler::m30::*;

fn bench_sum_of_fifth_powers(c: &mut Criterion) {
    c.bench_function("sum_of_fifth_powers", |b| {
        b.iter(|| sum_of_fifth_powers(black_box(5)))
    });
}

fn bench_sum_of_fifth_powers_precise(c: &mut Criterion) {
    c.bench_function("sum_of_fifth_powers_precise", |b| {
        b.iter(|| sum_of_fifth_powers_precise(black_box(5)))
    });
}
