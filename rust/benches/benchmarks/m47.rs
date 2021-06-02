use criterion::{criterion_group, Criterion};

criterion_group!(
    four_consective_numbers_with_four_primes,
    bench_distinct_prime_factors_four_consecutive_integers,
);

use project_euler::m47::*;

fn bench_distinct_prime_factors_four_consecutive_integers(c: &mut Criterion) {
    c.bench_function("distinct_prime_factors_four_consecutive_integers", |b| {
        b.iter(distinct_prime_factors_four_consecutive_integers)
    });
}

