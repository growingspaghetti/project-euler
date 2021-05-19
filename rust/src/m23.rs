//! See [m21](./m21.rs)

///
/// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
///
/// A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
///
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
///
/// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
///
///
///
/// ```rust
/// use self::project_euler::m23::sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers;
/// assert_eq!(sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers(), 4179871);
/// ```
pub fn sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers() -> u64 {
    fn proper_divisors_sigma_one(num: u64) -> u64 {
        if num < 2 {
            panic!()
        }
        crate::m21::divisor_function_sigma_one_function(num) - num
    }

    let mut abundant_numbers: Vec<u64> = vec![];
    for a in 2..28123u64 {
        let d = proper_divisors_sigma_one(a);
        if d > a {
            abundant_numbers.push(a);
        }
    }

    let check_array = &mut [false; 28123];
    abundant_numbers
        .iter()
        .for_each(|&d| check_array[d as usize] = true);

    let mut sum = 0u64;
    fn is_sum_of_two_abundant_numbers(
        n: u64,
        abundant_numbers: &[u64],
        check_array: &[bool],
    ) -> bool {
        for &a in abundant_numbers {
            // n = sum(a,b) || sum(b,a)
            // when a reached the half point of n, the other half of n is already pair searched.
            if n / 2 < a {
                break;
            }
            let b = n - a;
            if check_array[b as usize] {
                return true;
            }
        }
        false
    }
    for i in 1..28123u64 {
        if !is_sum_of_two_abundant_numbers(i, &abundant_numbers, check_array) {
            sum += i;
        }
    }
    sum
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

fn rule_out_square(sieve: &mut Vec<bool>, prime: usize) {
    for i in (prime * prime..sieve.len()).step_by(prime) {
        sieve[i] = false;
    }
}

fn primes(under: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![2u32, 3u32];
    let mut sieve = vec![true; under as usize];
    let sqrt = (sieve.len() as f64).sqrt() as usize;
    let mut index = Index::new();
    loop {
        if index.i > sqrt {
            break;
        }
        if sieve[index.i] {
            primes.push(index.i as u32);
            rule_out_square(&mut sieve, index.i);
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

struct AbundantNumberScanner {
    under: u32,
    _primes: Vec<u32>,
    _pair_sieve: Vec<bool>,
}

impl AbundantNumberScanner {
    fn new(under: u32) -> Self {
        AbundantNumberScanner {
            under: under,
            _primes: primes(under),
            _pair_sieve: vec![false; under as usize],
        }
    }
    fn _divide_fully(&self, n: &mut u32, d: u32, side: &mut u32, sum: &mut u32) {
        if *n % d == 0 {
            let mut exp = 0u32;
            while {
                *n /= d;
                exp += 1;
                *n % d == 0
            } {}
            *side = (*n as f32).sqrt() as u32;
            *sum *= (d.pow(exp + 1) - 1) / (d - 1);
        }
    }
    fn _sum_of_divisors(&mut self, mut n: u32) -> u32 {
        let mut side = (n as f32).sqrt() as u32;
        let mut sum = 1u32;
        for &p in self._primes.iter() {
            if p > side || n == 1 {
                break;
            }
            self._divide_fully(&mut n, p, &mut side, &mut sum);
        }
        if n != 1 {
            sum *= (n * n - 1) / (n - 1);
        }
        sum
    }
    fn init_abundant_num_pair_sieve(&mut self) {
        let mut abundant_numbers = vec![];
        for n in 12..self.under {
            let sum = self._sum_of_divisors(n) - n;
            if sum > n {
                abundant_numbers.push(n);
            }
        }
        for (i, &a) in abundant_numbers.iter().enumerate() {
            for &b in abundant_numbers[i..].iter() {
                if let Some(n) = self._pair_sieve.get_mut((a + b) as usize) {
                    *n = true;
                }
            }
        }
    }
    fn non_pair_sum(&mut self) -> u32 {
        let mut non_pair_sum = 0u32;
        for n in 1..self.under {
            if !self._pair_sieve[n as usize] {
                non_pair_sum += n;
            }
        }
        non_pair_sum
    }
}

/// ```rust
/// use self::project_euler::m23::sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2;
/// assert_eq!(sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2(), 4179871);
/// ```
pub fn sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers_2() -> u32 {
    let mut a = AbundantNumberScanner::new(28_124);
    a.init_abundant_num_pair_sieve();
    a.non_pair_sum()
}
