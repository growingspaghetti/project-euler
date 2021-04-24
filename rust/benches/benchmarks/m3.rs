use criterion::{criterion_group, Criterion};

criterion_group!(
    largest_prime_factor,
    bench_largest_prime_factor_of_the_number_600851475143,
    bench_largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60,
    bench_largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12,
    bench_largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes,
    bench_largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab,
);

use project_euler::m3::*;

fn bench_largest_prime_factor_of_the_number_600851475143(c: &mut Criterion) {
    c.bench_function("largest_prime_factor_of_the_number_600851475143", |b| {
        b.iter(largest_prime_factor_of_the_number_600851475143)
    });
}
fn bench_largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60(c: &mut Criterion) {
    c.bench_function(
        "largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60",
        |b| b.iter(|| largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60(600851475143)),
    );
}

fn bench_largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12(c: &mut Criterion) {
    c.bench_function(
        "largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12",
        |b| b.iter(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12),
    );
}

fn bench_largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes(
    c: &mut Criterion,
) {
    c.bench_function(
        "largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes",
        |b| {
            b.iter(|| {
                largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes(
                    600851475143,
                )
            })
        },
    );
}

fn bench_largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(c: &mut Criterion) {
    c.bench_function(
        "largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab",
        |b| {
            b.iter(|| {
                largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(
                    600851475143u64,
                )
            })
        },
    );
}
