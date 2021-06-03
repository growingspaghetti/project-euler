// Prime permutations

//   [Show HTML problem content]
// Problem 49

// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.

// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.

// What 12-digit number do you form by concatenating the three terms in this sequence?

struct Sieve {
    _sieve: Vec<bool>,
}

impl Sieve {
    fn rule_out(&mut self, prime: usize) {
        for i in (prime * prime..self._sieve.len()).step_by(prime) {
            self._sieve[i] = false;
        }
    }
    fn init(&mut self) {
        let sqrt = (self._sieve.len() as f64).sqrt() as usize;
        let mut index = 5usize;
        for &i in [2usize, 4].iter().cycle() {
            if index > sqrt {
                break;
            }
            if self._sieve[index] {
                self.rule_out(index);
            }
            index += i;
        }
    }
    fn new(below: u32) -> Self {
        assert!(below > 4);
        let sieve = vec![true; below as usize];
        let mut s = Self { _sieve: sieve };
        s.init();
        s
    }
    fn is_prime(&self, n: u32) -> bool {
        assert!(n < self._sieve.len() as u32);
        if n == 2 || n == 3 {
            return true;
        }
        if n == 0 || n == 1 || n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        self._sieve[n as usize]
    }
    fn primes(&self, below: u32) -> Vec<u32> {
        let mut primes: Vec<u32> = vec![2u32, 3u32];
        let mut index = 5usize;
        for &i in [2usize, 4].iter().cycle() {
            if index > below as usize {
                break;
            }
            if self._sieve[index] {
                primes.push(index as u32);
            }
            index += i;
        }
        primes
    }
}

fn is_permutations(mut a: u32, mut b: u32, mut c: u32) -> bool {
    assert!(a > 999 && b > 999 && c > 999);
    assert!(a < 10000 && b < 10000 && c < 10000);
    for n in [&mut a, &mut b, &mut c].iter_mut() {
        **n *= 10;
        **n += 1;
    }
    let (mut a_bits, mut b_bits, mut c_bits) = (0u16, 0u16, 0u16);
    for (n, bits) in [
        (&mut a, &mut a_bits),
        (&mut b, &mut b_bits),
        (&mut c, &mut c_bits),
    ]
    .iter_mut()
    {
        while **n > 1 {
            let d = **n % 10;
            **n /= 10;
            **bits |= 1 << d;
        }
    }
    a_bits == b_bits && b_bits == c_bits
}

// let mut bits = 0u16;
// for n in [a, b].iter_mut() {
//     while *n > 0 {
//         let d = *n % 10;
//         *n /= 10;
//         bits |= 1 << d;
//     }
// }
// let mut flip = 0u16;
// while c > 0 {
//     let d = c % 10;
//     c /= 10;
//     flip |= 1 << d;
// }
// bits ^ flip == 0 || bits ^ flip == 1

mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(is_permutations(2969, 6299, 9629), true);
        assert_eq!(is_permutations(1230, 3211, 3012), false);
        assert_eq!(is_permutations(1530, 3211, 3012), false);
        assert_eq!(is_permutations(2000, 1111, 2121), false);
    }
}

// 22.1 us
/// ```rust
/// use self::project_euler::m49::four_digit_circular_prime_3330_series;
/// assert_eq!(four_digit_circular_prime_3330_series(), 296962999629);
/// ```
pub fn four_digit_circular_prime_3330_series() -> u64 {
    let sieve = Sieve::new(10_000);
    sieve
        .primes(10_000)
        .iter()
        .filter(|&p| *p > 999 && *p < 10_000 - 6660)
        .filter(|&p| sieve.is_prime(*p + 3330) && sieve.is_prime(*p + 6660))
        .filter(|&p| *p != 1487)
        .filter(|&p| is_permutations(*p, *p + 3330, *p + 6660))
        .map(|&p| p as u64)
        .map(|p| p * 100_000_000 + (p + 3330) * 10_000 + p + 6660)
        .last()
        .expect("The prime series with a difference of 3330 not found!")
}
