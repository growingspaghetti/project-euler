//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- largest_palindrome_product --verbose --baseline new
//!     Finished bench [optimized] target(s) in 0.05s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking largest_palindrome_product_of_two_3_digits
//! Benchmarking largest_palindrome_product_of_two_3_digits: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.5s, or reduce sample count to 50.
//! Benchmarking largest_palindrome_product_of_two_3_digits: Collecting 100 samples in estimated 9.5441 s (100 iterations)
//! Benchmarking largest_palindrome_product_of_two_3_digits: Analyzing
//! largest_palindrome_product_of_two_3_digits
//!                         time:   [89.334 ms 90.374 ms 91.508 ms]
//!                         change: [-1.7249% +0.0000% +1.7127%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 12 outliers among 100 measurements (12.00%)
//!   7 (7.00%) high mild
//!   5 (5.00%) high severe
//! mean   [89.334 ms 91.508 ms] std. dev.      [4.1927 ms 6.8937 ms]
//! median [88.517 ms 89.844 ms] med. abs. dev. [2.5307 ms 4.8733 ms]
//!
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10: Warming up for 3.0000 s
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10: Collecting 100 samples in estimated 5.5829 s (700 iterations)
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10: Analyzing
//! largest_palindrome_product_of_two_3_digits_mod_10
//!                         time:   [7.9250 ms 7.9640 ms 8.0046 ms]
//!                         change: [-0.6946% +0.0000% +0.7355%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! mean   [7.9250 ms 8.0046 ms] std. dev.      [180.68 us 226.89 us]
//! median [7.8293 ms 7.9643 ms] med. abs. dev. [106.58 us 262.87 us]
//!
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair: Warming up for 3.0000 s
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair: Collecting 100 samples in estimated 5.4125 s (800 iterations)
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair: Analyzing
//! largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair
//!                         time:   [6.5351 ms 6.5685 ms 6.6031 ms]
//!                         change: [-0.7017% +0.0000% +0.7911%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 2 outliers among 100 measurements (2.00%)
//!   2 (2.00%) high mild
//! mean   [6.5351 ms 6.6031 ms] std. dev.      [146.05 us 201.90 us]
//! median [6.4876 ms 6.5756 ms] med. abs. dev. [134.98 us 223.07 us]
//!
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut: Warming up for 3.0000 s
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut: Collecting 100 samples in estimated 5.0265 s (80800 iterations)
//! Benchmarking largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut: Analyzing
//! largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut
//!                         time:   [61.878 us 62.194 us 62.531 us]
//!                         change: [-0.8191% +0.0000% +0.8570%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high severe
//! slope  [61.878 us 62.531 us] R^2            [0.9282094 0.9277869]
//! mean   [62.142 us 62.867 us] std. dev.      [1.4715 us 2.2793 us]
//! median [61.767 us 62.827 us] med. abs. dev. [1.3242 us 2.1558 us]
//!
//! Benchmarking largest_palindrome_product_of_two_3_digits_factorization
//! Benchmarking largest_palindrome_product_of_two_3_digits_factorization: Warming up for 3.0000 s
//! Benchmarking largest_palindrome_product_of_two_3_digits_factorization: Collecting 100 samples in estimated 5.0051 s (444400 iterations)
//! Benchmarking largest_palindrome_product_of_two_3_digits_factorization: Analyzing
//! largest_palindrome_product_of_two_3_digits_factorization
//!                         time:   [11.245 us 11.306 us 11.373 us]
//!                         change: [-0.7906% +0.0000% +0.7688%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high severe
//! slope  [11.245 us 11.373 us] R^2            [0.9155713 0.9148442]
//! mean   [11.332 us 11.461 us] std. dev.      [264.08 ns 410.07 ns]
//! median [11.303 us 11.450 us] med. abs. dev. [234.79 ns 413.06 ns]
//! ```

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits;
/// assert_eq!(largest_palindrome_product_of_two_3_digits(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let s = i.to_string();
        if s.chars().rev().collect::<String>() == s {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_mod_10;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_mod_10(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_mod_10() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        //println!("{} {}", i, rev);
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    // [999     998     997     996]
    // [999-100 998-100 997-100 996-100]
    for x in (100..1000).rev() {
        for y in (100..=x).rev() {
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        //println!("{} {}", i, rev);
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    // [999     998     997     996]
    // [999-100 998-100 997-100 996-100]
    // area -> -> -> -> -> | -> [in this sequence it's always smaller than largest_palindrome]
    for x in (100..1000).rev() {
        for y in (100..=x).rev() {
            if x * y <= largest_palindrome {
                break;
            }
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_factorization;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_factorization(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_factorization() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    // [999     998     997     996]
    // [999-100 998-100 997-100 996-100]
    // area -> -> -> -> -> | -> [in this sequence it's always smaller than largest_palindrome]

    // n is in 6 digits < 999*999=998001.
    // n = xyx_xyx = 100_000x + 10_000y + 1_000z + 100z + 10y + 1x
    //   = 100_001x + 10_010y + 1_100z
    //   = 11 * (9091 + 910 + 100)
    // n = a * b
    //   = 11c * b || a * 11c || 11c * 11d
    for x in (100..1000).rev() {
        if x % 11 == 0 {
            for y in (100..=x).rev() {
                if x * y <= largest_palindrome {
                    break;
                }
                if let Some(i) = is_palindrome(x * y) {
                    if i > largest_palindrome {
                        largest_palindrome = i
                    }
                }
            }
        } else {
            // 999 - 9 (999%11) = 990. 989 - 10 (989%11) = 979.
            for y in (100..=x - x % 11).rev().step_by(11) {
                if x * y <= largest_palindrome {
                    break;
                }
                if let Some(i) = is_palindrome(x * y) {
                    if i > largest_palindrome {
                        largest_palindrome = i
                    }
                }
            }
        }
    }
    largest_palindrome
}
