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
        // [2^2, 3^1, 2^2, 5^1, 3^1*2^1, 7^1, 2^3, 3^2, 5^1*2^1, .. 5^1*2^2]
        let map = prime_map(i);
        for (&p, &e) in &map {
            let &e_parent = prime_and_exp.get(&p).unwrap_or(&0);
            if e_parent < e {
                prime_and_exp.insert(p, e);
            }
        }
    }

    let mut sum = 1u32;
    println!("{:#?}", prime_and_exp);
    for (&p, &e) in &prime_and_exp {
        sum *= p.pow(e.into()) as u32;
    }
    sum
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
    fn greatest_common_factor(a: u64, b: u64) -> u64 {
        if b > a {
            greatest_common_factor(b, a)
        } else if b == 0 {
            a
        } else {
            greatest_common_factor(b, a % b)
        }
    };
    let least_common_multiple = |a: u64, b: u64| -> u64 { a * b / greatest_common_factor(a, b) };

    let mut lcm = 1u64;
    for i in 2..=20u64 {
        lcm = least_common_multiple(lcm, i);
    }
    lcm
}
