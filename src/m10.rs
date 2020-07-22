//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_all_the_primes_below --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!     Finished bench [optimized] target(s) in 8.94s
//!      Running target/release/deps/bench_main-e043d12d7fc498f6
//! Benchmarking sum_of_all_the_primes_below_two_million
//! Benchmarking sum_of_all_the_primes_below_two_million: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million: Collecting 100 samples in estimated 5.5181 s (500 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million: Analyzing
//! sum_of_all_the_primes_below_two_million
//!                         time:   [10.938 ms 10.996 ms 11.062 ms]
//!                         change: [-0.7710% +0.0000% +0.7731%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 11 outliers among 100 measurements (11.00%)
//!   7 (7.00%) high mild
//!   4 (4.00%) high severe
//! mean   [10.938 ms 11.062 ms] std. dev.      [203.51 us 426.69 us]
//! median [10.853 ms 10.911 ms] med. abs. dev. [76.156 us 159.30 us]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len: Collecting 100 samples in estimated 5.7125 s (700 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len: Analyzing
//! sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len
//!                         time:   [8.1322 ms 8.1670 ms 8.2040 ms]
//!                         change: [-0.6384% +0.0000% +0.5891%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//! mean   [8.1322 ms 8.2040 ms] std. dev.      [155.50 us 208.59 us]
//! median [8.0526 ms 8.1572 ms] med. abs. dev. [77.613 us 201.05 us]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Collecting 100 samples in estimated 5.5461 s (800 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Analyzing
//! sum_of_all_the_primes_below_two_million_modified_skip3_as_well
//!                         time:   [6.8694 ms 6.8949 ms 6.9232 ms]
//!                         change: [-0.5597% +0.0000% +0.5735%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 2 outliers among 100 measurements (2.00%)
//!   1 (1.00%) high mild
//!   1 (1.00%) high severe
//! mean   [6.8694 ms 6.9232 ms] std. dev.      [101.00 us 178.85 us]
//! median [6.8400 ms 6.8768 ms] med. abs. dev. [72.980 us 126.71 us]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec: Collecting 100 samples in estimated 5.2538 s (500 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec: Analyzing
//! sum_of_all_the_primes_below_two_million_bit_vec
//!                         time:   [10.400 ms 10.459 ms 10.532 ms]
//!                         change: [-0.9085% +0.0000% +0.9251%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 8 outliers among 100 measurements (8.00%)
//!   5 (5.00%) high mild
//!   3 (3.00%) high severe
//! mean   [10.400 ms 10.532 ms] std. dev.      [188.60 us 496.90 us]
//! median [10.327 ms 10.404 ms] med. abs. dev. [102.93 us 192.92 us]
//!
//! ```
//! When you mark numbers in the matrix as non-prime in the bottom up approach, the is_prime() logic and its loop are not required.
//! See ()[./m7.rs]

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
    let mut sum = 2u64; // because the first prime 2 is skipped and starts with 3
    matrix[0] = false;
    matrix[1] = false;
    for i in (3..matrix.len()).step_by(2) {
        if matrix[i] {
            sum += i as u64;
            for j in (2 * i..matrix.len()).step_by(i) {
                matrix[j] = false;
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
/// use self::project_euler::m10::sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len;
/// assert_eq!(sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len(), 142913828922);
/// ```
pub fn sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len() -> u64 {
    let mut matrix = [true; 2_000_000 + 1]; // n: 2_000_001 i: 0..=2_000_000
    let mut sum = 2u64; // because the first prime 2 is skipped and starts with 3
    matrix[0] = false;
    matrix[1] = false;
    for i in (3..matrix.len()).step_by(2) {
        if matrix[i] {
            sum += i as u64;
            for j in (i * i..matrix.len()).step_by(i) {
                matrix[j] = false;
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
        if matrix[index] {
            sum += index as u64;
            for j in (index * index..matrix.len()).step_by(index) {
                matrix[j] = false;
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

/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
///
/// ```rust
/// use self::project_euler::m10::sum_of_all_the_primes_below_two_million_bit_vec;
/// assert_eq!(sum_of_all_the_primes_below_two_million_bit_vec(), 142913828922);
/// ```
pub fn sum_of_all_the_primes_below_two_million_bit_vec() -> u64 {
    use bit_vec::BitVec;
    let mut bv = BitVec::from_elem(2_000_000 + 1, true);
    let len = bv.len();
    let mut sum = 2u64; // because the first prime 2 is skipped and starts with 3
    bv.set(0, false);
    bv.set(1, false);
    for i in (3..len).step_by(2) {
        if bv[i] {
            sum += i as u64;
            for j in (i * i..len).step_by(i) {
                bv.set(j, false)
            }
        }
    }
    sum
}
