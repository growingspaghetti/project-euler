use criterion::{criterion_group, Criterion};

criterion_group!(
    spiral_primes,
    bench_spiral_primes_1,
    bench_spiral_primes_2,
    bench_spiral_primes_3,
);

use project_euler::m58::*;

fn bench_spiral_primes_1(c: &mut Criterion) {
    c.bench_function("spiral_primes_1", |b| {
        b.iter(spiral_primes_1)
    });
}


fn bench_spiral_primes_2(c: &mut Criterion) {
    c.bench_function("spiral_primes_2", |b| {
        b.iter(spiral_primes_2)
    });
}



fn bench_spiral_primes_3(c: &mut Criterion) {
    c.bench_function("spiral_primes_3", |b| {
        b.iter(spiral_primes_3)
    });
}


