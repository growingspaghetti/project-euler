//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_all_the_amicable_numbers_under_ --verbose
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 14.63s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 33.7s, or reduce sample count to 10.
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute: Collecting 100 samples in estimated 33.713 s (100 iterations)
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute: Analyzing
//! sum_of_all_the_amicable_numbers_under_10000_brute
//!                         time:   [333.11 ms 333.42 ms 333.76 ms]
//! Found 2 outliers among 100 measurements (2.00%)
//!   1 (1.00%) high mild
//!   1 (1.00%) high severe
//! mean   [333.11 ms 333.76 ms] std. dev.      [1.2305 ms 2.1094 ms]
//! median [332.71 ms 333.88 ms] med. abs. dev. [1.3296 ms 1.9056 ms]
//!
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 23.5s, or reduce sample count to 20.
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache: Collecting 100 samples in estimated 23.471 s (100 iterations)
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache: Analyzing
//! sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache
//!                         time:   [232.95 ms 233.36 ms 233.82 ms]
//! Found 2 outliers among 100 measurements (2.00%)
//!   1 (1.00%) high mild
//!   1 (1.00%) high severe
//! mean   [232.95 ms 233.82 ms] std. dev.      [1.4617 ms 2.9985 ms]
//! median [232.24 ms 233.37 ms] med. abs. dev. [970.01 us 2.0710 ms]
//!
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_prime_factor
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_prime_factor: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_prime_factor: Collecting 100 samples in estimated 5.1553 s (1300 iterations)
//! Benchmarking sum_of_all_the_amicable_numbers_under_10000_prime_factor: Analyzing
//! sum_of_all_the_amicable_numbers_under_10000_prime_factor
//!                         time:   [3.7143 ms 3.7326 ms 3.7518 ms]
//!                         change: [-0.7309% +0.0000% +0.6928%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! mean   [3.7143 ms 3.7518 ms] std. dev.      [82.857 us 109.10 us]
//! median [3.6657 ms 3.7361 ms] med. abs. dev. [38.558 us 121.39 us]
//! ```
//! See (m12 Divisor function)[./m12.rs]; also (m3)[./m3.rs], (m7)[./m7.rs], (m10)[./m10.rs]
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/5d5689800970b38ea223075d4769abcaef1e5b02)
//! https://www.wolframalpha.com/input/?i=243

use std::usize;

///
///
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.
///
/// ```rust
/// use self::project_euler::m21::sum_of_all_the_amicable_numbers_under_10000_brute;
/// assert_eq!(sum_of_all_the_amicable_numbers_under_10000_brute(), 31626);
/// ```
pub fn sum_of_all_the_amicable_numbers_under_10000_brute() -> u32 {
    fn list_proper_divisors(num: u32) -> Vec<u32> {
        let mut proper_divisors: Vec<u32> = vec![];
        for d in 1..num {
            if num % d == 0 {
                proper_divisors.push(d);
            }
        }
        proper_divisors
    }
    let mut amicable_numbers: Vec<(u32, u32)> = vec![];
    for a in 1..10_000u32 {
        let a_proper_divisors = list_proper_divisors(a);
        let a_sum = a_proper_divisors.iter().sum::<u32>();
        // perfect number. exclusive
        if a_sum == a {
            continue;
        }

        let b = a_sum;
        // confusing but a -> p_divisors(a) -> a_sum
        //      a_sum as b -> p_divisors(b) -> b_sum
        // bool = if a == b_sum; if true -> pair(a, b) for looping condition established
        let b_proper_divisors = list_proper_divisors(b);
        let b_sum = b_proper_divisors.iter().sum::<u32>();

        if a == b_sum {
            // this condition actually never be satisfied
            if b > 9_999 {
                continue;
            }
            amicable_numbers.push((a, b));
            // 220 284
            // 284 220
            // 1184 1210
            // 1210 1184
            // 2620 2924
            // 2924 2620
            // 5020 5564
            // 5564 5020
            // 6232 6368
            // 6368 6232
            // println!("{} {}", i, sum_of_proper_divisors)
        }
    }
    let mut sum = 0u32;
    for (a, b) in amicable_numbers {
        sum += a + b
    }
    sum / 2
}

