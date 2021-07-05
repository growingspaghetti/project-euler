// Given a positive integer num, write a function which returns True if num is a perfect square else False.

// Follow up: Do not use any built-in library function such as sqrt.

// 2.8 s
/// ```rust
/// use self::project_euler::l367::is_perfect_square_brute;
/// assert_eq!(is_perfect_square_brute(4), true, "4");
/// assert_eq!(is_perfect_square_brute(8), false, "8");
/// assert_eq!(is_perfect_square_brute(9), true, "9");
/// ```
pub fn is_perfect_square_brute(aa: u32) -> bool {
    //dbg!(aa);
    if aa == 0 || aa == 1 {
        return true;
    }
    for a in 0u64..=aa as u64 / 2 {
        let square = a * a;
        if square == aa as u64 {
            return true;
        }
        if square > aa as u64 {
            return false;
        }
    }
    false
}

pub fn is_perfect_square_brute_bench() {
    for i in 0u32..2_000_000 {
        //dbg!("{}", i);
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_brute(i), "{}", i);
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        //dbg!("{}", i);
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_brute(i), "{}", i);
    }
}

pub fn is_perfect_square_newton(a: u32) -> bool {
    let mut x = a / 2 + 1;
    // while x != 0 && x > a / x {
    //     x = (a / x + x) / 2;
    // }
    while x as u64 * x as u64 > a as u64 {
        x = (a / x + x) / 2;
    }
    x * x == a
}

pub fn is_perfect_square_newton_bench() {
    for i in 0u32..2_000_000 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_newton(i), "{}", i);
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_newton(i), "{}", i);
    }
}

use std::cmp::Ordering;

pub fn is_perfect_square_bisection(a: u32) -> bool {
    let (mut top, mut bottom) = (a.clone(), 0u32);
    while bottom <= top {
        let median = bottom + (top - bottom) / 2;
        match (&(a as u64))
            .partial_cmp(&(median as u64 * median as u64))
            .unwrap()
        {
            Ordering::Less => top = median - 1,
            Ordering::Equal => return true,
            Ordering::Greater => bottom = median + 1,
        }
    }
    false
}

pub fn is_perfect_square_bisection_bench() {
    for i in 0u32..2_000_000 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_bisection(i), "{}", i);
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_bisection(i), "{}", i);
    }
}

fn rule_out(sieve: &mut Vec<bool>, prime: usize) {
    for i in (prime * prime..sieve.len()).step_by(prime) {
        sieve[i] = false;
    }
}

fn primes(below: u32) -> Vec<u32> {
    match below {
        0 | 1 | 2 => return vec![],
        3 => return vec![2],
        _ => (),
    }
    let mut primes: Vec<u32> = vec![2u32, 3u32];
    let mut sieve = vec![true; below as usize];
    let mut index = 5usize;
    let mut ite = [2usize, 4].iter().cycle();
    while index * index < below as usize {
        if sieve[index] {
            primes.push(index as u32);
            rule_out(&mut sieve, index);
        }
        index += ite.next().unwrap();
    }
    while index < sieve.len() {
        if sieve[index] {
            primes.push(index as u32);
        }
        index += ite.next().unwrap();
    }
    primes
}

fn num_of_divisors(mut n: u32) -> u64 {
    let mut count = 1u64;
    for &p in &primes(n + 1) {
        if n % p != 0 {
            continue;
        }
        //dbg!("{} {}", n, p);
        let mut exp = 0u64;
        while {
            n /= p;
            exp += 1;
            n % p == 0
        } {}
        count *= exp + 1;
    }
    count
}

pub fn is_perfect_square_prime(a: u32) -> bool {
    num_of_divisors(a) % 2 == 1
}

pub fn is_perfect_square_prime_bench() {
    // for i in 0u32..20 {
    //     let sqrt = (i as f64).sqrt() as u32;
    //     assert_eq!(sqrt * sqrt == i, is_perfect_square_prime(i), "{}", i);
    // }
    for i in 0u32..2_000_000 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_prime(i), "{}", i);
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(sqrt * sqrt == i, is_perfect_square_prime(i), "{}", i);
    }
}

pub fn is_perfect_square_digit_by_digit(mut a: u32) -> bool {
    let mut x = 0u32;
    let mut bit: u32 = 0b01000000000000000000000000000000;
    while bit > a {
        bit >>= 2;
    }
    while bit != 0 {
        let block = x + bit;
        x >>= 1;
        if a >= block {
            a -= block;
            x += bit;
        }
        bit >>= 2;
    }
    a == 0
}

pub fn is_perfect_square_digit_by_digit_bench() {
    for i in 0u32..2_000_000 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(
            sqrt * sqrt == i,
            is_perfect_square_digit_by_digit(i),
            "{}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(
            sqrt * sqrt == i,
            is_perfect_square_digit_by_digit(i),
            "{}",
            i
        );
    }
}

pub fn is_perfect_square_difference_sequence(a: u32) -> bool {
    let mut sum = 0u32;
    for i in (1..).step_by(2) {
        if sum == a {
            return true;
        }
        sum += i;
        if sum > a {
            break;
        }
    }
    false
}

pub fn is_perfect_square_difference_sequence_bench() {
    // for i in 0u32..1000 {
    //     let sqrt = (i as f64).sqrt() as u32;
    //     assert_eq!(
    //         sqrt * sqrt == i,
    //         is_perfect_square_difference_sequence(i),
    //         "{}",
    //         i
    //     );
    // }
    for i in 0u32..100_000 {
        let sqrt = (i as f64).sqrt() as u32;
        assert_eq!(
            sqrt * sqrt == i,
            is_perfect_square_difference_sequence(i),
            "{}",
            i
        );
    }
    // for i in 2_147_390_000u32..2_147_395_601 {
    //     let sqrt = (i as f64).sqrt() as u32;
    //     assert_eq!(
    //         sqrt * sqrt == i,
    //         is_perfect_square_difference_sequence(i),
    //         "{}",
    //         i
    //     );
    // }
}

mod tests {
    //use super::*;

    #[test]
    fn test_is_perfect_square_brute() {
        super::is_perfect_square_brute_bench();
    }

    #[test]
    fn test_is_perfect_square_newton() {
        super::is_perfect_square_newton_bench();
    }

    #[test]
    fn test_is_perfect_square_bisection() {
        super::is_perfect_square_bisection_bench();
    }

    #[test]
    fn test_is_perfect_square_prime() {
        //super::is_perfect_square_prime_bench();
    }

    #[test]
    fn test_is_perfect_square_digit_by_digit() {
        super::is_perfect_square_digit_by_digit_bench();
    }

    #[test]
    fn test_is_perfect_square_difference_sequence() {
        super::is_perfect_square_difference_sequence_bench();
    }
}
