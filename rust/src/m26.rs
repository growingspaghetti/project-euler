/// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
///
/// 1/2    =     0.5
/// 1/3    =     0.(3)
/// 1/4    =     0.25
/// 1/5    =     0.2
/// 1/6    =     0.1(6)
/// 1/7    =     0.(142857)
/// 1/8    =     0.125
/// 1/9    =     0.(1)
/// 1/10    =     0.1
///
/// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
///
/// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
///
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle;
/// assert_eq!(the_longest_recurring_cycle(), (4967, 4966));//(983, 982));
/// ```
pub fn the_longest_recurring_cycle() -> (u32, u32) {
    let mut number_having_longest_recurring_cycle = 0u32;
    let mut longest_recurring_cycle = 0u32;
    // apparently 1.000 can be evenly devided by 2a and 5a
    for divisor in (1..5000u32).step_by(2) {
        if divisor % 5 == 0 {
            continue;
        }
        let mut dividend = 1;
        let mut dividends: Vec<u32> = vec![];
        loop {
            dividend %= divisor;
            if dividend == 0 {
                break;
            }
            if dividends.contains(&dividend) {
                let pos = dividends.iter().position(|&v| v == dividend).unwrap();
                let recurring_cycle = (dividends.len() - pos) as u32;
                if recurring_cycle > longest_recurring_cycle {
                    number_having_longest_recurring_cycle = divisor;
                    longest_recurring_cycle = recurring_cycle;
                }
                //println!("{} {:?}", divisor, dividends);
                break;
            }
            dividends.push(dividend);
            dividend *= 10;
        }
    }
    (
        number_having_longest_recurring_cycle,
        longest_recurring_cycle,
    )
}

struct RecurringCycle {
    n: u32,
    _ite: Box<dyn Iterator<Item = usize>>,
}

impl RecurringCycle {
    fn new() -> Self {
        RecurringCycle {
            n: 1,
            _ite: Box::new(vec![2, 4, 2, 2].into_iter().cycle()),
        }
    }
    fn increment(&mut self) {
        self.n += self._ite.next().unwrap() as u32;
    }
    fn recurring_cycle_length(&mut self) -> u32 {
        let mut dividend = 1u32;
        let mut history: Vec<u32> = vec![];
        let mut term = 0u32;
        loop {
            term += 1;
            let residue = dividend % self.n;
            if residue == 0 {
                return 0;
            }
            for (i, &v) in history.iter().enumerate() {
                if v == residue {
                    return term; //(history.len() - i) as u32;
                }
            }
            history.push(residue);
            dividend = residue * 10;
        }
    }
}

// impl RecurringCycle {
//     fn new() -> Self {
//         RecurringCycle {
//             n: 1,
//             _ite: Box::new(vec![2, 4, 2, 2].into_iter().cycle()),
//         }
//     }
//     fn increment(&mut self) {
//         self.n += self._ite.next().unwrap() as u32;
//     }
//     fn recurring_cycle_length(&mut self) -> u32 {
//         let mut dividend = 1u32;
//         let mut history: Vec<u32> = vec![];
//         loop {
//             let residue = dividend % self.n;
//             if residue == 0 {
//                 return 0;
//             }
//             for (i, &v) in history.iter().enumerate() {
//                 if v == residue {
//                     return (history.len() - i) as u32;
//                 }
//             }
//             history.push(residue);
//             dividend = residue * 10;
//         }
//     }
// }

// // 10.3 ms
// /// ```rust
// /// use self::project_euler::m26::the_longest_recurring_cycle_2;
// /// assert_eq!(the_longest_recurring_cycle_2(5000), 4967);
// /// assert_eq!(the_longest_recurring_cycle_2(1000), 983);
// /// ```
// pub fn the_longest_recurring_cycle_2(below: u32) -> u32 {
//     let mut record_len = 0u32;
//     let mut record_num = 0u32;
//     let mut r = RecurringCycle::new();
//     while {
//         r.increment();
//         r.n < below
//     } {
//         let len = r.recurring_cycle_length();
//         if record_len < len {
//             record_num = r.n;
//             record_len = len;
//         }
//     }
//     record_num
// }

