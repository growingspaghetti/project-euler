// <p>Starting with 1 and spiralling anticlockwise in the following way, a square spiral with side length 7 is formed.</p>
// <p class="center monospace"><span class="red"><b>37</b></span> 36 35 34 33 32 <span class="red"><b>31</b></span><br />
// 38 <span class="red"><b>17</b></span> 16 15 14 <span class="red"><b>13</b></span> 30<br />
// 39 18 <span class="red"> <b>5</b></span>  4 <span class="red"> <b>3</b></span> 12 29<br />
// 40 19  6  1  2 11 28<br />
// 41 20 <span class="red"> <b>7</b></span>  8  9 10 27<br />
// 42 21 22 23 24 25 26<br /><span class="red"><b>43</b></span> 44 45 46 47 48 49</p>
// <p>It is interesting to note that the odd squares lie along the bottom right diagonal, but what is more interesting is that 8 out of the 13 numbers lying along both diagonals are prime; that is, a ratio of 8/13 ≈ 62%.</p>
// <p>If one complete new layer is wrapped around the spiral above, a square spiral with side length 9 will be formed. If this process is continued, what is the side length of the square spiral for which the ratio of primes along both diagonals first falls below 10%?</p>

// mod index {
//     pub struct Index {
//         pub i: usize,
//         f: u8,
//     }

//     impl Index {
//         pub fn increment(&mut self) {
//             self.i += 2 << self.f;
//             self.f ^= 1;
//         }
//         pub fn new() -> Self {
//             Self { i: 5, f: 0 }
//         }
//     }
// }

// mod sieve {
//     pub struct Sieve {
//         beginning: usize,
//         sieve: [bool; 10_000],
//         primes: Vec<usize>,
//         index: super::index::Index,
//     }

