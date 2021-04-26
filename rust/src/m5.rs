//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- smallest_positive_number_that_is_evenly_divisible_by --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!     Finished bench [optimized] target(s) in 6.21s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_brute
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_brute: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 173.6s, or reduce sample count to 10.
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_brute: Collecting 100 samples in estimated 173.61 s (100 iterations)
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_brute: Analyzing
//! smallest_positive_number_that_is_evenly_divisible_by_each_20_brute
//!                         time:   [1.7640 s 1.7742 s 1.7849 s]
//!                         change: [-0.8300% +0.0000% +0.8668%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! mean   [1.7640 s 1.7849 s] std. dev.      [47.055 ms 60.372 ms]
//! median [1.7329 s 1.7853 s] med. abs. dev. [29.680 ms 78.021 ms]
//!
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes: Warming up for 3.0000 s
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes: Collecting 100 samples in estimated 5.0100 s (1040300 iterations)
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes: Analyzing
//! smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes
//!                         time:   [4.6418 us 4.6613 us 4.6824 us]
//!                         change: [-0.6734% +0.0000% +0.7168%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 6 outliers among 100 measurements (6.00%)
//!   3 (3.00%) high mild
//!   3 (3.00%) high severe
//! slope  [4.6418 us 4.6824 us] R^2            [0.9505523 0.9501683]
//! mean   [4.6487 us 4.6958 us] std. dev.      [89.969 ns 148.02 ns]
//! median [4.6256 us 4.6712 us] med. abs. dev. [61.261 ns 102.76 ns]
//!
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd: Warming up for 3.0000 s
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd: Collecting 100 samples in estimated 5.0009 s (8797100 iterations)
//! Benchmarking smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd: Analyzing
//! smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd
//!                         time:   [567.72 ns 570.55 ns 573.48 ns]
//!                         change: [-0.6335% +0.0000% +0.6186%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 3 outliers among 100 measurements (3.00%)
//!   1 (1.00%) high mild
//!   2 (2.00%) high severe
//! slope  [567.72 ns 573.48 ns] R^2            [0.9453902 0.9451896]
//! mean   [568.74 ns 573.92 ns] std. dev.      [8.5358 ns 18.174 ns]
//! median [566.88 ns 572.40 ns] med. abs. dev. [7.6475 ns 11.706 ns]
//! ```
//! ![](https://upload.wikimedia.org/wikipedia/commons/1/1a/GCM_Of_20_And_32.gif)

