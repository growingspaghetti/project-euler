use criterion::{criterion_group, Criterion};

criterion_group!(
    conservative_mod_pow,
    bench_self_power_1000,
    bench_self_power_1000_128,
);

use project_euler::m48::*;

fn bench_self_power_1000(c: &mut Criterion) {
    c.bench_function("self_power_1000", |b| b.iter(self_power_1000));
}

fn bench_self_power_1000_128(c: &mut Criterion) {
    c.bench_function("self_power_1000_128", |b| b.iter(self_power_1000_128));
}
