use criterion::{criterion_group, Criterion};

criterion_group!(
    sum_of_all_the_multiples,
    bench_sum_of_all_the_multiples_of_3_or_5_below_1000,
    bench_sum_of_all_the_multiples_of_3_or_5_below_1000_iter,
    bench_sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series,
);

use project_euler::m1::*;

fn bench_sum_of_all_the_multiples_of_3_or_5_below_1000(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_multiples_of_3_or_5_below_1000", |b| {
        b.iter(sum_of_all_the_multiples_of_3_or_5_below_1000)
    });
}

fn bench_sum_of_all_the_multiples_of_3_or_5_below_1000_iter(c: &mut Criterion) {
    c.bench_function("sum_of_all_the_multiples_of_3_or_5_below_1000_iter", |b| {
        b.iter(sum_of_all_the_multiples_of_3_or_5_below_1000_iter)
    });
}

fn bench_sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series(c: &mut Criterion) {
    c.bench_function(
        "sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series",
        |b| b.iter(sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series),
    );
}