fn recurring_cycle_length(n: u32) -> u32 {
    let mut dividend = 1u32;
    let mut history: Vec<u32> = vec![];
    let mut term = 0u32;
    loop {
        term += 1;
        let residue = dividend % n;
        if residue == 0 {
            return 0;
        }
        for (i, &v) in history.iter().enumerate() {
            if v == residue {
                return term; //(history.len() - i) as u32;
            }
        }
        history.push(residue);
        dividend = residue * 10;
    }
}

// 817 us
// https://oeis.org/A051626
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle_3;
/// assert_eq!(the_longest_recurring_cycle_3(5000), 4967);
/// assert_eq!(the_longest_recurring_cycle_3(1000), 983);
/// assert_eq!(the_longest_recurring_cycle_3(8), 7);
/// assert_eq!(the_longest_recurring_cycle_3(20), 19);
/// assert_eq!(the_longest_recurring_cycle_3(18), 17);
/// assert_eq!(the_longest_recurring_cycle_3(25), 23);
/// ```
pub fn the_longest_recurring_cycle_3(until: u32) -> u32 {
    let blank_array = vec![0u32; until as usize];
    let mut record_len = 0u32;
    let mut record_num = 0u32;

    let mut reminders = vec![0u32; until as usize];
    for n in 1u32..until {
        reminders[..n as usize].copy_from_slice(&blank_array[..n as usize]);
        let mut dividend = 1u32;
        for i in 0u32.. {
            let residue = dividend % n;
            if residue == 0 {
                break;
            }
            let history = reminders[residue as usize];
            if history != 0 {
                let rec = i - history;
                if rec > record_len {
                    record_len = rec;
                    record_num = n;
                }
                break;
            }
            reminders[residue as usize] = i;
            dividend = residue * 10;
        }
    }
    record_num
}

struct UnitFraction {
    reciprocal: u32,
    repetend_length: u32,
}

fn is_recurring(mut n: u32) -> bool {
    if n % 2 != 0 && n % 5 != 0 {
        return true;
    }
    for &d in [2u32, 5].iter() {
        while n % d == 0 {
            n /= d;
        }
        if n == 1 {
            return false;
        }
    }
    true
}

fn repetend_length(n: u32, residue_history: &mut [u32]) -> u32 {
    assert!(residue_history.len() >= n as usize);
    let mut dividend = 1u32;
    for nth_time_around in 0u32.. {
        let residue = dividend % n;
        let last_time = residue_history[residue as usize];
        if last_time != 0 {
            return nth_time_around - last_time;
        }
        residue_history[residue as usize] = nth_time_around;
        dividend = residue * 10;
    }
    panic!("irrational number")
}

// 30 us
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle_reptend;
/// assert_eq!(the_longest_recurring_cycle_reptend(10000), 9967);
/// assert_eq!(the_longest_recurring_cycle_reptend(9968), 9967);
/// assert_eq!(the_longest_recurring_cycle_reptend(5000), 4967); // 4999
/// assert_eq!(the_longest_recurring_cycle_reptend(1000), 983);
/// assert_eq!(the_longest_recurring_cycle_reptend(8), 7);
/// assert_eq!(the_longest_recurring_cycle_reptend(20), 19);
/// assert_eq!(the_longest_recurring_cycle_reptend(18), 17);
/// assert_eq!(the_longest_recurring_cycle_reptend(25), 23);
/// assert_eq!(the_longest_recurring_cycle_reptend(6), 3);
/// ```
pub fn the_longest_recurring_cycle_reptend(below: u32) -> u32 {
    assert!(below > 3);
    let mut uf = UnitFraction {
        reciprocal: 1,
        repetend_length: 0,
    };
    let blank = vec![0u32; below as usize];
    let mut residue_history = vec![0u32; below as usize];
    for n in (1u32..below).rev() {
        if !is_recurring(n) {
            continue;
        }
        residue_history[..n as usize].copy_from_slice(&blank[..n as usize]);
        let length = repetend_length(n, &mut residue_history[0..n as usize]);
        if n - 1 == length {
            return n;
        }
        if length > uf.repetend_length {
            uf.repetend_length = length;
            uf.reciprocal = n;
        }
    }
    uf.reciprocal
}

