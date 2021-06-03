// Consecutive prime sum

//   [Show HTML problem content]
// Problem 50

// The prime 41, can be written as the sum of six consecutive primes:
// 41 = 2 + 3 + 5 + 7 + 11 + 13

// This is the longest sum of consecutive primes that adds to a prime below one-hundred.

// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.

// Which prime, below one-million, can be written as the sum of the most consecutive primes?
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

fn fill_sum_up_to_million(primes: &[u32]) -> u32 {
    let mut sum = 0u32;
    for &p in primes {
        sum += p;
        if sum > 1_000_000 {
            sum -= p;
            return sum;
        }
    }
    panic!("The prime list was not enough to fill up the sum to be 1 million!");
}

// 3.9 ms with sieve entirely
/// ```rust
/// use self::project_euler::m50::consecutive_prime_one_million;
/// assert_eq!(consecutive_prime_one_million(), 997651);
/// ```
pub fn consecutive_prime_one_million() -> u32 {
    let sieve = Sieve::new(1_000_000);
    let primes = sieve.primes(1_000_000);
    let mut sum = fill_sum_up_to_million(&primes);
    //println!("{}", prime_to_overflow);3943
    //primes.retain(|&p| p < prime_to_overflow);
    //primes.reverse();
    for p in primes {
        sum -= p;
        if sieve.is_prime(sum) {
            break;
        }
    }
    // while !sieve.is_prime(sum) {
    //     let p = primes.pop().expect("No prime left in the prime list!");
    //     sum -= p;
    // }
    sum
}

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

fn rule_out(sieve: &mut Vec<bool>, prime: usize) {
    for i in (prime * prime..sieve.len()).step_by(prime) {
        sieve[i] = false;
    }
}

fn rule_out_from_previous_position(sieve: &mut Vec<bool>, prime: usize, pp: usize) {
    for i in (prime * prime..sieve.len()).step_by(prime) {
        if i < pp {
            continue;
        }
        sieve[i] = false;
    }
}

fn extend(sieve: &mut Vec<bool>, primes: &Vec<usize>) {
    let previous_len = sieve.len();
    sieve.extend(vec![true; previous_len]);
    for &p in primes {
        rule_out_from_previous_position(sieve, p, previous_len);
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
    result as u32
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    assert!(b != 0);
    let r = a % b;
    if r == 0 {
        return b;
    }
    gcd(b, r)
}

fn fermat_test(n: u32) -> bool {
    gcd(223092870, n) == 1 && mod_pow(223092870, n - 1, n) == 1
}

// if n % 2 == 0 || n % 97 == 0  { // || n % 5 == 0 || n % 7 == 0
//     return false;
// }
// for &a in [97].iter() {
//     if mod_pow(a, n -1, n) != 1 {
//         return false;
//     }
// }
// for &a in [13, 17].iter() {
//     if mod_pow(a, n - 1, n) != 1 {
//         return false;
//     }
// }

// 16.8 us is_prime
// 15.2 fermat_test
// 14.8 without any test
/// ```rust
/// use self::project_euler::m50::consective_prime_million_fermat;
/// assert_eq!(consective_prime_million_fermat(), 997651);
/// ```
pub fn consective_prime_million_fermat() -> u32 {
    let mut sum = 2u32 + 3;
    let mut sieve = vec![true; 1000];
    let mut primes: Vec<usize> = vec![];
    let mut cursor = 5usize;
    'sum_fill: loop {
        let ite = [2usize, 4].iter().cycle();
        for &i in ite {
            if cursor >= sieve.len() {
                break;
            }
            if !sieve[cursor] {
                cursor += i;
                continue;
            }
            &primes.push(cursor);
            rule_out(&mut sieve, cursor);
            sum += cursor as u32;
            if sum >= 1_000_000 {
                sum -= cursor as u32;
                break 'sum_fill;
            }
            cursor += i;
        }
        extend(&mut sieve, &primes);
    }
    primes.insert(0, 2);
    primes.insert(1, 3);
    for p in primes {
        sum -= p as u32;
        if fermat_test(sum) {
            break;
        }
        /*&&  is_prime(sum)  */
        // if sum == 997651 {
        //     break;
        // }
    }
    sum
}
