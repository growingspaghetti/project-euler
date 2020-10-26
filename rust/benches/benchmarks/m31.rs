use criterion::{criterion_group, Criterion};

criterion_group!(
    change_making_problem,
    bench_how_many_different_ways_can_2_be_made_brute,
    bench_how_many_different_ways_can_2_be_made,
    bench_how_many_different_ways_can_2_be_made_rec,
    bench_how_many_different_ways_can_2_be_made_rec_memo,
);

use project_euler::m31::*;

fn bench_how_many_different_ways_can_2_be_made_brute(c: &mut Criterion) {
    c.bench_function("how_many_different_ways_can_2_be_made_brute", |b| {
        b.iter(how_many_different_ways_can_2_be_made_brute)
    });
}

fn bench_how_many_different_ways_can_2_be_made(c: &mut Criterion) {
    c.bench_function("how_many_different_ways_can_2_be_made", |b| {
        b.iter(how_many_different_ways_can_2_be_made)
    });
}

fn bench_how_many_different_ways_can_2_be_made_rec(c: &mut Criterion) {
    c.bench_function("how_many_different_ways_can_2_be_made_rec", |b| {
        b.iter(how_many_different_ways_can_2_be_made_rec)
    });
}

fn bench_how_many_different_ways_can_2_be_made_rec_memo(c: &mut Criterion) {
    c.bench_function("how_many_different_ways_can_2_be_made_rec_memo", |b| {
        b.iter(how_many_different_ways_can_2_be_made_rec_memo)
    });
}