fn repetend_length_one(n: u32) -> u32 {
    assert!(n % 2 != 0 && n % 5 != 0);
    let mut dividend = 10u32;
    for nth_time_around in 1u32.. {
        let residue = dividend % n;
        if residue == 1 {
            return nth_time_around;
        }
        // if residue == 0 {
        //     return 1;
        // }
        dividend = residue * 10;
    }
    panic!("irrational number")
}

// 28.5 us
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle_reptend_gcd;
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(10000), 9967);
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(9968), 9967);
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(5000), 4967); // 4999
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(1000), 983);
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(8), 7);
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(20), 19);
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(18), 17);
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(25), 23);
/// assert_eq!(the_longest_recurring_cycle_reptend_gcd(6), 3);
/// ```
pub fn the_longest_recurring_cycle_reptend_gcd(below: u32) -> u32 {
    assert!(below > 3);
    let mut uf = UnitFraction {
        reciprocal: 1,
        repetend_length: 0,
    };
    for n in (1u32..below).rev() {
        let mut n_dash = n.clone();
        for &d in [2u32, 5].iter() {
            while n_dash % d == 0 {
                n_dash /= d;
            }
        }
        if n_dash == 1 {
            continue;
        }
        let length = repetend_length_one(n_dash);
        if n - 1 == length {
            return n;
        }
        if length > uf.repetend_length {
            uf.repetend_length = length;
            uf.reciprocal = n;
        }
    }
    uf.reciprocal
}

