use criterion::{criterion_group, black_box, Criterion};

criterion_group!(
    sum_of_even_fibonacci_sequence,
    bench_sum_of_even_fibonacci_sequence_less_than_4000000_polynomial,
    bench_sum_of_even_fibonacci_sequence_less_than_4000000,
    bench_sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8,
    bench_sum_of_even_fibonacci_sequence_less_than_4000000_matrix,
    bench_th_fib,
    bench_th_fib_shift,
);

use project_euler::m2::*;

fn bench_sum_of_even_fibonacci_sequence_less_than_4000000_polynomial(c: &mut Criterion) {
    c.bench_function(
        "sum_of_even_fibonacci_sequence_less_than_4000000_polynomial",
        |b| b.iter(sum_of_even_fibonacci_sequence_less_than_4000000_polynomial),
    );
}

fn bench_sum_of_even_fibonacci_sequence_less_than_4000000(c: &mut Criterion) {
    c.bench_function("sum_of_even_fibonacci_sequence_less_than_4000000", |b| {
        b.iter(sum_of_even_fibonacci_sequence_less_than_4000000)
    });
}

fn bench_sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8(c: &mut Criterion) {
    c.bench_function(
        "sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8",
        |b| b.iter(sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8),
    );
}
fn bench_sum_of_even_fibonacci_sequence_less_than_4000000_matrix(c: &mut Criterion) {
    c.bench_function(
        "sum_of_even_fibonacci_sequence_less_than_4000000_matrix",
        |b| b.iter(sum_of_even_fibonacci_sequence_less_than_4000000_matrix),
    );
}

fn bench_th_fib(c: &mut Criterion) {
    c.bench_function(
        "th_fib",
        |b| b.iter(|| th_fib_matrix(black_box(185))),
    );
}

fn bench_th_fib_shift(c: &mut Criterion) {
    c.bench_function(
        "th_fib_shift",
        |b| b.iter(||th_fib_linear(black_box(185))),
    );
}
