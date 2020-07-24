use criterion::{criterion_group, Criterion};

criterion_group!(
    collatz_cache,
    bench_collatz_under_one_million_produces_the_longest_chain,
    bench_collatz_under_one_million_produces_the_longest_chain_cache,
    bench_collatz_under_one_million_produces_the_longest_chain_cache_rec,
    bench_collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2,
    bench_collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000,
    bench_collatz_under_one_million_produces_the_longest_chain_full,
);

use project_euler::m14::*;

fn bench_collatz_under_one_million_produces_the_longest_chain(c: &mut Criterion) {
    c.bench_function(
        "collatz_under_one_million_produces_the_longest_chain",
        |b| b.iter(collatz_under_one_million_produces_the_longest_chain),
    );
}

fn bench_collatz_under_one_million_produces_the_longest_chain_cache(c: &mut Criterion) {
    c.bench_function(
        "collatz_under_one_million_produces_the_longest_chain_cache",
        |b| b.iter(collatz_under_one_million_produces_the_longest_chain_cache),
    );
}

fn bench_collatz_under_one_million_produces_the_longest_chain_cache_rec(c: &mut Criterion) {
    c.bench_function(
        "collatz_under_one_million_produces_the_longest_chain_cache_rec",
        |b| b.iter(collatz_under_one_million_produces_the_longest_chain_cache_rec),
    );
}

fn bench_collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2(c: &mut Criterion) {
    c.bench_function(
        "collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2",
        |b| b.iter(collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2),
    );
}

fn bench_collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000(
    c: &mut Criterion,
) {
    c.bench_function(
        "collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000",
        |b| {
            b.iter(
                collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000,
            )
        },
    );
}

fn bench_collatz_under_one_million_produces_the_longest_chain_full(c: &mut Criterion) {
    c.bench_function(
        "collatz_under_one_million_produces_the_longest_chain_full",
        |b| b.iter(collatz_under_one_million_produces_the_longest_chain_full),
    );
}
