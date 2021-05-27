use criterion::{criterion_group, Criterion};

criterion_group!(
    path_sum_two_ways,
    bench_the_minimal_path_sum_from_the_top_left_to_the_bottom_right_2,
);

use project_euler::m81::*;

fn bench_the_minimal_path_sum_from_the_top_left_to_the_bottom_right_2(c: &mut Criterion) {
    c.bench_function("the_minimal_path_sum_from_the_top_left_to_the_bottom_right_2", |b| {
        b.iter(the_minimal_path_sum_from_the_top_left_to_the_bottom_right_2)
    });
}
