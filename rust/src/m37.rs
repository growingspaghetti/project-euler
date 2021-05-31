// The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 {
        return true;
    }
    for d in &[2u32, 3, 5] {
        if n % *d == 0 {
            return false;
        }
    }
    let side = (n as f32).sqrt() as u32;
    let mut d = 5u32;
    for i in [2u32, 4].iter().cycle() {
        if d > side {
            break;
        }
        d += *i;
        if n % d == 0 {
            return false;
        }
    }
    true
}

struct Sieve {
    _sieve: Vec<bool>,
    _primes: Vec<usize>,
    _index: Index,
    _queue: std::collections::VecDeque<usize>,
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

impl Sieve {
    fn rule_out(&mut self, prime: usize) {
        for i in (prime * prime..self._sieve.len()).step_by(prime) {
            self._sieve[i] = false;
        }
    }
    fn clean_sieve(&mut self) {
        let sqrt = (self._sieve.len() as f64).sqrt() as usize;
        while self._index.i <= sqrt {
            if self._sieve[self._index.i] {
                self._primes.push(self._index.i);
                self._queue.push_back(self._index.i);
                self.rule_out(self._index.i);
            }
            self._index.increment();
        }
        while self._index.i < self._sieve.len() {
            if self._sieve[self._index.i] {
                self._primes.push(self._index.i);
                self._queue.push_back(self._index.i);
            }
            self._index.increment();
        }
    }
    fn new(below: u32) -> Self {
        assert!(below > 4);
        let sieve = vec![true; below as usize];
        let mut s = Sieve {
            _sieve: sieve,
            _primes: vec![],
            _index: Index::new(),
            _queue: std::collections::VecDeque::new(),
        };
        s._queue.push_back(2);
        s._queue.push_back(3);
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
    fn next_prime(&mut self) -> u32 {
        loop {
            if let Some(p) = self._queue.pop_front() {
                return p as u32;
            }
            self.extend();
        }
    }
    fn is_left_trancatable_prime(&mut self, p: u32) -> bool {
        let mut d = 10u32;
        while d < p {
            if !self.is_prime(p % d) {
                return false;
            }
            d *= 10;
        }
        true
    }
    fn is_right_trancatable_prime(&mut self, p: u32) -> bool {
        let mut d = 10u32;
        while d < p {
            if !self.is_prime(p / d) {
                return false;
            }
            d *= 10;
        }
        true
    }
}

fn generate_right_truncatable_maybe_prime_numbers(mut p: u32) -> (u32, u32, u32, u32) {
    p *= 10;
    (p + 1, p + 3, p + 7, p + 9)
}

fn next_right_truncatable_primes(mut p: u32) -> Vec<u32> {
    p *= 10;
    [p + 1, p + 3, p + 7, p + 9]
        .iter()
        .map(|&n| n)
        .filter(|&n| is_prime(n))
        .collect::<Vec<u32>>()
}

fn is_left_trancatable_prime(p: u32) -> bool {
    let mut d = 10u32;
    while d < p {
        if !is_prime(p % d) {
            return false;
        }
        d *= 10;
    }
    true
}

fn is_right_trancatable_prime(p: u32) -> bool {
    let mut d = 10u32;
    while d < p {
        if !is_prime(p / d) {
            return false;
        }
        d *= 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn internal() {
        assert_eq!(false, is_left_trancatable_prime(31));
        assert_eq!(true, is_left_trancatable_prime(313));
    }
    // //#[test]
    // fn sieve() {
    //     let mut sieve = Sieve::new(1000);
    //     assert_eq!(true, sieve.is_prime(31));
    //     assert_eq!(true, sieve.is_prime(313));
    //     assert_eq!(false, sieve.is_prime(748317));
    //     assert_eq!(true, sieve.is_prime(104743));
    //     assert_eq!(false, sieve.is_prime(555555));
    //     assert_eq!(false, sieve.is_prime(555555555));
    // }

    #[test]
    fn test_both_truncatable_primes_sieve() {
        assert_eq!(both_truncatable_primes(), 748317);
    }
}

//fn depth_first_two_sided_prime_search(p: u32, counter: &mut u8, sum: &mut u32) {
fn expand_right_truncatable_prime(p: u32, left_truncatable_prime_sum: &mut u32) {
    // 149.7 us without
    // if *counter == 15 {
    //     return;
    // }
    if is_left_trancatable_prime(p) {
        //*counter += 1;
        *left_truncatable_prime_sum += p;
    }
    // 153 us
    // [p].iter()
    //     .map(|&p| p * 10)
    //     .flat_map(|seed| [1, 3, 7, 9].iter().map(move |&d| seed + d))
    //     .filter(|&n| is_prime(n))
    //     .for_each(|p| grow_right_truncatable_prime_tree(p, sum));
    let (n1, n2, n3, n4) = generate_right_truncatable_maybe_prime_numbers(p);
    for &n in [n1, n2, n3, n4].iter().filter(|&n| is_prime(*n)) {
        expand_right_truncatable_prime(n, left_truncatable_prime_sum);
        // if is_left_trancatable_prime(n) {
        //     *left_truncatable_prime_sum += n;
        // }
    }
    // 146.8 us
    // [n1, n2, n3, n4]
    //     .iter()
    //     .filter(|&c| is_prime(*c))
    //     .for_each(|&p| expand_right_truncatable_prime(p, left_truncatable_prime_sum));

    // 162 us
    // for rtp in next_right_truncatable_primes(p) {
    //     expand_right_truncatable_prime(rtp, left_truncatable_prime_sum);
    // }
    // 152 us
    // next_right_truncatable_primes(p)
    //     .iter()
    //     .for_each(|&p| expand_right_truncatable_prime(p, left_truncatable_prime_sum));
    // 162 us
    // p *= 10;
    // [1, 3, 7, 9].iter().map(|&d| p + d)
    //     .filter(|&n| is_prime(n))
    //     .for_each(|p| grow_right_truncatable_prime_tree(p, sum));
    // 151 us
    // if is_prime(c1) {
    //     depth_first_two_sided_prime_search(c1, counter, sum);
    // }
    // if is_prime(c2) {
    //     depth_first_two_sided_prime_search(c2, counter, sum);
    // }
    // if is_prime(c3) {
    //     depth_first_two_sided_prime_search(c3, counter, sum);
    // }
    // if is_prime(c4) {
    //     depth_first_two_sided_prime_search(c4, counter, sum);
    // }
}

fn expand_right_truncatable_prime_sieve(
    p: u32,
    left_truncatable_prime_sum: &mut u32,
    sieve: &mut Sieve
) {
    if sieve.is_left_trancatable_prime(p) {
        *left_truncatable_prime_sum += p;
    }
    let (n1, n2, n3, n4) = generate_right_truncatable_maybe_prime_numbers(p);
    for &n in [n1, n2, n3, n4].iter() {
        if sieve.is_prime(n) {
            expand_right_truncatable_prime_sieve(n, left_truncatable_prime_sum, sieve);
        }
    }
}

/// ```rust
/// use self::project_euler::m37::both_truncatable_primes;
/// assert_eq!(both_truncatable_primes(), 748317);
/// ```
pub fn both_truncatable_primes() -> u32 {
    let mut left_truncatable_prime_sum = 0u32;
    //let mut counter = 0u8;
    for &p in [2u32, 3, 5, 7].iter() {
        expand_right_truncatable_prime(p, &mut left_truncatable_prime_sum);
    }
    left_truncatable_prime_sum - 2 - 3 - 5 - 7
}


// /// ```rust
// /// use self::project_euler::m37::both_truncatable_primes_sieve_hybrid;
// /// assert_eq!(both_truncatable_primes_sieve_hybrid(), 748317);
// /// ```
// pub fn both_truncatable_primes_sieve_hybrid() -> u32 {
//     let mut left_truncatable_prime_sum = 0u32;
//     let mut sieve = Sieve::new(739391340);
//     for &p in [2u32, 3, 5, 7].iter() {
//         expand_right_truncatable_prime_sieve(p, &mut left_truncatable_prime_sum, &mut sieve);
//     }
//     left_truncatable_prime_sum - 2 - 3 - 5 - 7
// }

// 18 ms
/// ```rust
/// use self::project_euler::m37::both_truncatable_primes_sieve;
/// assert_eq!(both_truncatable_primes_sieve(), 748317);
/// ```
pub fn both_truncatable_primes_sieve() -> u32 {
    let mut sum = 0u32;
    let mut sieve = Sieve::new(10_000);
    let mut count = 0u8;
    while count < 15 {
        let p = sieve.next_prime();
        if !sieve.is_left_trancatable_prime(p) {
            continue;
        }
        if !sieve.is_right_trancatable_prime(p) {
            continue;
        }
        count += 1;
        sum += p;
    }
    sum - 2 - 3 - 5 - 7
}
