use criterion::{criterion_group, Criterion};

criterion_group!(
    triangle_path_max,
    bench_the_maximum_total_from_top_to_bottom_of_the_triangle,
    bench_the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up,
    bench_the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up_2,
);

use project_euler::m18::*;

fn bench_the_maximum_total_from_top_to_bottom_of_the_triangle(c: &mut Criterion) {
    c.bench_function(
        "the_maximum_total_from_top_to_bottom_of_the_triangle",
        |b| b.iter(the_maximum_total_from_top_to_bottom_of_the_triangle),
    );
}

fn bench_the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up(c: &mut Criterion) {
    c.bench_function(
        "the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up",
        |b| b.iter(the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up),
    );
}

fn bench_the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up_2(c: &mut Criterion) {
    c.bench_function(
        "the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up_2",
        |b| b.iter(the_maximum_total_from_top_to_bottom_of_the_triangle_bottom_up_2),
    );
}
