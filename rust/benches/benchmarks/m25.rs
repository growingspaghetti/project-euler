use criterion::{criterion_group, Criterion};

criterion_group!(
    fibonacci_general_expression,
    bench_index_of_fibonacci_sequence_to_contain_1000_digits_brute,
    bench_index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio,
    bench_index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio_2,
);

use project_euler::m25::*;

fn bench_index_of_fibonacci_sequence_to_contain_1000_digits_brute(c: &mut Criterion) {
    c.bench_function(
        "index_of_fibonacci_sequence_to_contain_1000_digits_brute",
        |b| b.iter(index_of_fibonacci_sequence_to_contain_1000_digits_brute),
    );
}

fn bench_index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio(
    c: &mut Criterion,
) {
    c.bench_function(
        "index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio",
        |b| b.iter(index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio),
    );
}
fn bench_index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio_2(
    c: &mut Criterion,
) {
    c.bench_function(
        "index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio_2",
        |b| b.iter(index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio_2),
    );
}
