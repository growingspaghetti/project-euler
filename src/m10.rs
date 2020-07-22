//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_all_the_primes_below_two --verbose
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!     Finished bench [optimized] target(s) in 8.79s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking sum_of_all_the_primes_below_two_million
//! Benchmarking sum_of_all_the_primes_below_two_million: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 67.2s, or reduce sample count to 10.
//! Benchmarking sum_of_all_the_primes_below_two_million: Collecting 100 samples in estimated 67.207 s (100 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million: Analyzing
//! sum_of_all_the_primes_below_two_million
//!                         time:   [671.28 ms 673.75 ms 676.66 ms]
//!                         change: [+0.4567% +0.8653% +1.3365%] (p = 0.00 < 0.05)
//!                         Change within noise threshold.
//! Found 8 outliers among 100 measurements (8.00%)
//!   2 (2.00%) high mild
//!   6 (6.00%) high severe
//! mean   [671.28 ms 676.66 ms] std. dev.      [8.1250 ms 18.921 ms]
//! median [669.37 ms 671.18 ms] med. abs. dev. [4.1897 ms 6.7778 ms]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_modified
//! Benchmarking sum_of_all_the_primes_below_two_million_modified: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 66.9s, or reduce sample count to 10.
//! Benchmarking sum_of_all_the_primes_below_two_million_modified: Collecting 100 samples in estimated 66.888 s (100 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_modified: Analyzing
//! sum_of_all_the_primes_below_two_million_modified
//!                         time:   [666.89 ms 667.58 ms 668.28 ms]
//!                         change: [-0.1841% -0.0395% +0.1020%] (p = 0.60 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//! mean   [666.89 ms 668.28 ms] std. dev.      [3.0265 ms 4.1454 ms]
//! median [666.39 ms 668.56 ms] med. abs. dev. [2.8950 ms 4.4666 ms]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 68.8s, or reduce sample count to 10.
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Collecting 100 samples in estimated 68.767 s (100 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Analyzing
//! sum_of_all_the_primes_below_two_million_modified_skip3_as_well
//!                         time:   [685.39 ms 686.07 ms 686.74 ms]
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//! mean   [685.39 ms 686.74 ms] std. dev.      [2.9463 ms 3.9102 ms]
//! median [685.33 ms 686.79 ms] med. abs. dev. [2.3510 ms 4.1134 ms]
//! ```
//!
//! See [./m7.rs]

/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
///
/// ```rust
/// use self::project_euler::m10::sum_of_all_the_primes_below_two_million;
/// assert_eq!(sum_of_all_the_primes_below_two_million(), 142913828922);
/// ```
pub fn sum_of_all_the_primes_below_two_million() -> u64 {
    let mut matrix = [true; 2_000_000 + 1]; // n: 2_000_001 i: 0..=2_000_000
    fn is_prime(i: u64) -> bool {
        if i == 0 || i == 1 {
            false
        } else if i == 2 || i == 3 {
            true
        } else if i % 2 == 0 {
            false
        } else if i < 9 {
            true
        } else if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            false
        } else {
            let mut d = 3u64;
            let max = (i as f64).sqrt().ceil() as u64;
            loop {
                if d > max {
                    break true;
                }
                if i % d == 0 {
                    break false;
                }
                d += 2;
            }
        }
    }
    let mut sum = 2u64; // because the first prime 2 is skipped and starts with 3
    matrix[0] = false;
    matrix[1] = false;
    for i in (3..matrix.len()).step_by(2) {
        if matrix[i] && is_prime(i as u64) {
            sum += i as u64;
            for i in (2 * i..matrix.len()).step_by(i) {
                matrix[i] = false;
            }
        }
    }
    sum
}

/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
///
/// ```rust
/// use self::project_euler::m10::sum_of_all_the_primes_below_two_million_modified;
/// assert_eq!(sum_of_all_the_primes_below_two_million_modified(), 142913828922);
/// ```
pub fn sum_of_all_the_primes_below_two_million_modified() -> u64 {
    let mut matrix = [true; 2_000_000 + 1]; // n: 2_000_001 i: 0..=2_000_000
    fn is_prime(i: u64) -> bool {
        if i == 0 || i == 1 {
            false
        } else if i == 2 || i == 3 {
            true
        } else if i % 2 == 0 {
            false
        } else if i < 9 {
            true
        } else if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            false
        } else {
            let mut d = 3u64;
            let max = (i as f64).sqrt().ceil() as u64;
            loop {
                if d > max {
                    break true;
                }
                if i % d == 0 {
                    break false;
                }
                d += 2;
            }
        }
    }
    let mut sum = 2u64; // because the first prime 2 is skipped and starts with 3
    matrix[0] = false;
    matrix[1] = false;
    for i in (3..matrix.len()).step_by(2) {
        if matrix[i] && is_prime(i as u64) {
            sum += i as u64;
            for i in (i * i..matrix.len()).step_by(i) {
                matrix[i] = false;
            }
        }
    }
    sum
}

/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
///
/// ```rust
/// use self::project_euler::m10::sum_of_all_the_primes_below_two_million_modified_skip3_as_well;
/// assert_eq!(sum_of_all_the_primes_below_two_million_modified_skip3_as_well(), 142913828922);
/// ```
pub fn sum_of_all_the_primes_below_two_million_modified_skip3_as_well() -> u64 {
    let mut matrix = [true; 2_000_000 + 1]; // n: 2_000_001 i: 0..=2_000_000
    fn is_prime(i: u64) -> bool {
        if i == 0 || i == 1 {
            false
        } else if i == 2 || i == 3 {
            true
        } else if i % 2 == 0 {
            false
        } else if i < 9 {
            true
        } else if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            false
        } else {
            let mut d = 3u64;
            let max = (i as f64).sqrt().ceil() as u64;
            loop {
                if d > max {
                    break true;
                }
                if i % d == 0 {
                    break false;
                }
                d += 2;
            }
        }
    }
    let mut sum = 5u64; // because the first primes sum([2,3]) are skipped and starts with [5,
    matrix[0] = false;
    matrix[1] = false;
    matrix[4] = false;
    let mut flag_four = false;
    let mut index = 5usize;
    // 3+2 5+2 7+4 11+2 13+4 17+2 19+4
    loop {
        if index >= matrix.len() {
            break sum;
        }
        if matrix[index] && is_prime(index as u64) {
            sum += index as u64;
            for i in (index * index..matrix.len()).step_by(index) {
                matrix[i] = false;
            }
        }
        if flag_four {
            index += 4;
            flag_four = false;
        } else {
            index += 2;
            flag_four = true;
        }
    }
}
