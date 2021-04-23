use criterion::{criterion_group, Criterion};

criterion_group!(
    sum_of_even_fibonacci_sequence,
    bench_sum_of_even_fibonacci_sequence_less_than_4000000_polynomial,
    bench_sum_of_even_fibonacci_sequence_less_than_4000000,
    bench_sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8,
);

use project_euler::m2::*;

fn bench_sum_of_even_fibonacci_sequence_less_than_4000000_polynomial(c: &mut Criterion) {
    c.bench_function("sum_of_even_fibonacci_sequence_less_than_4000000_polynomial", |b| {
        b.iter(sum_of_even_fibonacci_sequence_less_than_4000000_polynomial)
    });
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
