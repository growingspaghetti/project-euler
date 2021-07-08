use criterion::{criterion_group, Criterion};

criterion_group!(
    lychrel_numbers,
    bench_how_many_lychrel_numbers_are_there_below_ten_thousand,
    bench_how_many_lychrel_numbers_are_there_below_ten_thousand_linked_list,
);

use project_euler::m55::*;

fn bench_how_many_lychrel_numbers_are_there_below_ten_thousand(c: &mut Criterion) {
    c.bench_function("how_many_lychrel_numbers_are_there_below_ten_thousand", |b| {
        b.iter(how_many_lychrel_numbers_are_there_below_ten_thousand)
    });
}

fn bench_how_many_lychrel_numbers_are_there_below_ten_thousand_linked_list(c: &mut Criterion) {
    c.bench_function("how_many_lychrel_numbers_are_there_below_ten_thousand_linked_list", |b| {
        b.iter(how_many_lychrel_numbers_are_there_below_ten_thousand_linked_list)
    });
}
