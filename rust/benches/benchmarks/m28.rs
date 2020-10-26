use criterion::{criterion_group, Criterion};

criterion_group!(
    sum_of_the_numbers_on_the_diagonals,
    bench_sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec,
    bench_sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box,
    bench_sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square,
);

use project_euler::m28::*;

fn bench_sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec(c: &mut Criterion) {
    c.bench_function(
        "sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec",
        |b| b.iter(sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec),
    );
}

fn bench_sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box(c: &mut Criterion) {
    c.bench_function(
        "sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box",
        |b| b.iter(sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box),
    );
}

fn bench_sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square(
    c: &mut Criterion,
) {
    c.bench_function(
        "sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square",
        |b| b.iter(sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square),
    );
}
