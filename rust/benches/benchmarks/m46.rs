use criterion::{criterion_group, Criterion};

criterion_group!(
    goldbach_other_conjecture,
    bench_what_is_the_smallest_goldbach_other_conjecture,
);

use project_euler::m46::*;

fn bench_what_is_the_smallest_goldbach_other_conjecture(c: &mut Criterion) {
    c.bench_function("what_is_the_smallest_goldbach_other_conjecture", |b| {
        b.iter(what_is_the_smallest_goldbach_other_conjecture)
    });
}
