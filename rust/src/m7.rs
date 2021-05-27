//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- the_10001st_prime_number --verbose --baseline new
//!     Finished bench [optimized] target(s) in 0.13s
//!      Running target/release/deps/bench_main-e043d12d7fc498f6
//! Benchmarking the_10001st_prime_number
//! Benchmarking the_10001st_prime_number: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number: Collecting 100 samples in estimated 5.6934 s (400 iterations)
//! Benchmarking the_10001st_prime_number: Analyzing
//! the_10001st_prime_number
//!                         time:   [14.888 ms 15.028 ms 15.171 ms]
//!                         change: [-1.1649% +0.0000% +1.3746%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! mean   [14.888 ms 15.171 ms] std. dev.      [647.69 us 794.76 us]
//! median [14.743 ms 15.282 ms] med. abs. dev. [665.48 us 1.0652 ms]//!

//! Benchmarking the_10001st_prime_number_mod3
//! Benchmarking the_10001st_prime_number_mod3: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number_mod3: Collecting 100 samples in estimated 5.5024 s (400 iterations)
//! Benchmarking the_10001st_prime_number_mod3: Analyzing
//! the_10001st_prime_number_mod3
//!                         time:   [13.682 ms 13.731 ms 13.784 ms]
//!                         change: [-0.5404% +0.0000% +0.5351%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 7 outliers among 100 measurements (7.00%)
//!   7 (7.00%) high mild
//! mean   [13.682 ms 13.784 ms] std. dev.      [209.87 us 302.95 us]
//! median [13.619 ms 13.705 ms] med. abs. dev. [151.14 us 261.68 us]//!

//! Benchmarking the_10001st_prime_number_6k_p1_m1
//! Benchmarking the_10001st_prime_number_6k_p1_m1: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number_6k_p1_m1: Collecting 100 samples in estimated 5.6354 s (400 iterations)
//! Benchmarking the_10001st_prime_number_6k_p1_m1: Analyzing
//! the_10001st_prime_number_6k_p1_m1
//!                         time:   [13.989 ms 14.036 ms 14.086 ms]
//!                         change: [-0.4970% +0.0000% +0.4979%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//! mean   [13.989 ms 14.086 ms] std. dev.      [207.17 us 289.01 us]
//! median [13.899 ms 14.037 ms] med. abs. dev. [164.17 us 299.57 us]//!

//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix: Warming up for 3.0000 s
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix: Collecting 100 samples in estimated 5.1039 s (1300 iterations)
//! Benchmarking the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix: Analyzing
//! the_10001st_prime_number_sieve_of_eratosthenes_larger_matrix
//!                         time:   [3.9112 ms 3.9333 ms 3.9586 ms]
//!                         change: [-0.8296% +0.0000% +0.8381%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//!   4 (4.00%) high mild
//!   1 (1.00%) high severe
//! mean   [3.9112 ms 3.9586 ms] std. dev.      [82.632 us 162.57 us]
//! median [3.8762 ms 3.9051 ms] med. abs. dev. [42.126 us 87.216 us]
//! ```
//! [https://brilliant.org/wiki/sieve-of-eratosthenes/](https://brilliant.org/wiki/sieve-of-eratosthenes/)
//! See [./m3.rs](./m3.rs)

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
/// use self::project_euler::m7::the_10001st_prime_number_mod3;
/// assert_eq!(the_10001st_prime_number_mod3(10001), 104743);
/// ```
pub fn the_10001st_prime_number_mod3(nth: u32) -> u64 {
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
/// use self::project_euler::m7::the_10001st_prime_number_6k_p1_m1;
/// assert_eq!(the_10001st_prime_number_6k_p1_m1(10001), 104743);
/// ```
pub fn the_10001st_prime_number_6k_p1_m1(nth: u32) -> u64 {
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
    let mut counter = 1u32; // skipped 2 (no.1)
    let mut index = 3usize;
    matrix[0] = false;
    matrix[1] = false;
    loop {
        if matrix[index] {
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

fn rule_out(sieve: &mut Vec<bool>, prime: usize) {
    for i in (prime..sieve.len()).step_by(prime) {
        sieve[i] = false;
    }
}

fn extend(sieve: &mut Vec<bool>, primes: &Vec<usize>) {
    sieve.extend(vec![true; sieve.len()]);
    for &p in primes {
        rule_out(&mut sieve.as_mut(), p);
    }
}

// the_10001st_prime_number_sieve_of_eratosthenes_extend
//                         time:   [2.1864 ms 2.1980 ms 2.2100 ms]
//                         change: [-0.7583% +0.0000% +0.7446%] (p = 1.00 > 0.05)
//                         No change in performance detected.

/// ```rust
/// use self::project_euler::m7::the_10001st_prime_number_sieve_of_eratosthenes_extend;
/// assert_eq!(the_10001st_prime_number_sieve_of_eratosthenes_extend(10001), 104743);
/// ```
pub fn the_10001st_prime_number_sieve_of_eratosthenes_extend(nth: u32) -> u64 {
    let mut counter = 0u32;
    let mut sieve = vec![true; 10000];
    let mut primes: Vec<usize> = vec![];
    sieve[0] = false;
    sieve[1] = false;
    let mut cursor = 0usize;
    loop {
        for i in cursor..sieve.len() {
            if !sieve[i] {
                continue;
            }
            counter += 1;
            if counter == nth {
                return i as u64;
            }
            &primes.push(i);
            rule_out(&mut sieve, i);
        }
        cursor = sieve.len() - 1;
        extend(&mut sieve, &primes);
    }
}


fn is_prime_5(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 {
        return true;
    }
    for d in &[2u64, 3, 5] {
        if n % *d == 0 {
            return false;
        }
    }

    let side = (n as f64).sqrt() as u64;
    let mut d = 5u64;
    for i in [2, 4].iter().cycle() {
        if d > side {
            break;
        }
        d += *i;
        if n % d == 0 {
            return false;
        }
    }
    true
}

// the_10001st_prime_number_mod3_syntax                                                                             
//                         time:   [4.8235 ms 4.8504 ms 4.8790 ms]
//                         change: [-0.7991% +0.0000% +0.8361%] (p = 1.00 > 0.05)
//                         No change in performance detected.

/// ```rust
/// use self::project_euler::m7::the_10001st_prime_number_mod3_syntax;
/// assert_eq!(the_10001st_prime_number_mod3_syntax(10001), 104743);
/// ```
pub fn the_10001st_prime_number_mod3_syntax(nth: u32) -> u64 {
    let mut n = 0u64;
    let mut counter = 0u32;
    while counter < nth {
        n += 1;
        if is_prime_5(n) {
            counter += 1;
        }
    }
    n
}
