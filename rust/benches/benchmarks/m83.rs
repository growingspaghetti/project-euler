use criterion::{criterion_group, Criterion};

criterion_group!(
    path_sum_four_ways,
    bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left,
    bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left_pqueue
);

use project_euler::m83::*;

fn bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left(c: &mut Criterion) {
    c.bench_function("the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left", |b| {
        b.iter(the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left)
    });
}

fn bench_the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left_pqueue(c: &mut Criterion) {
    c.bench_function("the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left_pqueue", |b| {
        b.iter(the_minimal_path_sum_from_the_top_left_to_the_up_bottom_right_left_pqueue)
    });
}

