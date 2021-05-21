use criterion::{criterion_group, Criterion};

criterion_group!(
    integer_combinations_of_pow,
    bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute,
    bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string,
    bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors,
    bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort,
    bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort_2,
);

use project_euler::m29::*;

fn bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute(c: &mut Criterion) {
    c.bench_function(
        "distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute",
        |b| b.iter(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute),
    );
}

fn bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string(
    c: &mut Criterion,
) {
    c.bench_function(
        "distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string",
        |b| b.iter(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string),
    );
}

fn bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors(c: &mut Criterion) {
    c.bench_function(
        "distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors",
        |b| b.iter(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors),
    );
}

fn bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort(
    c: &mut Criterion,
) {
    c.bench_function(
        "distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort",
        |b| b.iter(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort),
    );
}

fn bench_distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort_2(
    c: &mut Criterion,
) {
    c.bench_function(
        "distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort_2",
        |b| b.iter(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort_2),
    );
}
