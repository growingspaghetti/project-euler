//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_all_the_primes_below_two_million --verbose --baseline new
//!     Finished bench [optimized] target(s) in 0.05s
//!      Running target/release/deps/bench_main-e043d12d7fc498f6
//! Benchmarking sum_of_all_the_primes_below_two_million
//! Benchmarking sum_of_all_the_primes_below_two_million: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million: Collecting 100 samples in estimated 5.4632 s (500 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million: Analyzing
//! sum_of_all_the_primes_below_two_million
//!                         time:   [10.855 ms 10.894 ms 10.939 ms]
//!                         change: [-0.5637% +0.0000% +0.5732%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 10 outliers among 100 measurements (10.00%)
//!   7 (7.00%) high mild
//!   3 (3.00%) high severe
//! mean   [10.855 ms 10.939 ms] std. dev.      [147.32 us 285.84 us]
//! median [10.788 ms 10.812 ms] med. abs. dev. [30.572 us 70.966 us]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len: Collecting 100 samples in estimated 5.6122 s (700 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len: Analyzing
//! sum_of_all_the_primes_below_two_million_pow_i_to_matrix_len
//!                         time:   [7.9762 ms 8.0042 ms 8.0370 ms]
//!                         change: [-0.5457% +0.0000% +0.5890%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 11 outliers among 100 measurements (11.00%)
//!   7 (7.00%) high mild
//!   4 (4.00%) high severe
//! mean   [7.9762 ms 8.0370 ms] std. dev.      [99.619 us 210.60 us]
//! median [7.9278 ms 7.9684 ms] med. abs. dev. [29.253 us 78.332 us]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Collecting 100 samples in estimated 5.5388 s (800 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well: Analyzing
//! sum_of_all_the_primes_below_two_million_modified_skip3_as_well
//!                         time:   [6.8274 ms 6.8499 ms 6.8782 ms]
//!                         change: [-0.5292% +0.0000% +0.5256%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 11 outliers among 100 measurements (11.00%)
//!   8 (8.00%) high mild
//!   3 (3.00%) high severe
//! mean   [6.8274 ms 6.8782 ms] std. dev.      [72.762 us 193.00 us]
//! median [6.7964 ms 6.8161 ms] med. abs. dev. [27.134 us 56.150 us]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut: Collecting 100 samples in estimated 5.1046 s (900 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut: Analyzing
//! sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut
//!                         time:   [5.6416 ms 5.6620 ms 5.6862 ms]
//!                         change: [-0.5534% +0.0000% +0.5411%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 12 outliers among 100 measurements (12.00%)
//!   11 (11.00%) high mild
//!   1 (1.00%) high severe
//! mean   [5.6416 ms 5.6862 ms] std. dev.      [72.373 us 159.11 us]
//! median [5.6114 ms 5.6346 ms] med. abs. dev. [29.984 us 61.131 us]
//!
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec: Collecting 100 samples in estimated 5.1985 s (500 iterations)
//! Benchmarking sum_of_all_the_primes_below_two_million_bit_vec: Analyzing
//! sum_of_all_the_primes_below_two_million_bit_vec
//!                         time:   [10.328 ms 10.365 ms 10.407 ms]
//!                         change: [-0.5319% +0.0000% +0.5511%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 14 outliers among 100 measurements (14.00%)
//!   4 (4.00%) high mild
//!   10 (10.00%) high severe
//! mean   [10.328 ms 10.407 ms] std. dev.      [133.60 us 265.39 us]
//! median [10.276 ms 10.294 ms] med. abs. dev. [26.320 us 55.592 us]
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
/// use self::project_euler::m10::sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut;
/// assert_eq!(sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut(), 142913828922);
/// ```
pub fn sum_of_all_the_primes_below_two_million_modified_skip3_as_well_sqrt_tailcut() -> u64 {
    let mut matrix = [true; 2_000_000 + 1]; // n: 2_000_001 i: 0..=2_000_000
    let sqrt = ((matrix.len() - 1) as f64).sqrt().ceil() as usize;
    let mut sum = 5u64; // because the first primes sum([2,3]) are skipped and starts with [5,
    matrix[0] = false;
    matrix[1] = false;
    matrix[4] = false;
    let mut flag_four = false;
    let mut index = 5usize;
    // 3+2 5+2 7+4 11+2 13+4 17+2 19+4
    loop {
        if index > sqrt {
            break;
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
    loop {
        if index >= matrix.len() {
            break sum;
        }
        if matrix[index] {
            sum += index as u64;
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