///
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.
///
/// ```rust
/// use self::project_euler::m21::sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache;
/// assert_eq!(sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache(), 31626);
/// ```
pub fn sum_of_all_the_amicable_numbers_under_10000_brute_pair_cache() -> u32 {
    fn list_proper_divisors(num: u32) -> Vec<u32> {
        let mut proper_divisors: Vec<u32> = vec![];
        for d in 1..num {
            if num % d == 0 {
                proper_divisors.push(d);
            }
        }
        proper_divisors
    }

    let checked = &mut [false; 10_000 /*last cell is checked[9999]*/];
    checked[0] = true;
    let mut amicable_numbers: Vec<(u32, u32)> = vec![];
    for a in 1..10_000u32 {
        if checked[a as usize] {
            continue;
        }
        let a_proper_divisors = list_proper_divisors(a);
        let a_sum = a_proper_divisors.iter().sum::<u32>();
        // perfect number. exclusive
        if a_sum == a {
            checked[a as usize] = true;
            continue;
        }

        // because it must be checked due to the ascending order of this loop
        if a_sum < a {
            checked[a as usize] = true;
            continue;
        }

        // name a_sum as b
        let b = a_sum;
        let b_proper_divisors = list_proper_divisors(b);
        let b_sum = b_proper_divisors.iter().sum::<u32>();
        if (b as usize) < checked.len() {
            checked[b as usize] = true;
        }

        if a == b_sum {
            // this condition actually never be satisfied
            if b > 9_999 {
                continue;
            }
            amicable_numbers.push((a, b));
            // 220 284
            // 1184 1210
            // 2620 2924
            // 5020 5564
            // 6232 6368
            // println!("{} {}", i, sum_of_proper_divisors)
        }
    }
    let mut sum = 0u32;
    for (a, b) in amicable_numbers {
        sum += a + b
    }
    sum
}

/// ```rust
/// use self::project_euler::m21::factoring;
/// assert_eq!(factoring(6), vec![(2u64, 1u64), (3u64, 1u64)]);
/// assert_eq!(factoring(1), vec![]);
/// assert_eq!(factoring(9), vec![(3u64, 2u64)]);
/// assert_eq!(factoring(241), vec![(241u64, 1u64)]);
/// assert_eq!(factoring(247445), vec![(5u64, 1u64), (11u64, 2u64), (409u64, 1u64)]);
/// assert_eq!(factoring(10), vec![(2u64, 1u64), (5u64, 1u64)]);
/// assert_eq!(factoring(42), vec![(2u64, 1u64), (3u64, 1u64), (7u64, 1u64)]);
/// assert_eq!(factoring(84), vec![(2u64, 2u64), (3u64, 1u64), (7u64, 1u64)]);
/// assert_eq!(factoring(588), vec![(2u64, 2u64), (3u64, 1u64), (7u64, 2u64)]);
/// ```
pub fn factoring(num: u64) -> Vec<(u64, u64)> {
    let mut prime_factors: Vec<(u64, u64)> = vec![];
    let mut tmp = num;
    if num == 1 {
        return prime_factors;
    }

    for d in 2..=3u64 {
        let mut exponent = 0u64;
        while tmp % d == 0 {
            exponent += 1;
            tmp /= d;
        }
        if exponent > 0 {
            prime_factors.push((d, exponent));
        }
    }

    // side a, regarding num is the are of a rectangle.
    // when side_a and side_b both are bigger than sqrt(S), the area doesn't fit in.
    let side_max = (num as f64).sqrt().ceil() as u64;
    let mut d = 5u64;
    let mut skip_four = false;
    loop {
        if d > side_max || tmp == 1 {
            break;
        }
        let mut exponent = 0u64;
        while tmp % d == 0 {
            exponent += 1;
            tmp /= d;
        }
        if exponent > 0 {
            prime_factors.push((d, exponent));
        }
        if skip_four {
            d += 4;
            skip_four = false;
        } else {
            d += 2;
            skip_four = true;
        }
    }
    // side b left is a prime
    if tmp != 1 {
        prime_factors.push((tmp, 1));
    }
    prime_factors
}

