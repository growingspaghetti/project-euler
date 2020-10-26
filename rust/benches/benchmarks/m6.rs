use criterion::{black_box, criterion_group, Criterion};

criterion_group!(
    square_pyramidal_number,
    bench_difference_between_the_sum_of_the_squares,
    bench_difference_between_the_sum_of_the_squares_square_pyramidal_number,
);

use project_euler::m6::*;

fn bench_difference_between_the_sum_of_the_squares(c: &mut Criterion) {
    c.bench_function("difference_between_the_sum_of_the_squares", |b| {
        b.iter(|| difference_between_the_sum_of_the_squares(black_box(100)))
    });
}

fn bench_difference_between_the_sum_of_the_squares_square_pyramidal_number(c: &mut Criterion) {
    c.bench_function(
        "difference_between_the_sum_of_the_squares_square_pyramidal_number",
        |b| {
            b.iter(|| {
                difference_between_the_sum_of_the_squares_square_pyramidal_number(black_box(100))
            })
        },
    );
}
