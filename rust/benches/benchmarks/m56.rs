use criterion::{criterion_group, Criterion};

criterion_group!(
    powerful_digit_sum,
    bench_a_exp_b_100_what_is_the_maximum_digital_sum,
    bench_a_exp_b_100_what_is_the_maximum_digital_sum_reuse,
    bench_a_exp_b_100_what_is_the_maximum_digital_sum_gradual,
);

use project_euler::m56::*;

fn bench_a_exp_b_100_what_is_the_maximum_digital_sum(c: &mut Criterion) {
    c.bench_function("a_exp_b_100_what_is_the_maximum_digital_sum", |b| {
        b.iter(a_exp_b_100_what_is_the_maximum_digital_sum)
    });
}

fn bench_a_exp_b_100_what_is_the_maximum_digital_sum_reuse(c: &mut Criterion) {
    c.bench_function("a_exp_b_100_what_is_the_maximum_digital_sum_reuse", |b| {
        b.iter(a_exp_b_100_what_is_the_maximum_digital_sum_reuse)
    });
}

fn bench_a_exp_b_100_what_is_the_maximum_digital_sum_gradual(c: &mut Criterion) {
    c.bench_function("a_exp_b_100_what_is_the_maximum_digital_sum_gradual", |b| {
        b.iter(a_exp_b_100_what_is_the_maximum_digital_sum_gradual)
    });
}
