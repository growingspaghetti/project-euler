// The first two consecutive numbers to have two distinct prime factors are:

// 14 = 2 × 7
// 15 = 3 × 5

// The first three consecutive numbers to have three distinct prime factors are:

// 644 = 2² × 7 × 23
// 645 = 3 × 5 × 43
// 646 = 2 × 17 × 19.

// Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?

struct Index {
    i: usize,
    _ite: Box<dyn Iterator<Item = usize>>,
}

impl Index {
    fn increment(&mut self) {
        self.i += self._ite.next().unwrap();
    }
    fn new() -> Self {
        Self {
            i: 5,
            _ite: Box::new(vec![2usize, 4].into_iter().cycle()),
        }
    }
}

struct Sieve {
    _sieve: Vec<bool>,
    _count: Vec<u8>,
    _primes: Vec<usize>,
    _index: Index,
    _cursor: usize,
}

impl Sieve {
    fn rule_out(&mut self, prime: usize) {
        for i in (prime..self._sieve.len()).step_by(prime) {
            self._sieve[i] = false;
            self._count[i] += 1;
        }
    }
    fn clean_sieve(&mut self) {
        while self._index.i < self._sieve.len() {
            if self._sieve[self._index.i] {
                self._primes.push(self._index.i);
                self.rule_out(self._index.i);
            }
            self._index.increment();
        }
    }
    fn new(below: u32) -> Self {
        assert!(below > 4);
        let sieve = vec![true; below as usize];
        let count = vec![0u8; below as usize];
        let mut s = Self {
            _sieve: sieve,
            _count: count,
            _primes: vec![2, 3],
            _index: Index::new(),
            _cursor: 1 + 2 * 3 * 5 * 7,
        };
        s.rule_out(2);
        s.rule_out(3);
        s.clean_sieve();
        s
    }
    fn extend(&mut self) {
        let prev_len = self._sieve.len();
        self._sieve.extend(vec![true; self._sieve.len()]);
        self._count.extend(vec![0u8; self._count.len()]);
        let primes = self._primes.clone();
        for &p in primes.iter() {
            for i in (p..self._sieve.len()).step_by(p) {
                if prev_len > i {
                    continue;
                }
                self._sieve[i] = false;
                self._count[i] += 1;
            }
        }
        self.clean_sieve();
    }
    fn first_of_consective_numbers_with_four_distinct_primes(&mut self) -> Option<u32> {
        for i in self._cursor..self._count.len() {
            if self._count[i] != 4 {
                continue;
            }
            if self._count.len() - 1 < i + 1 || self._count[i + 1] != 4 {
                continue;
            }
            if self._count.len() - 1 < i + 2 || self._count[i + 2] != 4 {
                continue;
            }
            if self._count.len() - 1 < i + 3 || self._count[i + 3] != 4 {
                continue;
            }
            return Some(i as u32);
        }
        self._cursor = self._count.len() - 3;
        None
    }
}

// 148 us
/// ```rust
/// use self::project_euler::m47::distinct_prime_factors_four_consecutive_integers;
/// assert_eq!(distinct_prime_factors_four_consecutive_integers(), 134043);
/// ```
pub fn distinct_prime_factors_four_consecutive_integers() -> u32 {
    // let mut count = vec![0u8; 150_000];
    // let mut sieve = vec![true; 150_000];
    // sieve[0] = false;
    // sieve[1] = false;
    // for i in 2..sieve.len() {
    //     if !sieve[i] {
    //         continue;
    //     }
    //     for j in (i..count.len()).step_by(i) {
    //         sieve[j] = false;
    //     }
    //     for j in (i..count.len()).step_by(i) {
    //         count[j] += 1;
    //     }
    // }
    // println!(
    //     "{} {} {} {} {}",
    //     &count[134043],
    //     &count[134043 + 1],
    //     &count[134043 + 2],
    //     &count[134043 + 3],
    //     &count[134043 + 4]
    // );

    let mut sieve = Sieve::new(10_000);
    loop {
        if let Some(n) = sieve.first_of_consective_numbers_with_four_distinct_primes() {
            break n;
        }
        sieve.extend();
    }
}
