//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- the_first_triangle_number_to_have_over_five_ --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//! warning: function is never used: `bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute`
//!   --> benches/benchmarks/m12.rs:17:4
//!    |
//! 17 | fn bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute(c: &mut Criterion) {
//!    |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!    |
//!    = note: `#[warn(dead_code)]` on by default
//!
//! warning: 1 warning emitted
//!
//!     Finished bench [optimized] target(s) in 10.16s
//!      Running target/release/deps/bench_main-e043d12d7fc498f6
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 51.1s, or reduce sample count to 10.
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt: Collecting 100 samples in estimated 51.098 s (100 iterations)
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt: Analyzing
//! the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt
//!                         time:   [494.89 ms 495.54 ms 496.25 ms]
//!                         change: [-0.1959% +0.0000% +0.1915%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 7 outliers among 100 measurements (7.00%)
//!   4 (4.00%) high mild
//!   3 (3.00%) high severe
//! mean   [494.89 ms 496.25 ms] std. dev.      [2.6261 ms 4.3068 ms]
//! median [494.05 ms 495.56 ms] med. abs. dev. [1.8988 ms 3.2427 ms]
//!
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 49.9s, or reduce sample count to 10.
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series: Collecting 100 samples in estimated 49.857 s (100 iterations)
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series: Analyzing
//! the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series
//!                         time:   [499.30 ms 500.62 ms 502.03 ms]
//!                         change: [-0.4014% +0.0000% +0.3987%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   4 (4.00%) high mild
//! mean   [499.30 ms 502.03 ms] std. dev.      [5.5989 ms 8.3139 ms]
//! median [496.94 ms 499.06 ms] med. abs. dev. [2.7832 ms 5.6601 ms]
//!
//!
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1: Warming up for 3.0000 s
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1: Collecting 100 samples in estimated 5.1798 s (400 iterations)
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1: Analyzing
//! the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1
//!                         time:   [12.616 ms 12.663 ms 12.713 ms]
//!                         change: [-0.5280% +0.0000% +0.5118%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//!   5 (5.00%) high mild
//! mean   [12.616 ms 12.713 ms] std. dev.      [196.15 us 294.28 us]
//! median [12.550 ms 12.639 ms] med. abs. dev. [131.65 us 240.55 us]
//!
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors: Warming up for 3.0000 s
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors: Collecting 100 samples in estimated 7.3389 s (200 iterations)
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors: Analyzing
//! the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors
//!                         time:   [36.700 ms 36.857 ms 37.033 ms]
//!                         change: [-0.6385% +0.0000% +0.6223%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 10 outliers among 100 measurements (10.00%)
//!   4 (4.00%) high mild
//!   6 (6.00%) high severe
//! mean   [36.700 ms 37.033 ms] std. dev.      [601.07 us 1.0772 ms]
//! median [36.499 ms 36.721 ms] med. abs. dev. [330.75 us 567.19 us]
//!
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1: Warming up for 3.0000 s
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1: Collecting 100 samples in estimated 8.5486 s (200 iterations)
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1: Analyzing
//! the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1
//!                         time:   [42.582 ms 42.694 ms 42.819 ms]
//!                         change: [-0.3846% +0.0000% +0.3810%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   3 (3.00%) high mild
//!   1 (1.00%) high severe
//! mean   [42.582 ms 42.819 ms] std. dev.      [414.23 us 807.16 us]
//! median [42.452 ms 42.606 ms] med. abs. dev. [271.19 us 530.76 us]
//!
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_...
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_...: Warming up for 3.0000 s
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_...: Collecting 100 samples in estimated 5.0858 s (2100 iterations)
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_...: Analyzing
//! the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_...
//!                         time:   [2.3708 ms 2.3902 ms 2.4102 ms]
//!                         change: [-1.1922% +0.0000% +1.1116%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! mean   [2.3708 ms 2.4102 ms] std. dev.      [87.994 us 114.78 us]
//! median [2.3431 ms 2.4187 ms] med. abs. dev. [93.268 us 145.90 us]
//!
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_s...
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_s...: Warming up for 3.0000 s
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_s...: Collecting 100 samples in estimated 5.2480 s (1600 iterations)
//! Benchmarking the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_s...: Analyzing
//! the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_s...
//!                         time:   [3.2782 ms 3.3047 ms 3.3332 ms]
//!                         change: [-1.1738% +0.0000% +1.1103%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 9 outliers among 100 measurements (9.00%)
//!   4 (4.00%) high mild
//!   5 (5.00%) high severe
//! mean   [3.2782 ms 3.3332 ms] std. dev.      [110.68 us 169.45 us]
//! median [3.2479 ms 3.2938 ms] med. abs. dev. [56.002 us 119.28 us]
//! ```

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_brute;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_brute(), 4613732);
///```
/// Note: this function takes hours to finish.
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_brute() -> u64 {
    let count_divisors = |num: u64| -> u32 {
        let mut counter = 0u32;
        for d in 1..=num {
            if num % d == 0 {
                counter += 1;
            }
        }
        counter
    };
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    let mut num = 1u64;
    let mut reg = 0u64;
    let mut sum: u64;
    loop {
        sum = reg + num;
        if count_divisors(sum) > 500 {
            break sum;
        }
        reg = sum;
        num += 1;
    }
}

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_brute_divisor_counting_sqrt(
) -> u64 {
    let count_divisors = |num: u64| -> u32 {
        let mut counter = 0u32;
        // regarding num is an area of a rectancle, it's a side * another side.
        // given it's a squre, sides are a=sqrt(num) b=sqrt(num) and area=a*b.
        let side_max = (num as f64).sqrt().floor() as u64;
        for d in 1..side_max {
            if num % d == 0 {
                // eg.
                //  when area=528 and side A is 2, side B is determined to be 264.
                //  because we will skip to check side A is 264 as it's determined when 2 is checked,
                //  add the divisor counter 1 for 2 and 1 for 264.
                counter += 2;
            }
        }
        if num % side_max == 0 {
            // eg. A:4 * B:4 = S:16 but only one divisor to add.
            counter += 1;
        }
        counter
    };
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    let mut num = 1u64;
    let mut reg = 0u64;
    let mut sum: u64;
    loop {
        sum = reg + num;
        if count_divisors(sum) > 500 {
            break sum;
        }
        reg = sum;
        num += 1;
    }
}

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series() -> u64
{
    let count_divisors = |num: u64| -> u32 {
        let mut counter = 0u32;
        // regarding num is an area of a rectancle, it's a side * another side.
        // given it's a squre, sides are a=sqrt(num) b=sqrt(num) and area=a*b.
        let side_max = (num as f64).sqrt().floor() as u64;
        for d in 1..side_max {
            if num % d == 0 {
                // eg.
                //  when area=528 and side A is 2, side B is determined to be 264.
                //  because we will skip to check side A is 264 as it's determined when 2 is checked,
                //  add the divisor counter 1 for 2 and 1 for 264.
                counter += 2;
            }
        }
        if num % side_max == 0 {
            // eg. A:4 * B:4 = S:16 but only one divisor to add.
            counter += 1;
        }
        counter
    };
    // triangle number is actually the easiest arithmetic series.
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    // arithmetic_series. See m1.rs
    //  [0 1 2 3 ... 333#last] S = (0+333 #doubling) * 334#n / 2#halving
    //  [0 1]       S = 0+1 * 2 / 2 = 1
    //  [0 1 2]     S = 0+2 * 3 / 2 = 3
    //  [0 1 2 3]   S = 0+3 * 4 / 2 = 6
    //  [0 1 2 3 4] S = 0+4 * 5 / 2 = 10
    let mut num = 1u64;
    loop {
        let sum = num * (num + 1) / 2;
        if count_divisors(sum) > 500 {
            break sum;
        }
        num += 1;
    }
}

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_brute_arithmetic_series_n_n1(
) -> u64 {
    let count_divisors = |num: u64| -> u32 {
        let mut counter = 0u32;
        // regarding num is an area of a rectancle, it's a side * another side.
        // given it's a squre, sides are a=sqrt(num) b=sqrt(num) and area=a*b.
        let side_max = (num as f64).sqrt().floor() as u64;
        for d in 1..side_max {
            if num % d == 0 {
                // eg.
                //  when area=528 and side A is 2, side B is determined to be 264.
                //  because we will skip to check side A is 264 as it's determined when 2 is checked,
                //  add the divisor counter 1 for 2 and 1 for 264.
                counter += 2;
            }
        }
        if num % side_max == 0 {
            // eg. A:4 * B:4 = S:16 but only one divisor to add.
            counter += 1;
        }
        counter
    };
    // triangle number is actually the easiest arithmetic series.
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    // arithmetic_series. See m1.rs
    //  [0 1 2 3 ... 333#last] S = (0+333 #doubling) * 334#n / 2#halving
    //  [0 1]       S = 0+1 * 2 / 2 = 1
    //  [0 1 2]     S = 0+2 * 3 / 2 = 3
    //  [0 1 2 3]   S = 0+3 * 4 / 2 = 6
    //  [0 1 2 3 4] S = 0+4 * 5 / 2 = 10
    let mut num = 1u64;
    loop {
        let total_divisors = if num % 2 == 0 {
            count_divisors(num / 2) * count_divisors(num + 1)
        } else {
            count_divisors(num) * count_divisors((num + 1) / 2)
        };
        if total_divisors > 500 {
            break num * (num + 1) / 2;
        }
        num += 1;
    }
}

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors(
) -> u64 {
    fn build_prime_array_of_1000() -> [u32; 1000] {
        let mut matrix = [true; 10000];
        let mut counter = 1u32; // skipped 2 (no.1)
        let mut index = 3usize;
        matrix[0] = false;
        loop {
            if matrix[index] {
                counter += 1;
                if counter == 1000 {
                    break;
                } else {
                    for i in (index * index..matrix.len()).step_by(index) {
                        matrix[i] = false;
                    }
                }
            }
            index += 2;
        }
        let mut prime_factors = [0u32; 1000];
        prime_factors[0] = 2;
        let mut prime_factor_nth = 1usize;
        for i in (3..=index).step_by(2) {
            if matrix[i] {
                prime_factors[prime_factor_nth] = i as u32;
                prime_factor_nth += 1;
            }
        }
        prime_factors
    }

    // https://cp-algorithms.com/algebra/divisors.html#toc-tgt-0
    // https://en.wikipedia.org/wiki/Divisor_function#Table_of_values
    // num=1, divisors=[1], divisors.len=1 .. 1^0->1 or 0+1=1
    // num=3, divisors=[1,3], divisors.len=2 .. 3^1->(1+1)=2
    // num=6, divisors=[1,2,3,6], divisors.len=3 .. 2^1,3^1->(1+1)*(1+1)=4
    // num=28, divisors=[1,2,4,7,14,28], divisors.len=6 .. 2^2,7^1->(1+2)*(1+1)=6
    // note: if given num is a prime bigger than the 1000th prime 7919, it can give a wrong count 1 instead of 2 [1, 7927].
    //       also 1001th prime is 7927 and 7927*7927=62837329 and so 62837329*2=125674658 has divisors [1 2 7927 62837329] but returns 2 [1 2]
    let prime_array = build_prime_array_of_1000();
    let count_divisors = |mut num: u64| -> u32 {
        if num == 1 {
            return 1;
        }
        let mut counter = 1u32;
        for &prime in prime_array.iter() {
            if num == 1 {
                break;
            }
            let mut exp = 0u32;
            while num % prime as u64 == 0 {
                num /= prime as u64;
                exp += 1;
            }
            counter *= 1 + exp;
        }
        counter
    };
    // triangle number is actually the easiest arithmetic series.
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    // arithmetic_series. See m1.rs
    //  [0 1 2 3 ... 333#last] S = (0+333 #doubling) * 334#n / 2#halving
    //  [0 1]       S = 0+1 * 2 / 2 = 1
    //  [0 1 2]     S = 0+2 * 3 / 2 = 3
    //  [0 1 2 3]   S = 0+3 * 4 / 2 = 6
    //  [0 1 2 3 4] S = 0+4 * 5 / 2 = 10
    let mut num = 1u64;
    loop {
        let sum = num * (num + 1) / 2;
        if count_divisors(sum) > 500 {
            break sum;
        }
        num += 1;
    }
}

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1(
) -> u64 {
    fn build_prime_array_of_1000() -> [u32; 1000] {
        let mut matrix = [true; 10000];
        let mut counter = 1u32; // skipped 2 (no.1)
        let mut index = 3usize;
        matrix[0] = false;
        loop {
            if matrix[index] {
                counter += 1;
                if counter == 1000 {
                    break;
                } else {
                    for i in (index * index..matrix.len()).step_by(index) {
                        matrix[i] = false;
                    }
                }
            }
            index += 2;
        }
        let mut prime_factors = [0u32; 1000];
        prime_factors[0] = 2;
        let mut prime_factor_nth = 1usize;
        for i in (3..=index).step_by(2) {
            if matrix[i] {
                prime_factors[prime_factor_nth] = i as u32;
                prime_factor_nth += 1;
            }
        }
        prime_factors
    }

    // https://cp-algorithms.com/algebra/divisors.html#toc-tgt-0
    // https://en.wikipedia.org/wiki/Divisor_function#Table_of_values
    // num=1, divisors=[1], divisors.len=1 .. 1^0->1 or 0+1=1
    // num=3, divisors=[1,3], divisors.len=2 .. 3^1->(1+1)=2
    // num=6, divisors=[1,2,3,6], divisors.len=3 .. 2^1,3^1->(1+1)*(1+1)=4
    // num=28, divisors=[1,2,4,7,14,28], divisors.len=6 .. 2^2,7^1->(1+2)*(1+1)=6
    // note: if given num is a prime bigger than the 1000th prime 7919, it can give a wrong count 1 instead of 2 [1, 7927].
    //       also 1001th prime is 7927 and 7927*7927=62837329 and so 62837329*2=125674658 has divisors [1 2 7927 62837329] but returns 2 [1 2]
    let prime_array = build_prime_array_of_1000();
    let count_divisors = |mut num: u64| -> u32 {
        if num == 1 {
            return 1;
        }
        let mut counter = 1u32;
        for &prime in prime_array.iter() {
            if num == 1 {
                break;
            }
            let mut exp = 0u32;
            while num % prime as u64 == 0 {
                num /= prime as u64;
                exp += 1;
            }
            counter *= 1 + exp;
        }
        counter
    };
    // triangle number is actually the easiest arithmetic series.
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    // arithmetic_series. See m1.rs
    //  [0 1 2 3 ... 333#last] S = (0+333 #doubling) * 334#n / 2#halving
    //  [0 1]       S = 0+1 * 2 / 2 = 1
    //  [0 1 2]     S = 0+2 * 3 / 2 = 3
    //  [0 1 2 3]   S = 0+3 * 4 / 2 = 6
    //  [0 1 2 3 4] S = 0+4 * 5 / 2 = 10
    let mut num = 1u64;
    loop {
        let total_divisors = if num % 2 == 0 {
            count_divisors(num / 2) * count_divisors(num + 1)
        } else {
            count_divisors(num) * count_divisors((num + 1) / 2)
        };
        if total_divisors > 500 {
            break num * (num + 1) / 2;
        }
        num += 1;
    }
}

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_matrix;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_matrix(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_small_matrix(
) -> u64 {
    // the answer 76576500 is 2^2 x 3^2 x 5^3 x 7^1 x 11^1 x 13^1 x 17^1 (17 is merely 7th prime)
    fn build_prime_array_of_10() -> [u32; 10] {
        let mut matrix = [true; 50];
        let mut counter = 1u32; // skipped 2 (no.1)
        let mut index = 3usize;
        matrix[0] = false;
        loop {
            if matrix[index] {
                counter += 1;
                if counter == 10 {
                    break;
                } else {
                    for i in (index * index..matrix.len()).step_by(index) {
                        matrix[i] = false;
                    }
                }
            }
            index += 2;
        }
        let mut prime_factors = [0u32; 10];
        prime_factors[0] = 2;
        let mut prime_factor_nth = 1usize;
        for i in (3..=index).step_by(2) {
            if matrix[i] {
                prime_factors[prime_factor_nth] = i as u32;
                prime_factor_nth += 1;
            }
        }
        prime_factors
    }

    // https://cp-algorithms.com/algebra/divisors.html#toc-tgt-0
    // https://en.wikipedia.org/wiki/Divisor_function#Table_of_values
    // num=1, divisors=[1], divisors.len=1 .. 1^0->1 or 0+1=1
    // num=3, divisors=[1,3], divisors.len=2 .. 3^1->(1+1)=2
    // num=6, divisors=[1,2,3,6], divisors.len=3 .. 2^1,3^1->(1+1)*(1+1)=4
    // num=28, divisors=[1,2,4,7,14,28], divisors.len=6 .. 2^2,7^1->(1+2)*(1+1)=6
    // note: if given num is a prime bigger than the 10th prime 29, it can give a wrong count 1 instead of 2 [1, 31].
    //       also 11th prime is 31 and 31*31=961 and so <omitted see above>
    let prime_array = build_prime_array_of_10();
    let count_divisors = |mut num: u64| -> u32 {
        if num == 1 {
            return 1;
        }
        let mut counter = 1u32;
        for &prime in prime_array.iter() {
            if num == 1 {
                break;
            }
            let mut exp = 0u32;
            while num % prime as u64 == 0 {
                num /= prime as u64;
                exp += 1;
            }
            counter *= 1 + exp;
        }
        counter
    };
    // triangle number is actually the easiest arithmetic series.
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    // arithmetic_series. See m1.rs
    //  [0 1 2 3 ... 333#last] S = (0+333 #doubling) * 334#n / 2#halving
    //  [0 1]       S = 0+1 * 2 / 2 = 1
    //  [0 1 2]     S = 0+2 * 3 / 2 = 3
    //  [0 1 2 3]   S = 0+3 * 4 / 2 = 6
    //  [0 1 2 3 4] S = 0+4 * 5 / 2 = 10
    let mut num = 1u64;
    loop {
        let sum = num * (num + 1) / 2;
        if count_divisors(sum) > 500 {
            break sum;
        }
        num += 1;
    }
}

/// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// Let us list the factors of the first seven triangle numbers
///
///  1 [ 1 ]
///  3 [ 1,3 ]
///  6 [ 1,2,3,6 ]
/// 10 [ 1,2,5,10 ]
/// 15 [ 1,3,5,15 ]
/// 21 [ 1,3,7,21 ]
/// 28 [ 1,2,4,7,14,28 ]
///
/// We can see that 28 is the first triangle number to have over five divisors.
///
/// What is the value of the first triangle number to have over five hundred divisors?
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix(
) -> u64 {
    // the answer 76576500 is 12375 × 12376 / 2.
    // primes of     12375 are 3^2 x 5^3 x 11^1
    //        12376/2=6188 are 2^2 x 7^1 x 13^1 x 17^1
    fn build_prime_array_of_10() -> [u32; 10] {
        let mut matrix = [true; 50];
        let mut counter = 1u32; // skipped 2 (no.1)
        let mut index = 3usize;
        matrix[0] = false;
        loop {
            if matrix[index] {
                counter += 1;
                if counter == 10 {
                    break;
                } else {
                    for i in (index * index..matrix.len()).step_by(index) {
                        matrix[i] = false;
                    }
                }
            }
            index += 2;
        }
        let mut prime_factors = [0u32; 10];
        prime_factors[0] = 2;
        let mut prime_factor_nth = 1usize;
        for i in (3..=index).step_by(2) {
            if matrix[i] {
                prime_factors[prime_factor_nth] = i as u32;
                prime_factor_nth += 1;
            }
        }
        prime_factors
    }

    // https://cp-algorithms.com/algebra/divisors.html#toc-tgt-0
    // https://en.wikipedia.org/wiki/Divisor_function#Table_of_values
    // num=1, divisors=[1], divisors.len=1 .. 1^0->1 or 0+1=1
    // num=3, divisors=[1,3], divisors.len=2 .. 3^1->(1+1)=2
    // num=6, divisors=[1,2,3,6], divisors.len=3 .. 2^1,3^1->(1+1)*(1+1)=4
    // num=28, divisors=[1,2,4,7,14,28], divisors.len=6 .. 2^2,7^1->(1+2)*(1+1)=6
    // note: if given num is a prime bigger than the 10th prime 29, it can give a wrong count 1 instead of 2 [1, 31].
    //       also 11th prime is 31 and 31*31=961 and so <omitted see above>
    let prime_array = build_prime_array_of_10();
    let count_divisors = |mut num: u64| -> u32 {
        if num == 1 {
            return 1;
        }
        let mut counter = 1u32;
        for &prime in prime_array.iter() {
            if num == 1 {
                break;
            }
            let mut exp = 0u32;
            while num % prime as u64 == 0 {
                num /= prime as u64;
                exp += 1;
            }
            counter *= 1 + exp;
        }
        counter
    };
    // triangle number is actually the easiest arithmetic series.
    // num: 1 2     3     4      5
    // reg: 0 1     3     6      10
    // sum: 1 1+2=3 3+3=6 6+4=10 10+5=15
    // arithmetic_series. See m1.rs
    //  [0 1 2 3 ... 333#last] S = (0+333 #doubling) * 334#n / 2#halving
    //  [0 1]       S = 0+1 * 2 / 2 = 1
    //  [0 1 2]     S = 0+2 * 3 / 2 = 3
    //  [0 1 2 3]   S = 0+3 * 4 / 2 = 6
    //  [0 1 2 3 4] S = 0+4 * 5 / 2 = 10
    let mut num = 1u64;
    loop {
        let total_divisors = if num % 2 == 0 {
            count_divisors(num / 2) * count_divisors(num + 1)
        } else {
            count_divisors(num) * count_divisors((num + 1) / 2)
        };
        if total_divisors > 500 {
            break num * (num + 1) / 2;
        }
        num += 1;
    }
}

#[test]
fn main() {
    assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors(), 76576500);
}

