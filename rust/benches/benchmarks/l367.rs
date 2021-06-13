use criterion::{criterion_group, Criterion};

criterion_group!(
    perfect_square,
    bench_perfect_square_brute,
    bench_perfect_square_newton,
    bench_perfect_square_bisection,
    bench_perfect_square_digit_by_digit,
);

use project_euler::l367::*;

fn bench_perfect_square_brute(c: &mut Criterion) {
    c.bench_function("is_perfect_square_brute_bench", |b| {
        b.iter(is_perfect_square_brute_bench)
    });
}

fn bench_perfect_square_newton(c: &mut Criterion) {
    c.bench_function("is_perfect_square_newton_bench", |b| {
        b.iter(is_perfect_square_newton_bench)
    });
}

fn bench_perfect_square_bisection(c: &mut Criterion) {
    c.bench_function("is_perfect_square_bisection_bench", |b| {
        b.iter(is_perfect_square_bisection_bench)
    });
}

fn bench_perfect_square_digit_by_digit(c: &mut Criterion) {
    c.bench_function("is_perfect_square_digit_by_digit_bench", |b| {
        b.iter(is_perfect_square_digit_by_digit_bench)
    });
}
