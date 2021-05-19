use criterion::{criterion_group, black_box, Criterion};

criterion_group!(
    reciprocal_cycle,
//    bench_the_longest_recurring_cycle_2,
    bench_the_longest_recurring_cycle,
    bench_the_longest_recurring_cycle_3,
    bench_the_longest_recurring_cycle_reptend,
    bench_the_longest_recurring_cycle_incl_7,
    bench_the_longest_recurring_cycle_reptend_gcd,
    bench_the_longest_recurring_cycle_prime,
    bench_the_longest_recurring_cycle_prime_square,
    bench_the_longest_recurring_cycle_prime_mod_pow,
);

use project_euler::m26::*;

// fn bench_the_longest_recurring_cycle_2(c: &mut Criterion) {
//     c.bench_function("the_longest_recurring_cycle_2", |b| {
//         b.iter(the_longest_recurring_cycle_2)
//     });
// }

fn bench_the_longest_recurring_cycle(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle", |b| {
        b.iter(the_longest_recurring_cycle)
    });
}

fn bench_the_longest_recurring_cycle_3(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle_3", |b| {
        b.iter(|| the_longest_recurring_cycle_3(black_box(1000)))
    });
}

fn bench_the_longest_recurring_cycle_reptend(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle_reptend", |b| {
        b.iter(|| the_longest_recurring_cycle_reptend(black_box(1000)))
    });
}

fn bench_the_longest_recurring_cycle_incl_7(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle_incl_7", |b| {
        b.iter(|| the_longest_recurring_cycle_incl_7(black_box(1000)))
    });
}

fn bench_the_longest_recurring_cycle_reptend_gcd(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle_reptend_gcd", |b| {
        b.iter(|| the_longest_recurring_cycle_reptend_gcd(black_box(1000)))
    });
}


fn bench_the_longest_recurring_cycle_prime(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle_prime", |b| {
        b.iter(|| the_longest_recurring_cycle_prime(black_box(1000)))
    });
}

fn bench_the_longest_recurring_cycle_prime_square(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle_prime_square", |b| {
        b.iter(|| the_longest_recurring_cycle_prime_square(black_box(1000)))
    });
}

fn bench_the_longest_recurring_cycle_prime_mod_pow(c: &mut Criterion) {
    c.bench_function("the_longest_recurring_cycle_prime_mod_pow", |b| {
        b.iter(|| the_longest_recurring_cycle_prime_mod_pow(black_box(1000)))
    });
}
