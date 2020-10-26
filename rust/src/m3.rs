//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- largest_prime_factor --verbose
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!     Finished bench [optimized] target(s) in 5.47s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking largest_prime_factor_of_the_number_600851475143
//! Benchmarking largest_prime_factor_of_the_number_600851475143: Warming up for 3.0000 s
//! Benchmarking largest_prime_factor_of_the_number_600851475143: Collecting 100 samples in estimated 5.3909 s (60600 iterations)
//! Benchmarking largest_prime_factor_of_the_number_600851475143: Analyzing
//! largest_prime_factor_of_the_number_600851475143
//!                         time:   [88.383 us 88.581 us 88.796 us]
//! Found 5 outliers among 100 measurements (5.00%)
//!   3 (3.00%) high mild
//!   2 (2.00%) high severe
//! slope  [88.383 us 88.796 us] R^2            [0.9871458 0.9870349]
//! mean   [89.013 us 89.879 us] std. dev.      [1.6218 us 2.8058 us]
//! median [88.242 us 89.115 us] med. abs. dev. [899.65 ns 1.8679 us]
//!
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12: Warming up for 3.0000 s
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12: Collecting 100 samples in estimated 5.1661 s (151500 iterations)
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12: Analyzing
//! largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12
//!                         time:   [33.970 us 34.048 us 34.132 us]
//! Found 5 outliers among 100 measurements (5.00%)
//!   5 (5.00%) high mild
//! slope  [33.970 us 34.132 us] R^2            [0.9841724 0.9840777]
//! mean   [34.191 us 34.444 us] std. dev.      [495.28 ns 780.40 ns]
//! median [33.987 us 34.313 us] med. abs. dev. [392.90 ns 682.13 ns]
//!
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab: Warming up for 3.0000 s
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab: Collecting 100 samples in estimated 5.0439 s (141400 iterations)
//! Benchmarking largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab: Analyzing
//! largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab
//!                         time:   [34.820 us 34.961 us 35.128 us]
//! Found 4 outliers among 100 measurements (4.00%)
//!   4 (4.00%) high mild
//! slope  [34.820 us 35.128 us] R^2            [0.8992922 0.8985196]
//! mean   [35.920 us 36.909 us] std. dev.      [1.8445 us 3.1597 us]
//! median [34.792 us 36.636 us] med. abs. dev. [682.53 ns 2.5030 us]
//! ```

/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143(), 6857);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143() -> u64 {
    let mut n = 600851475143u64;
    let mut divisor = 2u64;
    let mut max_factor = 0u64;
    // 16 -> 2 * 8 -> 2 * 4 -> 2 * 2 -> 2 * 1
    // 21 -> 3 * 7 -> 7 * 1
    // 28 -> 2 * 14 -> 2 * 7 -> 7 * 1
    while n != 0 && n != 1 {
        if n % divisor != 0 {
            divisor += 1;
        } else {
            max_factor = n;
            n /= divisor;
        }
    }
    max_factor
}

/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12(), 6857);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12() -> u64 {
    let mut n = 600851475143u64;
    let mut divisor = 3u64;
    let mut max_factor;

    if n % 2 == 0 {
        n /= 2;
        max_factor = 2;
        while n % 2 == 0 {
            n /= 2;
        }
    } else {
        max_factor = 1;
    }

    while n > 1 {
        if n % divisor == 0 {
            n /= divisor;
            max_factor = divisor;
            while n % divisor == 0 {
                n /= divisor
            }
        } else {
            divisor += 2
        }
    }
    max_factor
}

/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(), 6857);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab() -> u64 {
    let mut n = 600851475143u64;
    let mut divisor = 3u64;
    let mut max_factor;

    if n % 2 == 0 {
        n /= 2;
        max_factor = 2;
        while n % 2 == 0 {
            n /= 2;
        }
    } else {
        max_factor = 1;
    }

    // n = 1 * n || n = a * b
    //  in square,  n = sqrt(n) * sqrt(n)
    //  pattern 1: a = sqrt(n) && b = sqrt(n)
    //  pattern 2: a <= sqrt(n) || b <= sqrt(n)
    //  impossible: a > sqrt(n) && b >= sqrt(n)
    let a = (n as f64).sqrt() as u64;
    while n > 1 && divisor <= a {
        if n % divisor == 0 {
            n /= divisor;
            max_factor = divisor;
            while n % divisor == 0 {
                n /= divisor
            }
        } else {
            divisor += 2
        }
    }
    max_factor
}
