use criterion::{criterion_group, Criterion};

criterion_group!(
    number_of_divisors,
    //bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute,
    bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt,
    bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series,
    bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1,
    bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors,
    bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1,
    bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_matrix,
    bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix,
);

use project_euler::m12::*;

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute(c: &mut Criterion) {
    c.bench_function(
        "the_first_triangle_number_to_have_over_five_hundred_divisors_brute",
        |b| b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_brute),
    );
}

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt(
    c: &mut Criterion,
) {
    c.bench_function(
        "the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt",
        |b| b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt),
    );
}

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series(
    c: &mut Criterion,
) {
    c.bench_function(
        "the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series",
        |b| b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series),
    );
}

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1(
    c: &mut Criterion,
) {
    c.bench_function(
        "the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1",
        |b| b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1),
    );
}

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors(
    c: &mut Criterion,
) {
    c.bench_function(
        "the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors",
        |b| b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors),
    );
}

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1(
    c: &mut Criterion,
) {
    c.bench_function("the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1", |b| {
        b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1)
    });
}

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_matrix(
    c: &mut Criterion,
) {
    c.bench_function("the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_matrix", |b| {
        b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_matrix)
    });
}

fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix(
    c: &mut Criterion,
) {
    c.bench_function("the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix", |b| {
        b.iter(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix)
    });
}