// 30 us
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle_incl_7;
/// assert_eq!(the_longest_recurring_cycle_incl_7(10000), 9967);
/// assert_eq!(the_longest_recurring_cycle_incl_7(9968), 9967);
/// assert_eq!(the_longest_recurring_cycle_incl_7(1000), 983);
/// assert_eq!(the_longest_recurring_cycle_incl_7(8), 7);
/// assert_eq!(the_longest_recurring_cycle_incl_7(20), 19);
/// assert_eq!(the_longest_recurring_cycle_incl_7(18), 17);
/// assert_eq!(the_longest_recurring_cycle_incl_7(25), 23);
/// ```
pub fn the_longest_recurring_cycle_incl_7(below: u32) -> u32 {
    assert!(below > 7);
    let blank = vec![0u32; below as usize];
    let mut residue_history = vec![0u32; below as usize];
    for n in (1u32..below).rev() {
        if !is_recurring(n) {
            continue;
        }
        residue_history[..n as usize].copy_from_slice(&blank[..n as usize]);
        let length = repetend_length(n, &mut residue_history[0..n as usize]);
        if n - 1 == length {
            return n;
        }
    }
    panic!("couldn't find a point that n - 1 == recurring_length(n)")
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

// 19.5 us
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle_prime;
/// assert_eq!(the_longest_recurring_cycle_prime(10000), 9967);
/// assert_eq!(the_longest_recurring_cycle_prime(9968), 9967);
/// assert_eq!(the_longest_recurring_cycle_prime(1000), 983);
/// assert_eq!(the_longest_recurring_cycle_prime(5000), 4967);
/// assert_eq!(the_longest_recurring_cycle_prime(8), 7);
/// assert_eq!(the_longest_recurring_cycle_prime(20), 19);
/// assert_eq!(the_longest_recurring_cycle_prime(18), 17);
/// assert_eq!(the_longest_recurring_cycle_prime(25), 23);
/// assert_eq!(the_longest_recurring_cycle_prime(6), 3);
/// ```
pub fn the_longest_recurring_cycle_prime(below: u32) -> u32 {
    if below < 7 {
        return 3;
    }
    let primes = primes(below);
    for &p in primes.iter().rev() {
        let l = repetend_length_one(p);
        if l == p - 1 {
            return p;
        }
    }
    panic!("couldn't find a point that n - 1 == recurring_length(n)")
}

// 6.6 us
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle_prime_square;
/// assert_eq!(the_longest_recurring_cycle_prime_square(10000), 9967);
/// assert_eq!(the_longest_recurring_cycle_prime_square(9968), 9967);
/// assert_eq!(the_longest_recurring_cycle_prime_square(1000), 983);
/// assert_eq!(the_longest_recurring_cycle_prime_square(5000), 4967);
/// assert_eq!(the_longest_recurring_cycle_prime_square(8), 7);
/// assert_eq!(the_longest_recurring_cycle_prime_square(20), 19);
/// assert_eq!(the_longest_recurring_cycle_prime_square(18), 17);
/// assert_eq!(the_longest_recurring_cycle_prime_square(25), 23);
/// assert_eq!(the_longest_recurring_cycle_prime_square(6), 3);
/// ```
pub fn the_longest_recurring_cycle_prime_square(below: u32) -> u32 {
    if below < 7 {
        return 3;
    }
    let primes = primes(below);
    for &p in primes.iter().rev() {
        // https://en.wikipedia.org/wiki/Quadratic_reciprocity
        match p % 40 {
            1 | 3 | 9 | 13 | 27 | 31 | 37 | 39 => (),
            _ => return p,
        }
    }
    panic!("couldn't find a point that n - 1 == recurring_length(n)")
}

fn modpow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

// fn mod_pow(mut a: u32, mut exp: u32, m: u32) -> u32 {
//     if m == 1 {
//         return 0;
//     }
//     let mut result = 1;
//     a %= m; // a^1
//     while exp > 0 {
//         if exp % 2 == 1 { // a^1 a^3 a^5..
//             result *= a; // flush to result
//             result %= m;
//         }
//         exp >>= 1;
//         a *= a; // accumulate a^2, a^4, a^8, a^16
//         a %= m;
//     }
//     result
// }


fn mod_pow(mut a: u32, mut exp: u32, m: u32) -> u32 {
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
            result *= a;
            result %= m;
        }
        exp >>= 1;
        if exp == 0 {
            break;
        }
        a *= a;
        a %= m;
    }
    result
}

// 9.3 us
fn mod_pow_rec(a: u32, exp: u32, m: u32) -> u32 {
    if exp == 0 {
        return 1;
    }
    if exp == 1 {
        return a % m;
    }
    if exp % 2 == 1 {
        return (a * mod_pow_rec(a, exp - 1, m)) % m;
    }
    let t = mod_pow_rec(a, exp / 2, m);
    t * t % m
}

fn list_divisors(n: u32) -> Vec<u32> {
    let side = (n as f32).sqrt() as u32;
    let mut vec = vec![];
    for d in 1..=side {
        if n % d == 0 {
            vec.push(d);
            if d != side {
                vec.push(n / d);
            }
        }
    }
    vec.sort();
    vec
}

// 8.8 us
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle_prime_mod_pow;
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(10000), 9967);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(9968), 9967);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(1000), 983);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(5000), 4967);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(8), 7);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(20), 19);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(18), 17);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(25), 23);
/// assert_eq!(the_longest_recurring_cycle_prime_mod_pow(6), 3);
/// ```
pub fn the_longest_recurring_cycle_prime_mod_pow(below: u32) -> u32 {
    if below < 7 {
        return 3;
    }
    let primes = primes(below);
    'next_prime: for &p in primes.iter().rev() {
        let divisors = list_divisors(p - 1);
        //println!("{} {:?}", p, divisors);
        if let Some((_, foregoing_divisors)) = divisors.split_last() {
            for &d in foregoing_divisors {
                if mod_pow(10, d, p) == 1 {
                    continue 'next_prime;
                }
            }
            return p;
        }
    }
    panic!("couldn't find a point that n - 1 == recurring_length(n)")
}