/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_brute;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_brute(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_brute() -> u32 {
    let mut num = 1;
    while !(2..=20).all(|x| num % x == 0) {
        num += 1;
    }
    num
}

fn is_divisible_by_2_20(n: u32) -> bool {
    for d in 2..=20 {
        if n % d != 0 {
            return false;
        }
    }
    true
}

/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_brute_syntax;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_brute_syntax(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_brute_syntax() -> u32 {
    let mut n = 1;
    while !is_divisible_by_2_20(n) {
        n += 1;
    }
    n
}

/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes() -> u32 {
    use std::collections::HashMap;

    let prime_map = |mut i: u8| -> HashMap<u8, u8> {
        let mut prime_and_exp: HashMap<u8, u8> = HashMap::new();
        let mut diviser = 2;
        while i > 0 && diviser <= i {
            if i % diviser == 0 {
                prime_and_exp.insert(diviser, *prime_and_exp.get(&diviser).unwrap_or(&0) + 1);
                i /= diviser;
            } else {
                diviser += 1;
            }
        }
        prime_and_exp
    };

    let mut prime_and_exp: HashMap<u8, u8> = HashMap::new();
    for i in 2..=20u8 {
        // [2^1, 3^1, 2^2, 5^1, 3^1*2^1, 7^1, 2^3, 3^2, 5^1*2^1, .. 5^1*2^2]
        let map = prime_map(i);
        for (&p, &e) in &map {
            let &e_parent = prime_and_exp.get(&p).unwrap_or(&0);
            if e_parent < e {
                prime_and_exp.insert(p, e);
            }
        }
    }

    let mut sum = 1u32;
    //println!("{:#?}", prime_and_exp);
    for (&p, &e) in &prime_and_exp {
        sum *= p.pow(e.into()) as u32;
    }
    sum
}

#[derive(Clone)]
struct Factor {
    prime: u32,
    occurrence: u32,
}

fn increment_occurrence(factors: &mut [Option<Factor>; 20], p: u32) {
    assert!(p < 20);
    match factors[p as usize].as_mut() {
        Some(f) => {
            f.occurrence += 1;
        }
        None => {
            factors[p as usize] = Some(Factor {
                prime: p,
                occurrence: 1,
            })
        }
    }
}

fn list_factors(mut n: u32) -> [Option<Factor>; 20] {
    assert!(n <= 20);
    let mut factors: [Option<Factor>; 20] = Default::default();
    let mut d = 2u32;
    while n > 1 {
        while n % d == 0 {
            increment_occurrence(&mut factors, d);
            n /= d;
        }
        d += 1;
    }
    factors
}

fn merge(factors: &mut [Option<Factor>; 20], b: &[Option<Factor>; 20]) {
    for it in b.iter().zip(factors.iter_mut()) {
        match it {
            (Some(bf), Some(f)) => {
                if bf.occurrence > f.occurrence {
                    f.occurrence = bf.occurrence;
                }
            }
            (Some(bf), None) => {
                let (_, f) = it;
                *f = Some(bf.clone());
            }
            _ => (),
        }
    }
}

// smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct
//                         time:   [1.0688 us 1.0782 us 1.0884 us]
//                         change: [-1.1471% +0.0000% +1.2265%] (p = 1.00 > 0.05)
//                         No change in performance detected.
// Found 2 outliers among 100 measurements (2.00%)

/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_primes_struct() -> u32 {
    let mut factors: [Option<Factor>; 20] = Default::default();
    for n in 2..=20u32 {
        let local_factors = list_factors(n);
        merge(&mut factors, &local_factors);
    }
    let mut acc = 1u32;
    for o in &factors {
        if let Some(f) = o {
            acc *= f.prime.pow(f.occurrence);
        }
    }
    acc
}

/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd() -> u64 {
    // https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
    // 1. least common multiple(a, b) * greatest common divisor(a, b) = a * b
    //   ex, lcm(24, 14) = lcm(2^3*3^1, 2^1*7^1) = 2^3*3*7 = 168
    //       gcd(24, 14) = 2
    //       24 * 14 = 336 = 168 * 2
    // https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclid's_algorithm
    // 2. greatest common divisor has Euclid's_algorithm
    //   ex, 24 / 14 = q:1 r:10
    //       10 < 14
    //       14 / 10 = q:1 r:4
    //        4 < 10
    //       10 /  4 = q:2 r:2
    //        2 <  4
    //        4 /  2 = q:2 r:0
    //             2#this dividor is the gcd of 14 and 24
    fn greatest_common_factor(a: u64, b: u64) -> u64 {
        if b > a {
            greatest_common_factor(b, a)
        } else if b == 0 {
            a
        } else {
            greatest_common_factor(b, a % b)
        }
    }
    let least_common_multiple = |a: u64, b: u64| -> u64 { a * b / greatest_common_factor(a, b) };

    let mut lcm = 1u64;
    for i in 2..=20u64 {
        lcm = least_common_multiple(lcm, i);
    }
    lcm
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    assert!(b != 0);
    let r = a % b;
    if r == 0 {
        return b;
    }
    gcd(b, r)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_format;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_format(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_format() -> u64 {
    let mut acc = 2u64;
    for n in 3..=20u64 {
        acc = lcm(acc, n);
    }
    acc
}

/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_fold;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_fold(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_lcm_gcd_fold() -> u64 {
    (3..=20u64).into_iter().fold(2u64, |a, b| lcm(a, b))
}