/// https://en.wikipedia.org/wiki/Divisor_function#Definition
/// (2^0+2^1+2^2...2^x)*(3^0+3^1...3^y)*(5^0+5^1...5^z)...
/// eg. 6 = 2^1 * 3^1;
///     sigma_one(6) = (1+2)*(1+3) = 12 = sum(1 2 3 6)
///     12 = 2^2 * 3;
///     sigma_one(12) = (1+2+2^2)*(1+3) = 28 = sum(1 2 3 4 6 12)
///     10 = 2 * 5;
///     sigma_one(10) = (1+2)*(1+5) = 18 = sum(1 2 5 10)
///```rust
/// use self::project_euler::m21::divisor_function_sigma_one_function;
/// assert_eq!(divisor_function_sigma_one_function(6), 12u64);
/// assert_eq!(divisor_function_sigma_one_function(12), 28u64);
/// assert_eq!(divisor_function_sigma_one_function(10), 18u64);
/// assert_eq!(divisor_function_sigma_one_function(8), 15u64);
/// assert_eq!(divisor_function_sigma_one_function(9), 13u64);
///```
pub fn divisor_function_sigma_one_function(num: u64) -> u64 {
    let prime_factors = factoring(num);
    let mut total = 1u64;
    for (prime, exp) in prime_factors {
        let mut subtotal = 0u64;
        for i in 0..=exp {
            subtotal += prime.pow(i as u32)
        }
        total *= subtotal;
    }
    total
}

///
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.
///
/// ```rust
/// use self::project_euler::m21::sum_of_all_the_amicable_numbers_under_10000_prime_factor;
/// assert_eq!(sum_of_all_the_amicable_numbers_under_10000_prime_factor(), 31626);
/// ```
pub fn sum_of_all_the_amicable_numbers_under_10000_prime_factor() -> u64 {
    fn proper_divisors_sigma_one(num: u64) -> u64 {
        if num < 2 {
            panic!()
        }
        divisor_function_sigma_one_function(num) - num
    };

    let checked = &mut [false; 10_000 /*last cell is checked[9999]*/];
    checked[0] = true;
    checked[1] = true;
    let mut amicable_numbers: Vec<(u64, u64)> = vec![];
    for a in 2..10_000u64 {
        if checked[a as usize] {
            continue;
        }
        let a_sum = proper_divisors_sigma_one(a as u64);
        // perfect number. exclusive
        if a_sum == a {
            checked[a as usize] = true;
            continue;
        }

        // because it must be checked due to the ascending order of this loop
        if a_sum < a {
            checked[a as usize] = true;
            continue;
        }

        // name a_sum as b
        let b = a_sum;
        let b_sum = proper_divisors_sigma_one(b as u64);
        if (b as usize) < checked.len() {
            checked[b as usize] = true;
        }

        if a == b_sum {
            // this condition actually never be satisfied
            if b > 9_999 {
                continue;
            }
            amicable_numbers.push((a, b));
            // 220 284
            // 1184 1210
            // 2620 2924
            // 5020 5564
            // 6232 6368
            // println!("{} {}", i, a_sum)
        }
    }
    let mut sum = 0u64;
    for (a, b) in amicable_numbers {
        sum += a + b
    }
    sum
}

// trait AmicableNumber {
//     fn has_amicable_pair(under: u32) -> bool;
// }

struct AmicableNumberScanner {
    under: u32,
    _primes: Vec<u32>,
    _aliquot_sum: Vec<u32>,
}

impl AmicableNumberScanner {
    fn new(under: u32) -> Self {
        AmicableNumberScanner {
            under: under,
            _primes: vec![2, 3],
            _aliquot_sum: vec![0u32; under as usize],
        }
    }
    fn _divide_fully(&self, n: &mut u32, d: u32, side: &mut u32, sum: &mut u32) {
        if *n % d == 0 {
            let mut exp = 0u32;
            while {
                *n /= d;
                exp += 1;
                *n % d == 0
            } {}
            *side = (*n as f32).sqrt() as u32;
            *sum *= (d.pow(exp + 1) - 1) / (d - 1);
        }
    }
    fn _sum_of_divisors(&mut self, mut n: u32) -> u32 {
        let mut side = (n as f32).sqrt() as u32;
        let mut sum = 1u32;
        for &p in self._primes.iter() {
            if p > side || n == 1 {
                break;
            }
            self._divide_fully(&mut n, p, &mut side, &mut sum);
        }
        if n != 1 {
            sum *= (n * n - 1) / (n - 1);
            self._primes.push(n);
        }
        sum
    }
    fn pair_sum(&mut self) -> u32 {
        for n in 4..self.under {
            self._aliquot_sum[n as usize] = self._sum_of_divisors(n) - n;
        }
        let mut sum = 0u32;
        for (i, v) in self._aliquot_sum.iter().enumerate() {
            let vsize = *v as usize;
            if vsize >= self._aliquot_sum.len() {
                continue;
            }
            if vsize == i {
                continue;
            }
            if self._aliquot_sum[vsize] == i as u32 {
                sum += *v;
            }
        }
        sum
    }
}

