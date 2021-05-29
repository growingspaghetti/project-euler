// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

// How many circular primes are there below one million?

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
        let mut s = Sieve { _sieve: sieve };
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

fn is_circular_prime(mut p: u32, sieve: &Sieve) -> bool {
    let log10 = (p as f32).log10();
    let exp10 = 10u32.pow(log10 as u32);
    for _ in 0..log10 as u8 {
        let d = p % 10;
        p /= 10;
        p += exp10 * d;
        if !sieve.is_prime(p) {
            return false;
        }
    }
    true
}

/// ```rust
/// use self::project_euler::m35::circular_primes_are_there_below_one_million;
/// assert_eq!(circular_primes_are_there_below_one_million(), 55);
/// ```
pub fn circular_primes_are_there_below_one_million() -> u32 {
    let sieve = Sieve::new(1_000_000);
    sieve
        .primes(1_000_000)
        .iter()
        .filter(|&p| is_circular_prime(*p, &sieve))
        .count() as u32
}
