//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100 --verbose --baseline new
//! warning: function is never used: `bench_the_first_triangle_number_to_have_over_five_hundred_divisors_brute`
//!
//! Finished bench [optimized] target(s) in 0.06s
//! Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute: Warming up for 3.0000 s
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute: Collecting 100 samples in estimated 7.0014 s (300 iterations)
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute: Analyzing
//! distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute
//!                    time:   [23.220 ms 23.308 ms 23.406 ms]
//!                    change: [-0.5484% +0.0000% +0.5579%] (p = 1.00 > 0.05)
//!                    No change in performance detected.
//! Found 13 outliers among 100 measurements (13.00%)
//! 9 (9.00%) high mild
//! 4 (4.00%) high severe
//! mean   [23.220 ms 23.406 ms] std. dev.      [337.29 us 616.44 us]
//! median [23.083 ms 23.164 ms] med. abs. dev. [122.73 us 249.90 us]
//!
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string: Warming up for 3.0000 s
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string: Collecting 100 samples in estimated 5.7543 s (400 iterations)
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string: Analyzing
//! distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string
//!                    time:   [15.327 ms 15.813 ms 16.345 ms]
//!                    change: [-4.6019% +0.0000% +4.7903%] (p = 1.00 > 0.05)
//!                    No change in performance detected.
//! Found 10 outliers among 100 measurements (10.00%)
//! 9 (9.00%) high mild
//! 1 (1.00%) high severe
//! mean   [15.327 ms 16.345 ms] std. dev.      [2.0348 ms 3.0669 ms]
//! median [14.338 ms 14.855 ms] med. abs. dev. [429.51 us 1.1562 ms]
//!
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors: Warming up for 3.0000 s
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors: Collecting 100 samples in estimated 5.0527 s (2400 iterations)
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors: Analyzing
//! distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors
//!                    time:   [2.0836 ms 2.0975 ms 2.1134 ms]
//!                    change: [-1.0003% +0.0000% +1.0350%] (p = 0.99 > 0.05)
//!                    No change in performance detected.
//! Found 11 outliers among 100 measurements (11.00%)
//! 7 (7.00%) high mild
//! 4 (4.00%) high severe
//! mean   [2.0836 ms 2.1134 ms] std. dev.      [49.864 us 100.75 us]
//! median [2.0605 ms 2.0780 ms] med. abs. dev. [17.289 us 38.626 us]
//!
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.8s, enable flat sampling, or reduce sample count to 60.
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort: Collecting 100 samples in estimated 5.8477 s (5050 iterations)
//! Benchmarking distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort: Analyzing
//! distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort
//!                    time:   [1.1459 ms 1.1515 ms 1.1574 ms]
//!                    change: [-1.0250% +0.0000% +1.1051%] (p = 1.00 > 0.05)
//!                    No change in performance detected.
//! Found 6 outliers among 100 measurements (6.00%)
//! 5 (5.00%) high mild
//! 1 (1.00%) high severe
//! slope  [1.1459 ms 1.1574 ms] R^2            [0.9373576 0.9369576]
//! mean   [1.1522 ms 1.1697 ms] std. dev.      [26.277 us 65.162 us]
//! median [1.1386 ms 1.1518 ms] med. abs. dev. [12.371 us 29.249 us]
//! ```
//! See (m16)[m16.rs],See (m20)[20.rs]
//! See (m32)[21.rs]

