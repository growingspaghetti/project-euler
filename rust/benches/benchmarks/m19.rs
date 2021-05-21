use criterion::{criterion_group, Criterion};

criterion_group!(
    how_many_sundays,
    bench_how_many_sundays_fell_on_the_first_of_the_month_zeller,
    bench_how_many_sundays_fell_on_the_first_of_the_month_2,
);

use project_euler::m19::*;

fn bench_how_many_sundays_fell_on_the_first_of_the_month_zeller(c: &mut Criterion) {
    c.bench_function("how_many_sundays_fell_on_the_first_of_the_month_zeller", |b| {
        b.iter(how_many_sundays_fell_on_the_first_of_the_month_zeller)
    });
}


fn bench_how_many_sundays_fell_on_the_first_of_the_month_2(c: &mut Criterion) {
    c.bench_function("how_many_sundays_fell_on_the_first_of_the_month_2", |b| {
        b.iter(how_many_sundays_fell_on_the_first_of_the_month_2)
    });
}
