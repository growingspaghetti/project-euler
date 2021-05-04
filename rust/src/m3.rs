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

// largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60
//                         time:   [73.492 us 73.776 us 74.051 us]
//                         change: [-0.4297% +0.0000% +0.4565%] (p = 1.00 > 0.05)
//                         No change in performance detected.

///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60(600851475143u64), 6857);
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60(60), 5);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143_2_2_3_5_1_60(mut n: u64) -> u64 {
    assert!(n > 1);
    let mut divisor = 2u64;
    while n != 1 {
        if n % divisor == 0 {
            n /= divisor;
        } else {
            divisor += 1;
        }
    }
    divisor
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
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(600851475143u64), 6857);
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(60), 5);
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(5), 5);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(mut n: u64) -> u64 {
    //let mut n = 600851475143u64;
    let mut divisor = 3u64;
    let mut max_factor;

    let side = (n as f64).sqrt() as u64;
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
    while n > 1 && divisor <= side {
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
    if n == 1 {
        max_factor
    } else {
        n
    }
}

// largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes
//                         time:   [13.864 us 13.965 us 14.078 us]
//                         change: [-0.9576% +0.0000% +1.0199%] (p = 1.00 > 0.05)
//                         No change in performance detected.

///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes(600851475143u64), 6857);
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes(60), 5);
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes(5), 5);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes(
    mut n: u64,
) -> u64 {
    assert!(n > 1);
    fn divide_fully(n: &mut u64, d: u64) {
        while *n % d == 0 {
            *n /= d;
        }
    }

    let basic_primes = [2u64, 3, 5, 7];
    for &d in &basic_primes {
        divide_fully(&mut n, d);
        if n == 1 {
            return d;
        }
    }
    let side = (n as f64).sqrt() as u64;
    let mut divisor = 7u64;
    let steps = [2u64, 2, 2, 4];
    while divisor <= side {
        divisor += steps[0];
        divide_fully(&mut n, divisor);
        if n == 1 {
            return divisor;
        }
        divisor += steps[1];
        divide_fully(&mut n, divisor);
        if n == 1 {
            return divisor;
        }
        divisor += steps[2];
        divide_fully(&mut n, divisor);
        if n == 1 {
            return divisor;
        }
        divisor += steps[3];
        divide_fully(&mut n, divisor);
        if n == 1 {
            return divisor;
        }
    }
    if n == 1 {
        divisor
    } else {
        n
    }
}

fn divide_fully(n: &mut u64, d: u64) {
    while *n % d == 0 {
        *n /= d;
    }
}

///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor;
/// assert_eq!(largest_prime_factor(600851475143u64), 6857);
/// assert_eq!(largest_prime_factor(60), 5);
/// assert_eq!(largest_prime_factor(5), 5);
/// assert_eq!(largest_prime_factor(6), 3);
/// assert_eq!(largest_prime_factor(15), 5);
/// assert_eq!(largest_prime_factor(25698751364526), 328513);
/// assert_eq!(largest_prime_factor(13195), 29);
/// ```
pub
fn largest_prime_factor(mut n: u64) -> u64 {
    assert!(n > 1);
    let basic_primes = [2u64, 3, 5];
    for &d in &basic_primes {
        divide_fully(&mut n, d);
        if n == 1 {
            return d;
        }
    }
    let side = (n as f64).sqrt() as u64;
    let mut divisor = 5u64;
    for i in [2u64, 4].iter().cycle() {
        if divisor > side {
            break;
        }
        divisor += *i;
        divide_fully(&mut n, divisor);
        if n == 1 {
            return divisor;
        }
    }
    if n == 1 {
        divisor
    } else {
        n
    }
}

// 17 us
// pub fn largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab_primes(
//     mut n: u64,
// ) -> u64 {
//     assert!(n > 1);
//     let side = (n as f64).sqrt() as u64;
//     let basic_primes = [2u64, 3, 5, 7];
//     for &d in &basic_primes {
//         while n % d == 0 {
//             n /= d;
//         }
//         if n == 1 {
//             return d;
//         }
//     }

//     let steps = [2u64, 2, 2, 4];
//     let mut i = 0usize;
//     let mut divisor = 7u64;
//     while n != 1 && divisor <= side {
//         if n % divisor == 0 {
//             n /= divisor;
//             while n % divisor == 0 {
//                 n /= divisor;
//             }
//         } else {
//             divisor += steps[i];
//             if i == 3 {
//                 i = 0;
//             } else {
//                 i += 1;
//             }
//         }
//     }
//     if n == 1 {
//         divisor
//     } else {
//         n
//     }
// }
