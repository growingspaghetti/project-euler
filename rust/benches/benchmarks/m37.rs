use criterion::{criterion_group, Criterion};

criterion_group!(
    truncatable_primes,
    bench_both_truncatable_primes,
    bench_both_truncatable_primes_sieve,
);

use project_euler::m37::*;

fn bench_both_truncatable_primes(c: &mut Criterion) {
    c.bench_function("both_truncatable_primes", |b| {
        b.iter(both_truncatable_primes)
    });
}

fn bench_both_truncatable_primes_sieve(c: &mut Criterion) {
    c.bench_function("both_truncatable_primes_sieve", |b| {
        b.iter(both_truncatable_primes_sieve)
    });
}