struct TriangleNumber {
    _nth: u64,
    _primes: Vec<u64>,
    _num_div_even: u64,
    _num_div_odd: u64,
}

trait Divisors {
    fn number_of_divisors(&self) -> u64;
}

impl TriangleNumber {
    fn new() -> Self {
        TriangleNumber {
            _nth: 3,
            _num_div_even: 2,
            _num_div_odd: 2,
            _primes: vec![2, 3],
        }
    }
    fn num(&self) -> u64 {
        self._nth * (self._nth + 1) / 2
    }
    fn _divide_fully(&self, n: &mut u64, d: u64, side: &mut u64, count: &mut u64) {
        if *n % d == 0 {
            let mut exp = 0u64;
            while {
                *n /= d;
                exp += 1;
                *n % d == 0
            } {}
            *side = (*n as f64).sqrt() as u64;
            *count *= exp + 1;
        }
    }
    fn _num_of_divisors(&mut self, mut n: u64) -> u64 {
        let mut count = 1u64;
        let mut side = (n as f64).sqrt() as u64;
        for &p in self._primes.iter() {
            if p > side || n == 1 {
                break;
            }
            self._divide_fully(&mut n, p, &mut side, &mut count);
        }
        if n != 1 {
            count *= 2;
            self._primes.push(n);
        }
        count
    }
    fn increment(&mut self) {
        self._nth += 1;
        if self._nth % 2 == 0 {
            self._num_div_odd = self._num_of_divisors(self._nth + 1);
        } else {
            self._num_div_even = self._num_of_divisors((self._nth + 1) / 2);
        }
    }
}

