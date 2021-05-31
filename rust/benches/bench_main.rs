use criterion::criterion_main;

mod benchmarks;

criterion_main!(
    benchmarks::m1::sum_of_all_the_multiples,
    benchmarks::m2::sum_of_even_fibonacci_sequence,
    benchmarks::m3::largest_prime_factor,
    benchmarks::m4::largest_palindrome_product,
    benchmarks::m5::least_common_multiple,
    benchmarks::m6::square_pyramidal_number,
    benchmarks::m7::sieve_of_eratosthenes,
    benchmarks::m8::adjacent_digits,
    benchmarks::m9::pythagorean_triples,
    benchmarks::m10::eratosthenes_modified,
    benchmarks::m11::largest_product_in_a_grid,
    benchmarks::m12::number_of_divisors,
    benchmarks::m13::splice_gigantic_number,
    benchmarks::m14::collatz_cache,
    benchmarks::m15::ne_lattice_paths,
    benchmarks::m16::two_power_thousand,
    benchmarks::m18::triangle_path_max,
    benchmarks::m19::how_many_sundays,
    benchmarks::m20::factorial_100,
    benchmarks::m21::sigma_function,
    benchmarks::m22::selection_sort,
    benchmarks::m23::abundant_nums_pair,
    benchmarks::m24::permutation_range,
    benchmarks::m25::fibonacci_general_expression,
    benchmarks::m26::reciprocal_cycle,
    benchmarks::m27::primes_for_consecutive_values_of_n,
    benchmarks::m28::sum_of_the_numbers_on_the_diagonals,
    benchmarks::m29::integer_combinations_of_pow,
    benchmarks::m30::sum_of_dth_powers,
    benchmarks::m31::change_making_problem,
    benchmarks::m32::pandigital_nine,
    benchmarks::m34::digit_factorials,
    benchmarks::m35::circular_primes,
    benchmarks::m36::double_based_palindromes,
    benchmarks::m37::truncatable_primes,
    benchmarks::m38::pandigital_9_digit_number_concatenated_products,
    benchmarks::m81::path_sum_two_ways,
    benchmarks::m82::path_sum_three_ways,
    benchmarks::m83::path_sum_four_ways,
);
