use criterion::{criterion_group, Criterion};

criterion_group!(
    selection_sort,
    bench_the_total_of_all_the_name_scores_std,
    bench_the_total_of_all_the_name_scores_std_unstable_sort,
    bench_the_total_of_all_the_name_scores_bubble_sort,
    bench_the_total_of_all_the_name_scores_selection_sort,
    bench_the_total_of_all_the_name_scores_quick_sort,
);

use project_euler::m22::*;

fn bench_the_total_of_all_the_name_scores_std(c: &mut Criterion) {
    c.bench_function("the_total_of_all_the_name_scores_std", |b| {
        b.iter(the_total_of_all_the_name_scores_std)
    });
}

fn bench_the_total_of_all_the_name_scores_std_unstable_sort(c: &mut Criterion) {
    c.bench_function("the_total_of_all_the_name_scores_std_unstable_sort", |b| {
        b.iter(the_total_of_all_the_name_scores_std_unstable_sort)
    });
}

fn bench_the_total_of_all_the_name_scores_bubble_sort(c: &mut Criterion) {
    c.bench_function("the_total_of_all_the_name_scores_bubble_sort", |b| {
        b.iter(the_total_of_all_the_name_scores_bubble_sort)
    });
}

fn bench_the_total_of_all_the_name_scores_selection_sort(c: &mut Criterion) {
    c.bench_function("the_total_of_all_the_name_scores_selection_sort", |b| {
        b.iter(the_total_of_all_the_name_scores_selection_sort)
    });
}

fn bench_the_total_of_all_the_name_scores_quick_sort(c: &mut Criterion) {
    c.bench_function("the_total_of_all_the_name_scores_quick_sort", |b| {
        b.iter(the_total_of_all_the_name_scores_quick_sort)
    });
}
