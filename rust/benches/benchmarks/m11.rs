use criterion::{criterion_group, Criterion};

criterion_group!(
    largest_product_in_a_grid,
    bench_greatest_product_of_four_adjacent_numbers,
    bench_greatest_product_of_four_adjacent_numbers_2,
    bench_greatest_product_of_four_adjacent_numbers_3,
    bench_greatest_product_of_four_adjacent_numbers_no_const,
    bench_greatest_product_of_four_adjacent_numbers_no_next,
    bench_greatest_product_of_four_adjacent_numbers_one_loop,
    bench_greatest_product_of_four_adjacent_numbers_no_const_vec,
);

use project_euler::m11::*;

fn bench_greatest_product_of_four_adjacent_numbers_2(c: &mut Criterion) {
    c.bench_function("greatest_product_of_four_adjacent_numbers_2", |b| {
        b.iter(greatest_product_of_four_adjacent_numbers_2)
    });
}

fn bench_greatest_product_of_four_adjacent_numbers(c: &mut Criterion) {
    c.bench_function(
        "greatest_product_of_four_adjacent_numbers",
        |b| b.iter(greatest_product_of_four_adjacent_numbers),
    );
}

fn bench_greatest_product_of_four_adjacent_numbers_3(c: &mut Criterion) {
    c.bench_function(
        "greatest_product_of_four_adjacent_numbers_3",
        |b| b.iter(greatest_product_of_four_adjacent_numbers_3),
    );
}

fn bench_greatest_product_of_four_adjacent_numbers_no_const(c: &mut Criterion) {
    c.bench_function(
        "greatest_product_of_four_adjacent_numbers_no_const",
        |b| b.iter(greatest_product_of_four_adjacent_numbers_no_const),
    );
}

fn bench_greatest_product_of_four_adjacent_numbers_no_next(c: &mut Criterion) {
    c.bench_function(
        "greatest_product_of_four_adjacent_numbers_no_next",
        |b| b.iter(greatest_product_of_four_adjacent_numbers_no_next),
    );
}

fn bench_greatest_product_of_four_adjacent_numbers_one_loop(c: &mut Criterion) {
    c.bench_function(
        "greatest_product_of_four_adjacent_numbers_one_loop",
        |b| b.iter(greatest_product_of_four_adjacent_numbers_one_loop),
    );
}

fn bench_greatest_product_of_four_adjacent_numbers_no_const_vec(c: &mut Criterion) {
    c.bench_function(
        "greatest_product_of_four_adjacent_numbers_no_const_vec",
        |b| b.iter(greatest_product_of_four_adjacent_numbers_no_const_vec),
    );
}
