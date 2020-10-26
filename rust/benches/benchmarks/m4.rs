use criterion::{criterion_group, Criterion};

criterion_group!(
    largest_palindrome_product,
    bench_largest_palindrome_product_of_two_3_digits,
    bench_largest_palindrome_product_of_two_3_digits_mod_10,
    bench_largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair,
    bench_largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut,
    bench_largest_palindrome_product_of_two_3_digits_factorization,
);

use project_euler::m4::*;

fn bench_largest_palindrome_product_of_two_3_digits(c: &mut Criterion) {
    c.bench_function("largest_palindrome_product_of_two_3_digits", |b| {
        b.iter(largest_palindrome_product_of_two_3_digits)
    });
}

fn bench_largest_palindrome_product_of_two_3_digits_mod_10(c: &mut Criterion) {
    c.bench_function("largest_palindrome_product_of_two_3_digits_mod_10", |b| {
        b.iter(largest_palindrome_product_of_two_3_digits_mod_10)
    });
}

fn bench_largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair(c: &mut Criterion) {
    c.bench_function(
        "largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair",
        |b| b.iter(largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair),
    );
}

fn bench_largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut(
    c: &mut Criterion,
) {
    c.bench_function(
        "largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut",
        |b| b.iter(largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut),
    );
}

fn bench_largest_palindrome_product_of_two_3_digits_factorization(c: &mut Criterion) {
    c.bench_function(
        "largest_palindrome_product_of_two_3_digits_factorization",
        |b| b.iter(largest_palindrome_product_of_two_3_digits_factorization),
    );
}
