use criterion::{criterion_group, Criterion};

criterion_group!(
    two_power_thousand,
    bench_the_sum_of_the_digits_of_the_number_2_1000,
    bench_the_sum_of_the_digits_of_the_number_2_1000_carry_bool,
    bench_the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula,
    bench_the_sum_of_the_digits_of_the_number_2_1000_u64,
    bench_the_sum_of_the_digits_of_the_number_2_1000_u64_2,
);

use project_euler::m16::*;

fn bench_the_sum_of_the_digits_of_the_number_2_1000(c: &mut Criterion) {
    c.bench_function("the_sum_of_the_digits_of_the_number_2_1000", |b| {
        b.iter(the_sum_of_the_digits_of_the_number_2_1000)
    });
}

fn bench_the_sum_of_the_digits_of_the_number_2_1000_carry_bool(c: &mut Criterion) {
    c.bench_function(
        "the_sum_of_the_digits_of_the_number_2_1000_carry_bool",
        |b| b.iter(the_sum_of_the_digits_of_the_number_2_1000_carry_bool),
    );
}

fn bench_the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula(c: &mut Criterion) {
    c.bench_function(
        "the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula",
        |b| b.iter(the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula),
    );
}

fn bench_the_sum_of_the_digits_of_the_number_2_1000_u64(c: &mut Criterion) {
    c.bench_function("the_sum_of_the_digits_of_the_number_2_1000_u64", |b| {
        b.iter(the_sum_of_the_digits_of_the_number_2_1000_u64)
    });
}
fn bench_the_sum_of_the_digits_of_the_number_2_1000_u64_2(c: &mut Criterion) {
    c.bench_function("the_sum_of_the_digits_of_the_number_2_1000_u64_2", |b| {
        b.iter(the_sum_of_the_digits_of_the_number_2_1000_u64_2)
    });
}