//     impl Sieve {
//         fn clean_sieve(&mut self) {
//             let sqrt = ((self.beginning + 9_999) as f32).sqrt() as usize;
//             while self.index.i <= sqrt {
//                 if self.sieve[self.index.i - self.beginning] {
//                     let p = self.index.i;
//                     self.primes.push(p);
//                     let head = (self.beginning / p) * p + p + 1;
//                     ((head - self.beginning)..self.sieve.len())
//                         .step_by(p)
//                         .for_each(|i| self.sieve[i] = false);
//                 }
//                 self.index.increment();
//             }
//             while self.index.i < self.beginning + self.sieve.len() {
//                 if self.sieve[self.index.i - self.beginning] {
//                     self.primes.push(self.index.i);
//                 }
//                 self.index.increment();
//             }
//         }
//         pub fn new(below: u32) -> Self {
//             assert!(below > 4);
//             let mut s = Self {
//                 beginning: 0,
//                 sieve: [true; 10_000],
//                 primes: vec![],
//                 index: super::index::Index::new(),
//             };
//             s.clean_sieve();
//             s
//         }
//         fn next_page(&mut self) {
//             self.beginning += 10_000;
//             for i in 0usize..10_000 {
//                 self.sieve[i] = true;
//             }
//             let primes = self.primes.clone();
//             for &p in primes.iter() {
//                 let head = (self.beginning / p) * p + p + 1;
//                 ((head - self.beginning)..self.sieve.len())
//                     .step_by(p)
//                     .for_each(|i| self.sieve[i] = false);
//             }
//             self.clean_sieve();
//         }
//         pub fn is_prime(&mut self, n: u32) -> bool {
//             if n == 2 || n == 3 {
//                 return true;
//             }
//             if n == 0 || n == 1 || n % 2 == 0 || n % 3 == 0 {
//                 return false;
//             }
//             while n > (self.beginning + self.sieve.len()) as u32 - 1 {
//                 self.next_page();
//             }
//             self.sieve[n as usize - self.beginning]
//         }
//     }
// }

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 {
        return true;
    }
    for d in &[2, 3, 5] {
        if n % *d == 0 {
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

struct Spiral {
    n: u32,
    diagonal_area: u32,
    primes: u32,
}

impl Spiral {
    fn increment(&mut self) {
        self.n += 1;
        self.diagonal_area += 4;
        self.primes += self.prime_corners();
    }
    fn prime_corners(&self) -> u32 {
        let square = (2 * self.n + 1) * (2 * self.n + 1);
        (1u32..=3)
            .map(|i| square - 2 * self.n * i)
            .filter(|&n| is_prime(n))
            .count() as u32
    }
}

// 124 ms
/// ```rust
/// use self::project_euler::m58::spiral_primes_1;
/// assert_eq!(spiral_primes_1(), 26241);
/// ```
pub fn spiral_primes_1() -> u32 {
    // let mut entirety = 1u32;
    // let mut primes = 0u32;
    // //let mut sieve = sieve::Sieve::new(10);
    // for n in 1u32.. {
    //     let mut sq = (2 * n + 1) * (2 * n + 1);
    //     // sq -= 6 * n;
    //     // if sieve.is_prime(sq) {
    //     //     primes += 1;
    //     // }
    //     for _ in 0..3 {
    //         sq -= 2 * n;
    //         if is_prime(sq) {
    //             primes += 1;
    //         }
    //     }
    //     entirety += 4;
    //     //dbg!(n, primes * 10, entirety);
    //     if primes * 10 < entirety {
    //         return 2 * n + 1;
    //     }
    // }
    // unreachable!();
    let mut spiral = Spiral {
        n: 0,
        diagonal_area: 1,
        primes: 0,
    };
    while {
        spiral.increment();
        spiral.primes * 10 >= spiral.diagonal_area
    } {}
    2 * spiral.n + 1
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
        sieve: [bool; 100_001],
        index: super::index::Index,
    }

    impl Sieve {
        pub fn new() -> Self {
            let mut s = Self {
                sieve: [true; 100_001],
                index: super::index::Index::new(),
            };
            s.init();
            s
        }
        pub fn sieve_len(&self) -> u32 {
            self.sieve.len() as u32
        }
        fn init(&mut self) {
            let side = ((self.sieve.len() - 1) as f32).sqrt() as usize;
            while self.index.i <= side {
                if self.sieve[self.index.i] {
                    let p = self.index.i;
                    (p * p..self.sieve.len())
                        .step_by(p)
                        .for_each(|i| self.sieve[i] = false);
                }
                self.index.increment();
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

// 46 ms
/// ```rust
/// use self::project_euler::m58::spiral_primes_2;
/// assert_eq!(spiral_primes_2(), 26241);
/// ```
pub fn spiral_primes_2() -> u32 {
    let sieve = sieve::Sieve::new();

    let mut entirety = 1u32;
    let mut primes = 1u32; //3
                           //let mut sieve = sieve::Sieve::new(10);
    for n in 1u32.. {
        let mut sq = (2 * n + 1) * (2 * n + 1);
        // sq -= 6 * n;
        // if sieve.is_prime(sq) {
        //     primes += 1;
        // }
        for _ in 0..3 {
            sq -= 2 * n;
            if sieve.is_prime(sq) {
                primes += 1;
            }
        }
        entirety += 4;
        //dbg!(n, primes * 10, entirety);
        if primes * 10 < entirety {
            return 2 * n + 1;
        }
    }
    unreachable!();
}

fn mod_pow_64(a: u64, exp: u64, m: u64) -> u64 {
    let (mut a, mut exp, m) = (a as u128, exp as u128, m as u128);
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
    result as u64
}

fn check_composite_64(n: u64, a: u64, d: u64, s: u32) -> bool {
    let mut x = mod_pow_64(a, d, n);
    if x == 1 || x == n - 1 {
        return false;
    }
    for r in 1..s {
        x = x * x % n;
        if x == n - 1 {
            return false;
        }
    }
    true
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

fn check_composite(n: u32, a: u32, d: u32, s: u32) -> bool {
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return false;
    }
    for _ in 1..s {
        x = ((x as u64 * x as u64) % n as u64) as u32;
        if x == n - 1 {
            return false;
        }
    }
    true
}

use core::panic;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::m58::sieve::Sieve;

struct MinStdRand {
    state: u64,
}

impl MinStdRand {
    const M: u64 = 2147483647;
    const A: u64 = 48271;
    const MAX: u64 = 2147483646;
    fn new(seed: u32) -> Self {
        Self { state: seed as u64 }
    }
    fn next(&mut self, partition: u32) -> u32 {
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

// fn minstd_rand(n: u32) -> u32 {
//     ((48271u64 * n as u64) % 2_147_483_647) as u32
// }

// def witness(n, a):
//     t, u = 0, n - 1
//     while u & 1 == 0:
//         t += 1
//         u >>= 1
//     # n-1 = 2^t * u

//     x = pow(a, u, n)
//     # x_0 = a^u mod n

//     for _ in range(t):
//         y = x * x % n
//         # x_{i+1} = (x_i)^2 mod n

//         if y == 1:
//             # (x^2 mod n) = 1

//             if x != 1 and x != n - 1:
//                 return True
//             else:
//                 # 以降全て1なのでOK
//                 return False
//         x = y

//     # x_t = (a^{n-1} mod n) != 1
//     return True

/// finds the k*2^e form of given n 
fn coefficient_and_exponent_of_two(mut n: u32) -> (u32, u32) {
    let mut exp = 0u32;
    while n % 2 == 0 {
        n /= 2;
        exp += 1;
    }
    (n, exp)
}

fn is_probable_prime(n: u32, rand: &mut MinStdRand) -> bool {
    if n == 1 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }

    // let mut s = 0u32;
    // let mut d = n - 1;
    // while d % 2 == 0 {
    //     d /= 2;
    //     s += 1;
    // }
    let (d, s) = coefficient_and_exponent_of_two(n - 1); 
    //let nmm = (n - 1) as u64;
    'NEXT_TRIAL: for _ in 0..3 {
        // 2 <= a < n
        let a = 2 + rand.next(n - 2);
        // if check_composite(n, a, d, s) {
        //     return false;
        // }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'NEXT_TRIAL;
        }
        for _ in 1..s {
            x = (x as u64 * x as u64 % n as u64) as u32;
            if x == n - 1 {
                continue 'NEXT_TRIAL;
            }
        }
        return false;
    }
    true
}

// 29 ms
/// ```rust
/// use self::project_euler::m58::spiral_primes_3;
/// assert_eq!(spiral_primes_3(), 26241);
/// ```
pub fn spiral_primes_3() -> u32 {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let mut rand = MinStdRand::new(seed);

    let sieve = Sieve::new();
    let mut entirety = 1u32;
    let mut primes = 0u32;
    for n in 1u32.. {
        let mut sq = (2 * n + 1) * (2 * n + 1);
        for _ in 0..3 {
            sq -= 2 * n;
            if sq < sieve.sieve_len() {
                if sieve.is_prime(sq) {
                    primes += 1;
                }
                continue;
            }
            if is_probable_prime(sq, &mut rand) {
                primes += 1;
            }
        }
        entirety += 4;
        //dbg!(n, primes * 10, entirety);
        if primes * 10 < entirety {
            return 2 * n + 1;
        }
    }
    unreachable!();
}

mod tests {
    use super::*;

    #[test]
    fn test_spiral_primes_1() {
        assert_eq!(spiral_primes_1(), 26241);
    }
}

// prng