/// Consider all integer combinations of a^b for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
///
/// 2 ^ 2 = 4, 2 ^ 3 = 8, 2 ^ 4 = 16, 2 ^ 5 = 32
/// 3 ^ 2 = 9, 3 ^ 3 = 27, 3 ^ 4 = 81, 3 ^ 5 = 243
/// 4 ^ 2 = 16, 4 ^ 3 = 64, 4 ^ 4 = 256, 4 ^ 5 = 1024
/// 5 ^ 2 = 25, 5 ^ 3 = 125, 5 ^ 4 = 625, 5 ^ 5 = 3125
///
/// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:
///
/// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
///
/// How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
///
/// ```rust
/// use self::project_euler::m29::distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute;
/// assert_eq!(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute(), 9183);
/// ```
#[allow(clippy::needless_range_loop)]
pub fn distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute() -> u64 {
    fn multiply(containers: &mut std::vec::Vec<u64>, num: u64) {
        let mut carry = 0u64;
        for i in 0..containers.len() {
            containers[i] = containers[i] * num + carry;
            if containers[i] > 1_000_000_000 {
                carry = containers[i] / 1_000_000_000;
                containers[i] -= 1_000_000_000 * carry;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            containers.push(carry);
        }
    }
    fn pow(num: u64, exp: u64) -> String {
        let containers = &mut std::vec::Vec::<u64>::new();
        containers.push(1);
        for _ in 0..exp {
            multiply(containers, num);
        }
        containers
            .iter()
            .rev()
            .map(|&u| format!("{:09}", u))
            .collect::<Vec<String>>()
            .join("")
    }
    fn fab(a: u64, b: u64) -> String {
        pow(a, b)
    }

    use std::collections::HashSet;
    let mut map: HashSet<String> = HashSet::new();
    for a in 2..=100u64 {
        for b in 2..=100u64 {
            map.insert(fab(a, b));
        }
    }
    // ..
    // 000002059862363607268589106591502209383050789423074359962839707804960198700428009033203125
    // 000047101286972462448349216036890000000000000000000000000000000000000000000000000000000000
    // 000000904625697166532776746648320380374280103671755200316906558262375061821325312
    // 000318644812890625
    // ..
    //map.iter().for_each(|s|println!("{}", s));
    map.len() as u64
}

/// Consider all integer combinations of a^b for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
///
/// 2 ^ 2 = 4, 2 ^ 3 = 8, 2 ^ 4 = 16, 2 ^ 5 = 32
/// 3 ^ 2 = 9, 3 ^ 3 = 27, 3 ^ 4 = 81, 3 ^ 5 = 243
/// 4 ^ 2 = 16, 4 ^ 3 = 64, 4 ^ 4 = 256, 4 ^ 5 = 1024
/// 5 ^ 2 = 25, 5 ^ 3 = 125, 5 ^ 4 = 625, 5 ^ 5 = 3125
///
/// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:
///
/// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
///
/// How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
///
/// ```rust
/// use self::project_euler::m29::distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string;
/// assert_eq!(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string(), 9183);
/// ```
#[allow(clippy::needless_range_loop)]
pub fn distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_brute_not_using_string() -> u64 {
    fn multiply(containers: &mut std::vec::Vec<u64>, num: u64) {
        let mut carry = 0u64;
        for i in 0..containers.len() {
            containers[i] = containers[i] * num + carry;
            if containers[i] > 1_000_000_000 {
                carry = containers[i] / 1_000_000_000;
                containers[i] -= 1_000_000_000 * carry;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            containers.push(carry);
        }
    }
    fn pow(num: u64, exp: u64) -> Vec<u64> {
        let mut containers = std::vec::Vec::<u64>::new();
        containers.push(1);
        for _ in 0..exp {
            multiply(&mut containers, num);
        }
        containers
    }
    fn fab(a: u64, b: u64) -> Vec<u64> {
        pow(a, b)
    }

    use std::collections::HashSet;
    let mut map: HashSet<Vec<u64>> = HashSet::new();
    for a in 2..=100u64 {
        for b in 2..=100u64 {
            map.insert(fab(a, b));
        }
    }
    map.len() as u64
}

/// Consider all integer combinations of a^b for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
///
/// 2 ^ 2 = 4, 2 ^ 3 = 8, 2 ^ 4 = 16, 2 ^ 5 = 32
/// 3 ^ 2 = 9, 3 ^ 3 = 27, 3 ^ 4 = 81, 3 ^ 5 = 243
/// 4 ^ 2 = 16, 4 ^ 3 = 64, 4 ^ 4 = 256, 4 ^ 5 = 1024
/// 5 ^ 2 = 25, 5 ^ 3 = 125, 5 ^ 4 = 625, 5 ^ 5 = 3125
///
/// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:
///
/// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
///
/// How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
///
/// ```rust
/// use self::project_euler::m29::distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors;
/// assert_eq!(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors(), 9183);
/// ```
pub fn distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors() -> u64 {
    fn factoring(num: u64) -> Vec<(u64, u64)> {
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

    use std::collections::HashSet;
    let mut map: HashSet<Vec<(u64, u64)>> = HashSet::new();
    for a in 2..=100u64 {
        let prime_factors = factoring(a);
        for b in 2..=100u64 {
            // 6^2 = 6 * 6 = 2*3 * 2*3 = 2^2 * 3^2
            let calculating_formula = prime_factors
                .iter()
                .map(|t| (t.0, t.1 * b))
                .collect::<Vec<(u64, u64)>>();
            map.insert(calculating_formula);
        }
    }
    // ..
    // [(71, 37)]
    // [(2, 156),(11, 78)]
    // [(2, 32),(37, 32)]
    // [(2, 96),(19, 48)]
    // [(2, 198),(7, 66)]
    // [(43, 92)]
    // ..
    // map.iter().for_each(|v|println!("{:?}", v));
    map.len() as u64
}

/// Consider all integer combinations of a^b for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
///
/// 2 ^ 2 = 4, 2 ^ 3 = 8, 2 ^ 4 = 16, 2 ^ 5 = 32
/// 3 ^ 2 = 9, 3 ^ 3 = 27, 3 ^ 4 = 81, 3 ^ 5 = 243
/// 4 ^ 2 = 16, 4 ^ 3 = 64, 4 ^ 4 = 256, 4 ^ 5 = 1024
/// 5 ^ 2 = 25, 5 ^ 3 = 125, 5 ^ 4 = 625, 5 ^ 5 = 3125
///
/// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:
///
/// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
///
/// How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
///
/// ```rust
/// use self::project_euler::m29::distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort;
/// assert_eq!(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort(), 9183);
/// ```
pub fn distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort() -> u64 {
    fn factoring(num: u64) -> Vec<(u64, u64)> {
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

    let mut map: Vec<Vec<(u64, u64)>> = Vec::new();
    for a in 2..=100u64 {
        let prime_factors = factoring(a);
        for b in 2..=100u64 {
            // 6^2 = 6 * 6 = 2*3 * 2*3 = 2^2 * 3^2
            let calculating_formula = prime_factors
                .iter()
                .map(|t| (t.0, t.1 * b))
                .collect::<Vec<(u64, u64)>>();
            map.push(calculating_formula);
        }
    }
    // ..
    // [(71, 37)]
    // [(2, 156),(11, 78)]
    // [(2, 32),(37, 32)]
    // [(2, 96),(19, 48)]
    // [(2, 198),(7, 66)]
    // [(43, 92)]
    // ..
    // map.iter().for_each(|v|println!("{:?}", v));
    map.sort();
    let mut count = 1u64;
    for i in 1..map.len() {
        if map[i - 1] != map[i] {
            count += 1;
        }
    }
    count
}

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

fn rule_out(sieve: &mut Vec<bool>, prime: usize) {
    for i in (prime * prime..sieve.len()).step_by(prime) {
        sieve[i] = false;
    }
}

fn primes(below: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![2u32, 3u32];
    let mut sieve = vec![true; below as usize];
    let sqrt = (sieve.len() as f64).sqrt() as usize;
    let mut index = Index::new();
    loop {
        if index.i > sqrt {
            break;
        }
        if sieve[index.i] {
            primes.push(index.i as u32);
            rule_out(&mut sieve, index.i);
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct Factor {
    prime: u32,
    exp: u32,
}

fn divide_fully(n: &mut u32, d: u32, side: &mut u32, factors: &mut Vec<Factor>) {
    if *n % d != 0 {
        return;
    }
    let mut exp = 0u32;
    while {
        *n /= d;
        exp += 1;
        *n % d == 0
    } {}
    factors.push(Factor { prime: d, exp: exp });
    *side = (*n as f32).sqrt() as u32;
}

fn factorize(mut n: u32, primes: &[u32]) -> Vec<Factor> {
    let mut factors = vec![];
    let mut side = (n as f32).sqrt() as u32;
    for &p in primes.iter() {
        if p > side || n == 1 {
            break;
        }
        divide_fully(&mut n, p, &mut side, &mut factors);
    }
    if n != 1 {
        factors.push(Factor { prime: n, exp: 1 });
    }
    factors
}

fn count_duplication(arr: &mut [Vec<Factor>]) -> u32 {
    arr.sort();
    let mut dup = 0u32;
    for i in 1..arr.len() {
        if arr[i - 1] == arr[i] {
            dup += 1;
        }
    }
    dup
}

// 885 us
/// ```rust
/// use self::project_euler::m29::distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort_2;
/// assert_eq!(distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort_2(), 9183);
/// ```
pub fn distinct_terms_are_in_a_pow_b_for_2_a_100_2_b_100_prime_factors_list_sort_2() -> u32 {
    let primes = primes(101);
    let mut expressions = Vec::new();
    (2..=100u32).map(|a| factorize(a, &primes)).for_each(|a| {
        for b in 2..=100u32 {
            let mut ab = a.to_vec();
            ab.iter_mut().for_each(|f| f.exp *= b);
            expressions.push(ab);
        }
    });
    expressions.len() as u32 - count_duplication(&mut expressions)
}

    // for a in 2..=100u32 {
    //     let factors = factorize(a, &primes);
    //     for b in 2..=100u32 {
    //         let mut ab = factors.to_vec();
    //         ab.iter_mut().for_each(|v| v.exp *= b);
    //         equations.push(ab);
    //     }
    // }