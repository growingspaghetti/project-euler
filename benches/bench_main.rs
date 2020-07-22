use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::m1::sum_of_all_the_multiples,
    benchmarks::m2::sum_of_even_fibonacci_sequence,
    benchmarks::m3::largest_prime_factor,
    benchmarks::m4::largest_palindrome_product,
    benchmarks::m5::least_common_multiple,
    benchmarks::m6::square_pyramidal_number,
);
