use criterion::{criterion_group, Criterion};

criterion_group!(
    continued_fraction,
    bench_fractions_contain_a_numerator_with_more_digits,
    bench_fractions_contain_a_numerator_with_more_digits_u64,
    bench_fractions_contain_a_numerator_with_more_digits_bignum,
    bench_fractions_contain_a_numerator_with_more_digits_linear,
);

use project_euler::m57::*;

fn bench_fractions_contain_a_numerator_with_more_digits(c: &mut Criterion) {
    c.bench_function("fractions_contain_a_numerator_with_more_digits", |b| {
        b.iter(fractions_contain_a_numerator_with_more_digits)
    });
}

fn bench_fractions_contain_a_numerator_with_more_digits_u64(c: &mut Criterion) {
    c.bench_function("fractions_contain_a_numerator_with_more_digits_u64", |b| {
        b.iter(fractions_contain_a_numerator_with_more_digits_u64)
    });
}

fn bench_fractions_contain_a_numerator_with_more_digits_bignum(c: &mut Criterion) {
    c.bench_function("fractions_contain_a_numerator_with_more_digits_bignum", |b| {
        b.iter(fractions_contain_a_numerator_with_more_digits_bignum)
    });
}

fn bench_fractions_contain_a_numerator_with_more_digits_linear(c: &mut Criterion) {
    c.bench_function("fractions_contain_a_numerator_with_more_digits_linear", |b| {
        b.iter(fractions_contain_a_numerator_with_more_digits_linear)
    });
}


