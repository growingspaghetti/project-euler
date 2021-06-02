use core::panic;

// It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.

// 9 = 7 + 2×12
// 15 = 7 + 2×22
// 21 = 3 + 2×32
// 25 = 7 + 2×32
// 27 = 19 + 2×22
// 33 = 31 + 2×12

// It turns out that the conjecture was false.

// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

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

struct Sieve {
    _sieve: Vec<bool>,
    _primes: Vec<usize>,
    _index: Index,
}

impl Sieve {
    fn rule_out(&mut self, prime: usize) {
        for i in (prime * prime..self._sieve.len()).step_by(prime) {
            self._sieve[i] = false;
        }
    }
    fn clean_sieve(&mut self) {
        let sqrt = (self._sieve.len() as f32).sqrt() as usize;
        while self._index.i <= sqrt {
            if self._sieve[self._index.i] {
                self._primes.push(self._index.i);
                self.rule_out(self._index.i);
            }
            self._index.increment();
        }
        while self._index.i < self._sieve.len() {
            if self._sieve[self._index.i] {
                self._primes.push(self._index.i);
            }
            self._index.increment();
        }
    }
    fn new(below: u32) -> Self {
        assert!(below > 4);
        let sieve = vec![true; below as usize];
        let mut s = Self {
            _sieve: sieve,
            _primes: vec![],
            _index: Index::new(),
        };
        s.clean_sieve();
        s
    }
    fn extend(&mut self) {
        self._sieve.extend(vec![true; self._sieve.len()]);
        let primes = self._primes.clone();
        for &p in primes.iter() {
            self.rule_out(p);
        }
        self.clean_sieve();
    }
    fn is_prime(&mut self, n: u32) -> bool {
        if n == 2 || n == 3 {
            return true;
        }
        if n == 0 || n == 1 || n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        while n > self._sieve.len() as u32 - 1 {
            self.extend();
        }
        self._sieve[n as usize]
    }
}

struct DoubleSquares {
    _n: u32,
    _elements: Vec<u32>,
}

impl DoubleSquares {
    fn new() -> Self {
        Self {
            _n: 5,
            _elements: vec![2, 8, 18, 32, 50],
        }
    }
    fn extend(&mut self) {
        for n in self._n + 1..self._n * 2 {
            self._elements.push(n * n * 2);
        }
        self._n *= 2;
    }
    fn double_check(&mut self, sieve: &mut Sieve, o: u32) -> Result<(), ()> {
        while o > self._elements.last().map(|&n| n).unwrap_or(0) {
            self.extend();
        }
        for &d in &self._elements {
            if d >= o {
                return Err(());
            }
            if sieve.is_prime(o - d) {
                return Ok(());
            }
        }
        panic!("This function is supposed to have return but not break in loop!");
    }
}

// 148 us
/// ```rust
/// use self::project_euler::m46::what_is_the_smallest_goldbach_other_conjecture;
/// assert_eq!(what_is_the_smallest_goldbach_other_conjecture(), 5777);
/// ```
pub fn what_is_the_smallest_goldbach_other_conjecture() -> u32 {
    let mut sieve = Sieve::new(1000);
    let mut double_squares = DoubleSquares::new();
    for o in (9..).step_by(2) {
        if sieve.is_prime(o) {
            continue;
        }
        if double_squares.double_check(&mut sieve, o).is_err() {
            return o;
        }
    }
    panic!("This function is supposed to have return but not break in loop!");
}
