use criterion::{criterion_group, Criterion};

criterion_group!(
    right_triangle,
    bench_perimeter_of_a_right_angle_triangle_1000,
    bench_perimeter_of_a_right_angle_triangle_1000_2,
    bench_perimeter_of_a_right_angle_triangle_1000_4,
);

use project_euler::m39::*;

fn bench_perimeter_of_a_right_angle_triangle_1000(c: &mut Criterion) {
    c.bench_function("perimeter_of_a_right_angle_triangle_1000", |b| {
        b.iter(perimeter_of_a_right_angle_triangle_1000)
    });
}

fn bench_perimeter_of_a_right_angle_triangle_1000_2(c: &mut Criterion) {
    c.bench_function("perimeter_of_a_right_angle_triangle_1000_2", |b| {
        b.iter(perimeter_of_a_right_angle_triangle_1000_2)
    });
}


fn bench_perimeter_of_a_right_angle_triangle_1000_4(c: &mut Criterion) {
    c.bench_function("perimeter_of_a_right_angle_triangle_1000_4", |b| {
        b.iter(perimeter_of_a_right_angle_triangle_1000_4)
    });
}
