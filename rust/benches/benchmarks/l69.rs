use criterion::{criterion_group, Criterion};

criterion_group!(
    square_root,
    bench_square_root_brute,
    bench_square_root_newton_32bit_precision,
    bench_square_root_newton_64bit_precision,
    bench_square_root_newton_integer_64bit_precision,
    bench_square_root_newton_integer_32bit_precision,
    bench_square_root_digit_by_digit_decimal,
    bench_square_root_digit_by_digit_binary,
    bench_square_root_bisection_bench,
);

use project_euler::l69::*;

fn bench_square_root_brute(c: &mut Criterion) {
    c.bench_function("square_root_brute", |b| {
        b.iter(square_root_brute_bench)
    });
}

fn bench_square_root_newton_32bit_precision(c: &mut Criterion) {
    c.bench_function("square_root_newton_32bit_precision_bench", |b| {
        b.iter(square_root_newton_32bit_precision_bench)
    });
}

fn bench_square_root_newton_64bit_precision(c: &mut Criterion) {
    c.bench_function("square_root_newton_64bit_precision_bench", |b| {
        b.iter(square_root_newton_64bit_precision_bench)
    });
}

fn bench_square_root_newton_integer_64bit_precision(c: &mut Criterion) {
    c.bench_function("square_root_newton_integer_64_bench", |b| {
        b.iter(square_root_newton_integer_64_bench)
    });
}

fn bench_square_root_newton_integer_32bit_precision(c: &mut Criterion) {
    c.bench_function("square_root_newton_integer_32_bench", |b| {
        b.iter(square_root_newton_integer_32_bench)
    });
}

fn bench_square_root_digit_by_digit_decimal(c: &mut Criterion) {
    c.bench_function("square_root_digit_by_digit_bench", |b| {
        b.iter(square_root_digit_by_digit_bench)
    });
}

fn bench_square_root_digit_by_digit_binary(c: &mut Criterion) {
    c.bench_function("square_root_digit_by_digit_binary_bench", |b| {
        b.iter(square_root_digit_by_digit_binary_bench)
    });
}

fn bench_square_root_bisection_bench(c: &mut Criterion) {
    c.bench_function("square_root_bisection_bench", |b| {
        b.iter(square_root_bisection_bench)
    });
}