impl Divisors for TriangleNumber {
    fn number_of_divisors(&self) -> u64 {
        self._num_div_even * self._num_div_odd
    }
}

fn number_of_divisors(mut n: u64, primes: &mut Vec<u64>) -> u64 {
    let mut count = 1u64;
    for &p in primes.iter() {
        if n == 1 {
            break;
        }
        let mut exp = 0u64;
        while n % p == 0 {
            n /= p;
            exp += 1;
        }
        count *= exp + 1;
    }
    if n != 1 {
        count *= 2;
        primes.push(n);
    }
    count
}

// 2.26 ms
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix_struct;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix_struct(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix_struct(
) -> u64 {
    let mut triangle_number = TriangleNumber::new();
    while triangle_number.number_of_divisors() <= 500 {
        triangle_number.increment();
    }
    triangle_number.num()
}

// 23 ms
///
///```rust
/// use self::project_euler::m12::the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix_2;
/// assert_eq!(the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix_2(), 76576500);
///```
pub fn the_first_triangle_number_to_have_over_five_hundred_divisors_prime_factors_number_of_divisors_n_n1_small_matrix_2(
) -> u64 {
    let mut primes = vec![2u64, 3, 5];
    let mut num = 7u64;
    loop {
        let total_divisors = if num % 2 == 0 {
            number_of_divisors(num / 2, &mut primes) * number_of_divisors(num + 1, &mut primes)
        } else {
            number_of_divisors(num, &mut primes) * number_of_divisors((num + 1) / 2, &mut primes)
        };
        if total_divisors > 500 {
            break num * (num + 1) / 2;
        }
        num += 1;
    }
}
