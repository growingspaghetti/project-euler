use criterion::{criterion_group, Criterion};

criterion_group!(
    pendatonal_diff_sum,
    bench_sum_and_difference_are_pentagonal,
);

use project_euler::m44::*;

fn bench_sum_and_difference_are_pentagonal(c: &mut Criterion) {
    c.bench_function("sum_and_difference_are_pentagonal", |b| {
        b.iter(sum_and_difference_are_pentagonal)
    });
}
