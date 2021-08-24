// <p>The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating them in any order the result will always be prime. For example, taking 7 and 109, both 7109 and 1097 are prime. The sum of these four primes, 792, represents the lowest sum for a set of four primes with this property.</p>
// <p>Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.</p>

use std::collections::HashMap;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 {
        return true;
    }
    for &d in &[2, 3, 5] {
        if n % d == 0 {
            return false;
        }
    }
    let side = (n as f32).sqrt() as u32;
    let mut d = 5;
    for &i in [2, 4].iter().cycle() {
        d += i;
        if d > side {
            break;
        }
        if n % d == 0 {
            return false;
        }
    }
    true
}

fn concat(mut a: u32, b: u32) -> u32 {
    let mut t = b.clone();
    while t > 0 {
        a *= 10;
        t /= 10;
    }
    a + b
}

fn is_pair(a: u32, b: u32, sieve: &sieve::Sieve, rand: &mut rand::MinStdRand) -> bool {
    let ab = concat(a, b);
    let is_ab_prime = if ab < sieve.sieve_len() {
        sieve.is_prime(ab)
    } else {
        is_probable_prime(ab, rand)
    };
    if !is_ab_prime {
        return false;
    }
    let ba = concat(b, a);
    if ba < sieve.sieve_len() {
        sieve.is_prime(ba)
    } else {
        is_probable_prime(ba, rand)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_prime_set_1() {
        assert_eq!(prime_set_1(), 26033);
    }
}


use std::time::{SystemTime, UNIX_EPOCH};

// 316 ms
/// ```rust
/// use self::project_euler::m60::prime_set_1;
/// assert_eq!(prime_set_1(), 26033);
/// ```
pub fn prime_set_1() -> u32 {
    let mut primes = vec![];
    let mut sieve = sieve::Sieve::new(1000);
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let mut rand = rand::MinStdRand::new(seed);
    let mut prime_pairs: HashMap<u32, Vec<u32>> = HashMap::new();
    sieve.next_prime(); // discard 2
    loop {
        let mut pairs: Option<Vec<u32>> = None;
        let a = sieve.next_prime();
        for &b in &primes {
            if is_pair(a, b, &sieve, &mut rand) {
                if let Some(vec) = pairs.as_mut() {
                    vec.push(b);
                } else {
                    pairs.insert(vec![b]);
                }
            }
        }
        if let Some(vec) = pairs {
            if vec.len() >= 4 {
                if let Some(ans) = dig(&vec, &prime_pairs, &vec![a], 3) {
                    println!("{:?}", ans);
                    break ans.iter().sum();
                }
                // rec
            }
            //break;
            prime_pairs.insert(a, vec);
        }
        primes.push(a);
    }
}

/// finds the k*2^e form of given n 
fn coefficient_and_exponent_of_two(mut n: u32) -> (u32, u32) {
    let mut exp = 0u32;
    while n % 2 == 0 {
        n /= 2;
        exp += 1;
    }
    (n, exp)
}

fn is_probable_prime(n: u32, rand: &mut rand::MinStdRand) -> bool {
    if n == 1 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let (d, s) = coefficient_and_exponent_of_two(n - 1); 
    'next_trial: for _ in 0..3 {
        let a = 2 + rand.next(n - 2);
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'next_trial;
        }
        for _ in 1..s {
            x = (x as u64 * x as u64 % n as u64) as u32;
            if x == n - 1 {
                continue 'next_trial;
            }
        }
        return false;
    }
    true
}

mod rand {
    pub struct MinStdRand {
        state: u64,
    }

    impl MinStdRand {
        const M: u64 = 2147483647;
        const A: u64 = 48271;
        const MAX: u64 = 2147483646;
        pub fn new(seed: u32) -> Self {
            Self { state: seed as u64 }
        }
        pub fn next(&mut self, partition: u32) -> u32 {
            let p = partition as u64;
            assert!(p > 0 && p <= Self::MAX);
            self.state = Self::A * self.state % Self::M;
            loop {
                let n = self.state * p / Self::MAX;
                if n < p {
                    return n as u32;
                }
            }
        }
    }
}

