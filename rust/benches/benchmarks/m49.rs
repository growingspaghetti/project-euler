use criterion::{criterion_group, Criterion};

criterion_group!(
    four_digit_circular_prime,
    bench_four_digit_circular_prime_3330_series,
);

use project_euler::m49::*;

fn bench_four_digit_circular_prime_3330_series(c: &mut Criterion) {
    c.bench_function("four_digit_circular_prime_3330_series", |b| {
        b.iter(four_digit_circular_prime_3330_series)
    });
}
