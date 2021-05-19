use criterion::{criterion_group, Criterion};

criterion_group!(
    abundant_nums_pair,
    bench_sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2,
    bench_sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers,
);

use project_euler::m23::*;

fn bench_sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2(c: &mut Criterion) {
    c.bench_function("sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2", |b| {
        b.iter(sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2)
    });
}

fn bench_sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers(c: &mut Criterion) {
    c.bench_function("sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2", |b| {
        b.iter(sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2)
    });
}