fn mod_pow(a: u32, exp: u32, m: u32) -> u32 {
    let (mut a, mut exp, m) = (a as u64, exp as u64, m as u64);
    if m == 1 {
        return 0;
    }
    if exp == 0 {
        return 1;
    }
    let mut result = 1;
    a %= m;
    loop {
        if exp % 2 == 1 {
            result = result * a % m;
        }
        exp >>= 1;
        if exp == 0 {
            break;
        }
        a = a * a % m;
    }
    result as u32
}

fn dig(
    candidates: &[u32],
    lookup_table: &HashMap<u32, Vec<u32>>,
    drain: &[u32],
    angles: usize,
) -> Option<Vec<u32>> {
    //println!("{:?} {:?} {}", candidates, drain, angles);
    if angles == 0 {
        let mut a = drain.to_vec();
        a.push(*candidates.get(0).unwrap());
        return Some(a);
    }
    for p in candidates.iter() {
        if let Some(vec) = lookup_table.get(p) {
            if vec.len() >= angles {
                let elements = vec
                    .iter()
                    .filter(|&e| candidates.binary_search(e).is_ok())
                    .map(|e| *e)
                    .collect::<Vec<u32>>();
                let mut next_container = drain.clone().to_vec();
                next_container.push(*p);
                if elements.len() >= angles {
                    let satisfactory_set =
                        dig(&elements, lookup_table, &next_container, angles - 1);
                    if satisfactory_set.is_some() {
                        return satisfactory_set;
                    }
                }
            }
        }
    }
    return None;
}

mod index {
    pub struct Index {
        pub i: usize,
        f: u8,
    }

    impl Index {
        pub fn increment(&mut self) {
            self.i += 2 << self.f;
            self.f ^= 1;
        }
        pub fn new() -> Self {
            Self { i: 5, f: 0 }
        }
    }
}

mod sieve {
    pub struct Sieve {
        sieve: Vec<bool>,
        index: super::index::Index,
        primes: Vec<usize>,
        queue: std::collections::VecDeque<usize>,
    }

    impl Sieve {
        pub fn new(below: u32) -> Self {
            assert!(below > 4);
            let sieve = vec![true; below as usize];
            let mut s = Self {
                sieve: sieve,
                index: super::index::Index::new(),
                primes: vec![],
                queue: std::collections::VecDeque::new(),
            };
            s.queue.push_back(2);
            s.queue.push_back(3);
            s.clean_sieve();
            s
        }
        pub fn sieve_len(&self) -> u32 {
            self.sieve.len() as u32
        }
        fn rule_out(&mut self, prime: usize) {
            for i in (prime * prime..self.sieve.len()).step_by(prime) {
                self.sieve[i] = false;
            }
        }
        fn rule_out_from_previous_position(&mut self, prime: usize, pp: usize) {
            use std::cmp::max;
            let begin = max((((pp - 1) / prime) + 1) * prime, prime * prime);
            for i in (begin..self.sieve.len()).step_by(prime) {
                self.sieve[i] = false;
            }
        }
        fn clean_sieve(&mut self) {
            let side = ((self.sieve.len() - 1) as f32).sqrt() as usize;
            while self.index.i <= side {
                if self.sieve[self.index.i] {
                    self.primes.push(self.index.i);
                    self.queue.push_back(self.index.i);
                    self.rule_out(self.index.i);
                }
                self.index.increment();
            }
            while self.index.i < self.sieve.len() {
                if self.sieve[self.index.i] {
                    self.primes.push(self.index.i);
                    self.queue.push_back(self.index.i);
                }
                self.index.increment();
            }
        }
        fn extend(&mut self) {
            let previous_len = self.sieve.len();
            self.sieve.extend(vec![true; previous_len]);
            for &p in &self.primes.clone() {
                self.rule_out_from_previous_position(p, previous_len);
            }
            self.clean_sieve();
        }
        pub fn next_prime(&mut self) -> u32 {
            loop {
                if let Some(p) = self.queue.pop_front() {
                    return p as u32;
                }
                self.extend();
            }
        }
        pub fn is_prime(&self, n: u32) -> bool {
            assert!(n < self.sieve.len() as u32);
            if n < 2 {
                return false;
            }
            if n % 2 == 0 {
                return n == 2;
            }
            if n % 3 == 0 {
                return n == 3;
            }
            return self.sieve[n as usize];
        }
    }
}
