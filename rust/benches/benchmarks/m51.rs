use criterion::{criterion_group, Criterion};

criterion_group!(
    prime_digit_replacements,
    bench_smallest_prime_which_by_replacing_part,
);

use project_euler::m51::*;

fn bench_smallest_prime_which_by_replacing_part(c: &mut Criterion) {
    c.bench_function("smallest_prime_which_by_replacing_part", |b| {
        b.iter(|| smallest_prime_which_by_replacing_part(3, 9))
    });
}
