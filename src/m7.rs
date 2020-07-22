//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- the_10001st_prime_number --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!     Finished bench [optimized] target(s) in 7.14s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking the_10001st_prime_number
//! Benchmarking the_10001st_prime_number: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number: Collecting 100 samples in estimated 6.0320 s (400 iterations)
//! Benchmarking the_10001st_prime_number: Analyzing
//! the_10001st_prime_number
//!                         time:   [14.857 ms 14.904 ms 14.954 ms]
//!                         change: [-0.4681% +0.0000% +0.4520%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   4 (4.00%) high mild
//! mean   [14.857 ms 14.954 ms] std. dev.      [206.77 us 285.30 us]
//! median [14.792 ms 14.936 ms] med. abs. dev. [179.14 us 290.55 us]
//! 
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_mod3
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_mod3: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_mod3: Collecting 100 samples in estimated 5.7103 s (400 iterations)
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_mod3: Analyzing
//! the_10001st_prime_number_sieve_of_eratosthenes_mod3
//!                         time:   [14.191 ms 14.227 ms 14.265 ms]
//!                         change: [-0.3812% +0.0000% +0.3585%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   4 (4.00%) high mild
//! mean   [14.191 ms 14.265 ms] std. dev.      [156.59 us 220.70 us]
//! median [14.150 ms 14.245 ms] med. abs. dev. [126.61 us 202.42 us]
//! 
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1: Collecting 100 samples in estimated 5.8043 s (400 iterations)
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1: Analyzing
//! the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1
//!                         time:   [14.366 ms 14.405 ms 14.445 ms]
//!                         change: [-0.3828% +0.0000% +0.3793%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//! mean   [14.366 ms 14.445 ms] std. dev.      [168.65 us 233.28 us]
//! median [14.314 ms 14.419 ms] med. abs. dev. [124.76 us 230.74 us]
//! 
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix: Collecting 100 samples in estimated 5.4940 s (400 iterations)
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix: Analyzing
//! the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix
//!                         time:   [13.666 ms 13.711 ms 13.758 ms]
//!                         change: [-0.4672% +0.0000% +0.4638%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   4 (4.00%) high mild
//! mean   [13.666 ms 13.758 ms] std. dev.      [193.45 us 267.22 us]
//! median [13.640 ms 13.721 ms] med. abs. dev. [161.22 us 266.44 us]
//! ```
//! See [](./m3.rs)

/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///
/// What is the 10 001st prime number?
///
/// ```rust
/// use self::project_euler::m7::the_10001st_prime_number;
/// assert_eq!(the_10001st_prime_number(10001), 104743);
/// ```
pub fn the_10001st_prime_number(nth: u32) -> u64 {
    fn is_prime(i: u64) -> bool {
        if i % 2 == 0 {
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
    let mut n = 3u64;
    let mut counter = 1u32; // as skiped 2 (no.1)
    loop {
        if is_prime(n) {
            counter += 1;
            if counter == nth {
                break n;
            }
        }
        n += 2;
    }
}

/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///
/// What is the 10 001st prime number?
///
/// ```rust
/// use self::project_euler::m7::the_10001st_prime_number_sieve_of_eratosthenes_mod3;
/// assert_eq!(the_10001st_prime_number_sieve_of_eratosthenes_mod3(10001), 104743);
/// ```
pub fn the_10001st_prime_number_sieve_of_eratosthenes_mod3(nth: u32) -> u64 {
    fn is_prime(i: u64) -> bool {
        if i == 1 {
            false
        } else if i == 2 || i == 3 {
            true
        } else if i % 2 == 0 {
            false
        } else if i < 9 {
            // 5 7
            true
        } else if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            false
        } else {
            // 11-
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
    let mut n = 3u64;
    let mut counter = 1u32; // as skiped 2 (no.1)
    loop {
        if is_prime(n) {
            counter += 1;
            if counter == nth {
                break n;
            }
        }
        n += 2;
    }
}

/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///
/// What is the 10 001st prime number?
///
/// ```rust
/// use self::project_euler::m7::the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1;
/// assert_eq!(the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1(10001), 104743);
/// ```
pub fn the_10001st_prime_number_sieve_of_eratosthenes_6k_p1_m1(nth: u32) -> u64 {
    fn is_prime(i: u64) -> bool {
        if i == 1 {
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
    let mut n = 0u64;
    let mut counter = 2u32; // as skiped 2, 3
    loop {
        n += 6;
        if is_prime(n + 1) {
            counter += 1;
            if counter == nth {
                break n + 1;
            }
        }
        if is_prime(n - 1) {
            counter += 1;
            if counter == nth {
                break n + 1;
            }
        }
    }
}

/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///
/// What is the 10 001st prime number?
///
/// ```rust
/// use self::project_euler::m7::the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix;
/// assert_eq!(the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix(10001), 104743);
/// ```
pub fn the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix(nth: u32) -> u64 {
    let mut matrix = [true; 1000000];
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
    let mut counter = 1u32; // skipped 2 (no.1)
    let mut index = 3usize;
    matrix[0] = false;
    matrix[1] = false;
    loop {
        if !matrix[index] {
            index += 2;
            continue;
        }
        if is_prime(index as u64) {
            counter += 1;
            if counter == nth {
                break index as u64;
            } else {
                for i in (2 * index..matrix.len()).step_by(index) {
                    matrix[i] = false;
                }
            }
        }
        index += 2;
    }
}
