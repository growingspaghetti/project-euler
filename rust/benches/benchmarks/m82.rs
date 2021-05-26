use criterion::{criterion_group, Criterion};

criterion_group!(
    path_sum_three_ways,
    bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right,
    bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_proper,
);

use project_euler::m82::*;

fn bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right(c: &mut Criterion) {
    c.bench_function("the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right", |b| {
        b.iter(the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right)
    });
}

fn bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_proper(c: &mut Criterion) {
    c.bench_function("the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_proper", |b| {
        b.iter(the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_proper)
    });
}