// 1.8 ms
/// ```rust
/// use self::project_euler::m21::sum_of_all_the_amicable_numbers_under_10000_prime_factor_2;
/// assert_eq!(sum_of_all_the_amicable_numbers_under_10000_prime_factor_2(), 31626);
/// ```
pub fn sum_of_all_the_amicable_numbers_under_10000_prime_factor_2() -> u32 {
    let sum = AmicableNumberScanner::new(10_000).pair_sum();
    sum
}

// 1.3 ms
/// ```rust
/// use self::project_euler::m21::sum_of_all_the_amicable_numbers_under_10000_prime_factor_3;
/// assert_eq!(sum_of_all_the_amicable_numbers_under_10000_prime_factor_3(), 31626);
/// ```
pub fn sum_of_all_the_amicable_numbers_under_10000_prime_factor_3() -> u32 {
    struct Index {
        i: usize,
        _ite: Box<dyn Iterator<Item = usize>>,
    }

    impl Index {
        fn increment(&mut self) {
            self.i += self._ite.next().unwrap();
        }
        fn new() -> Self {
            Index {
                i: 5,
                _ite: Box::new(vec![2usize, 4].into_iter().cycle()),
            }
        }
    }

    fn rule_out_square(sieve: &mut Vec<bool>, prime: usize) {
        for i in (prime * prime..sieve.len()).step_by(prime) {
            sieve[i] = false;
        }
    }

    fn primes(under: u32) -> Vec<u32> {
        let mut primes: Vec<u32> = vec![2u32, 3u32];
        let mut sieve = vec![true; under as usize];
        let sqrt = (sieve.len() as f64).sqrt() as usize;
        let mut index = Index::new();
        loop {
            if index.i > sqrt {
                break;
            }
            if sieve[index.i] {
                primes.push(index.i as u32);
                rule_out_square(&mut sieve, index.i);
            }
            index.increment();
        }
        loop {
            if index.i >= sieve.len() {
                break;
            }
            if sieve[index.i] {
                primes.push(index.i as u32);
            }
            index.increment();
        }
        primes
    }

    struct AmicableNumberScanner {
        under: u32,
        _primes: Vec<u32>,
        _checked: Vec<bool>,
    }

    impl AmicableNumberScanner {
        fn new(under: u32) -> Self {
            AmicableNumberScanner {
                under: under,
                _primes: primes(under),
                _checked: vec![false; under as usize],
            }
        }
        fn _divide_fully(&self, n: &mut u32, d: u32, side: &mut u32, sum: &mut u32) {
            if *n % d == 0 {
                let mut exp = 0u32;
                while {
                    *n /= d;
                    exp += 1;
                    *n % d == 0
                } {}
                *side = (*n as f32).sqrt() as u32;
                *sum *= (d.pow(exp + 1) - 1) / (d - 1);
            }
        }
        fn _sum_of_divisors(&mut self, mut n: u32) -> u32 {
            let mut side = (n as f32).sqrt() as u32;
            let mut sum = 1u32;
            for &p in self._primes.iter() {
                if p > side || n == 1 {
                    break;
                }
                self._divide_fully(&mut n, p, &mut side, &mut sum);
            }
            if n != 1 {
                sum *= (n * n - 1) / (n - 1);
            }
            sum
        }
        fn pair_sum(&mut self) -> u32 {
            let mut pair_sum = 0u32;
            for a in 4..self.under {
                if self._checked[a as usize] {
                    continue;
                }
                let sum = self._sum_of_divisors(a) - a;
                if sum <= a {
                    continue;
                }
                if sum >= self.under {
                    continue;
                }
                let b = self._sum_of_divisors(sum) - sum;
                self._checked[sum as usize] = true;
                if a == b {
                    pair_sum += a + sum;
                }
            }
            pair_sum
        }
    }

    let sum = AmicableNumberScanner::new(10_000).pair_sum();
    sum
}
