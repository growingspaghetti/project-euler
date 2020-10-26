use criterion::{criterion_group, Criterion};

criterion_group!(
    ne_lattice_paths,
    bench_routes_are_there_through_a_20_20_grid_walk_through,
    bench_routes_are_there_through_a_20_20_grid_combination,
    bench_routes_are_there_through_a_20_20_grid_combination_small_numbers,
    bench_routes_are_there_through_a_20_20_grid_combination_small_numbers_series,
    bench_routes_are_there_through_a_20_20_grid_combination_perfect_square,
);

use project_euler::m15::*;

fn bench_routes_are_there_through_a_20_20_grid_walk_through(c: &mut Criterion) {
    c.bench_function("routes_are_there_through_a_20_20_grid_walk_through", |b| {
        b.iter(routes_are_there_through_a_20_20_grid_walk_through)
    });
}

fn bench_routes_are_there_through_a_20_20_grid_combination(c: &mut Criterion) {
    c.bench_function("routes_are_there_through_a_20_20_grid_combination", |b| {
        b.iter(routes_are_there_through_a_20_20_grid_combination)
    });
}

fn bench_routes_are_there_through_a_20_20_grid_combination_small_numbers(c: &mut Criterion) {
    c.bench_function(
        "routes_are_there_through_a_20_20_grid_combination_small_numbers",
        |b| b.iter(routes_are_there_through_a_20_20_grid_combination_small_numbers),
    );
}

fn bench_routes_are_there_through_a_20_20_grid_combination_small_numbers_series(c: &mut Criterion) {
    c.bench_function(
        "routes_are_there_through_a_20_20_grid_combination_small_numbers_series",
        |b| b.iter(routes_are_there_through_a_20_20_grid_combination_small_numbers_series),
    );
}

fn bench_routes_are_there_through_a_20_20_grid_combination_perfect_square(c: &mut Criterion) {
    c.bench_function(
        "routes_are_there_through_a_20_20_grid_combination_perfect_square",
        |b| b.iter(routes_are_there_through_a_20_20_grid_combination_perfect_square),
    );
}
