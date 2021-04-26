use criterion::{black_box, criterion_group, Criterion};

criterion_group!(
    sieve_of_eratosthenes,
    bench_the_10001st_prime_number,
    bench_the_10001st_prime_number_mod3,
    bench_the_10001st_prime_number_6k_p1_m1,
    bench_the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix,
    bench_the_10001st_prime_number_mod3_syntax,
    bench_the_10001st_prime_number_sieve_of_eratosthenes_extend,
);

use project_euler::m7::*;

fn bench_the_10001st_prime_number(c: &mut Criterion) {
    c.bench_function("the_10001st_prime_number", |b| {
        b.iter(|| the_10001st_prime_number(black_box(10001)))
    });
}

fn bench_the_10001st_prime_number_mod3(c: &mut Criterion) {
    c.bench_function("the_10001st_prime_number_mod3", |b| {
        b.iter(|| the_10001st_prime_number_mod3(black_box(10001)))
    });
}

fn bench_the_10001st_prime_number_6k_p1_m1(c: &mut Criterion) {
    c.bench_function("the_10001st_prime_number_6k_p1_m1", |b| {
        b.iter(|| the_10001st_prime_number_6k_p1_m1(black_box(10001)))
    });
}

fn bench_the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix(c: &mut Criterion) {
    c.bench_function(
        "the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix",
        |b| {
            b.iter(|| {
                the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix(black_box(10001))
            })
        },
    );
}

fn bench_the_10001st_prime_number_mod3_syntax(c: &mut Criterion) {
    c.bench_function("the_10001st_prime_number_mod3_syntax", |b| {
        b.iter(|| the_10001st_prime_number_mod3_syntax(black_box(10001)))
    });
}
fn bench_the_10001st_prime_number_sieve_of_eratosthenes_extend(c: &mut Criterion) {
    c.bench_function("the_10001st_prime_number_sieve_of_eratosthenes_extend", |b| {
        b.iter(|| the_10001st_prime_number_sieve_of_eratosthenes_extend(black_box(10001)))
    });
}

