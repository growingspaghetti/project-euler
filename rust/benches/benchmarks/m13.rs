use criterion::{criterion_group, Criterion};

criterion_group!(
    splice_gigantic_number,
    bench_sum_of_one_hundred_50_digit_numbers,
    bench_sum_of_one_hundred_50_digit_numbers_u64,
    bench_sum_of_one_hundred_50_digit_numbers_u64_3,
);

use project_euler::m13::*;

fn bench_sum_of_one_hundred_50_digit_numbers(c: &mut Criterion) {
    c.bench_function("sum_of_one_hundred_50_digit_numbers", |b| {
        b.iter(sum_of_one_hundred_50_digit_numbers)
    });
}

fn bench_sum_of_one_hundred_50_digit_numbers_u64(c: &mut Criterion) {
    c.bench_function("sum_of_one_hundred_50_digit_numbers_u64", |b| {
        b.iter(sum_of_one_hundred_50_digit_numbers_u64)
    });
}


fn bench_sum_of_one_hundred_50_digit_numbers_u64_3(c: &mut Criterion) {
    c.bench_function("sum_of_one_hundred_50_digit_numbers_u64_3", |b| {
        b.iter(sum_of_one_hundred_50_digit_numbers_u64_3)
    });
}
