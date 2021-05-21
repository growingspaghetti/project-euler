//!```txt
//!ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- of_primes_for_consecutive_values_of_n --verbose
//!   Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!    Finished bench [optimized] target(s) in 15.27s
//!     Running target/release/deps/bench_main-6ef44f4b2c4834b9
//!Benchmarking of_primes_for_consecutive_values_of_n_brute
//!Benchmarking of_primes_for_consecutive_values_of_n_brute: Warming up for 3.0000 s
//!
//!Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.4s, or reduce sample count to 50.
//!Benchmarking of_primes_for_consecutive_values_of_n_brute: Collecting 100 samples in estimated 9.3623 s (100 iterations)
//!Benchmarking of_primes_for_consecutive_values_of_n_brute: Analyzing
//!of_primes_for_consecutive_values_of_n_brute
//!                        time:   [92.937 ms 93.446 ms 93.949 ms]
//!mean   [92.937 ms 93.949 ms] std. dev.      [2.2889 ms 2.8450 ms]
//!median [92.864 ms 93.939 ms] med. abs. dev. [2.4862 ms 3.5572 ms]
//!
//!Benchmarking of_primes_for_consecutive_values_of_n_eratosthenes
//!Benchmarking of_primes_for_consecutive_values_of_n_eratosthenes: Warming up for 3.0000 s
//!Benchmarking of_primes_for_consecutive_values_of_n_eratosthenes: Collecting 100 samples in estimated 6.4774 s (300 iterations)
//!Benchmarking of_primes_for_consecutive_values_of_n_eratosthenes: Analyzing
//!of_primes_for_consecutive_values_of_n_eratosthenes
//!                        time:   [21.357 ms 21.558 ms 21.776 ms]
//!Found 1 outliers among 100 measurements (1.00%)
//!  1 (1.00%) high severe
//!mean   [21.357 ms 21.776 ms] std. dev.      [804.99 us 1.3956 ms]
//!median [21.234 ms 21.657 ms] med. abs. dev. [779.35 us 1.3006 ms]
//!
//!Benchmarking of_primes_for_consecutive_values_of_n_optimization
//!Benchmarking of_primes_for_consecutive_values_of_n_optimization: Warming up for 3.0000 s
//!Benchmarking of_primes_for_consecutive_values_of_n_optimization: Collecting 100 samples in estimated 5.7226 s (700 iterations)
//!Benchmarking of_primes_for_consecutive_values_of_n_optimization: Analyzing
//!of_primes_for_consecutive_values_of_n_optimization
//!                        time:   [8.0035 ms 8.0667 ms 8.1336 ms]
//!Found 1 outliers among 100 measurements (1.00%)
//!  1 (1.00%) high mild
//!mean   [8.0035 ms 8.1336 ms] std. dev.      [279.51 us 381.17 us]
//!median [7.8594 ms 8.1573 ms] med. abs. dev. [175.93 us 448.42 us]
//!```
//! See [m12](./m12.rs) [m10](./m10.rs)

// 93
/// Euler discovered the remarkable quadratic formula:
///
/// n^2 + n + 41
///
/// It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39. However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
/// The incredible formula n^2 - 79n + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.
/// Considering quadratics of the form:
///
/// n^2 + an + b, where |a| < 1000 and |b| <= 1000 where |n| is the modulus/absolute value of n. e.g. |11| = 11 and |-4| = 4
///
/// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.
///
/// ```rust
/// use self::project_euler::m27::of_primes_for_consecutive_values_of_n_brute;
/// assert_eq!(of_primes_for_consecutive_values_of_n_brute(), (-59231, 71));
/// ```
pub fn of_primes_for_consecutive_values_of_n_brute() -> (i32, u32) {
    let quadratic_formula = |a: i32, b: i32, n: u32| {
        let ni = n as i32;
        ni * ni + a * ni + b
    };
    fn is_prime(i: u64) -> bool {
        if i == 1 {
            return false;
        } else if i == 2 || i == 3 {
            return true;
        } else if i % 2 == 0 {
            return false;
        };
        let mut skip_four = false;
        let mut d = 5u64;
        let side_max = (i as f64).sqrt().ceil() as u64;
        loop {
            if d > side_max {
                break true;
            }
            if i % d == 0 {
                break false;
            }
            if skip_four {
                d += 4;
                skip_four = false;
            } else {
                d += 2;
                skip_four = true;
            }
        }
    }

    let mut max_n = 0u32;
    let mut coefficient_a_at_max_n = 0i32;
    let mut coefficient_b_at_max_n = 0i32;
    for a in -999..=999 {
        for b in -1000..=1000 {
            // Question is saying that in consecutive integer values 0 <= n <= x it's always a prime
            //   n  = [0,     1,     2,     3,     4,     5     ..... x,     x+1      ]
            // f(n) = [prime, prime, prime, prime, prime, prime ..... prime, not prime]
            let mut n = 0u32;
            loop {
                let i = quadratic_formula(a, b, n);
                if i < 0 || !is_prime(i as u64) {
                    break;
                }
                n += 1;
            }
            if n > max_n {
                max_n = n;
                coefficient_a_at_max_n = a;
                coefficient_b_at_max_n = b;
            }
        }
    }
    (coefficient_a_at_max_n * coefficient_b_at_max_n, max_n)
}

// 21
/// Euler discovered the remarkable quadratic formula:
///
/// n^2 + n + 41
///
/// It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39. However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
/// The incredible formula n^2 - 79n + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.
/// Considering quadratics of the form:
///
/// n^2 + an + b, where |a| < 1000 and |b| <= 1000 where |n| is the modulus/absolute value of n. e.g. |11| = 11 and |-4| = 4
///
/// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.
///
/// ```rust
/// use self::project_euler::m27::of_primes_for_consecutive_values_of_n_eratosthenes;
/// assert_eq!(of_primes_for_consecutive_values_of_n_eratosthenes(), (-59231, 71));
/// ```
pub fn of_primes_for_consecutive_values_of_n_eratosthenes() -> (i32, u32) {
    let quadratic_formula = |a: i32, b: i32, n: u32| {
        let ni = n as i32;
        ni * ni + a * ni + b
    };
    // The possible maximum value of f(a,b,n) is produced when f(n,1000,99) = n*n + 999n + 1000,
    // because member a, b appear only in addition.
    //
    // Concerning the maximum value of the formula f(n,1000,99),
    // when f(1000,1000,99), certainly it's already stopped to keep producing a prime.
    // I mean, 1000*1000 + 999*1000 + 1000 = 1000*1000+1000*1000, a compound.
    //
    // For it has stopped producing a prime, values greater than 1000*1000+1000*1000,
    // which is 2000000, can be ruled out from consideration. when f(n, a, b) produces a prime,
    // it never reaches to that number.
    fn build_prime_array_of_2_000_000() -> [bool; 2_000_000] {
        let mut matrix = [true; 2_000_000];
        let mut index = 2usize;
        matrix[0] = false;
        matrix[1] = false;
        matrix[4] = false;
        for i in (index * index..matrix.len()).step_by(index) {
            matrix[i] = false;
        }
        index += 1;
        for i in (index * index..matrix.len()).step_by(index) {
            matrix[i] = false;
        }
        index += 2;
        let mut skip_four = false;
        let side_max = 2_000_000f64.sqrt().floor() as usize;
        loop {
            if index > side_max {
                break;
            }
            if matrix[index] {
                for i in (index * index..matrix.len()).step_by(index) {
                    matrix[i] = false;
                }
            }
            if skip_four {
                index += 4;
                skip_four = false;
            } else {
                index += 2;
                skip_four = true;
            }
        }
        matrix
    }

    let mut max_n = 0u32;
    let mut coefficient_a_at_max_n = 0i32;
    let mut coefficient_b_at_max_n = 0i32;
    let check_matrix = build_prime_array_of_2_000_000();
    for a in -999..=999 {
        for b in -1000..=1000 {
            // Question is saying that in consecutive integer values 0 <= n <= x it's always a prime
            //   n  = [0,     1,     2,     3,     4,     5     ..... x,     x+1      ]
            // f(n) = [prime, prime, prime, prime, prime, prime ..... prime, not prime]
            let mut n = 0u32;
            loop {
                let i = quadratic_formula(a, b, n);
                if i < 0 {
                    break;
                }
                if !check_matrix[i as usize] {
                    break;
                }
                n += 1;
            }
            if n > max_n {
                max_n = n;
                coefficient_a_at_max_n = a;
                coefficient_b_at_max_n = b;
            }
        }
    }
    (coefficient_a_at_max_n * coefficient_b_at_max_n, max_n)
}

//
/// Euler discovered the remarkable quadratic formula:
///
/// n^2 + n + 41
///
/// It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39. However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
/// The incredible formula n^2 - 79n + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.
/// Considering quadratics of the form:
///
/// n^2 + an + b, where |a| < 1000 and |b| <= 1000 where |n| is the modulus/absolute value of n. e.g. |11| = 11 and |-4| = 4
///
/// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.
///
/// ```rust
/// use self::project_euler::m27::of_primes_for_consecutive_values_of_n_optimization;
/// assert_eq!(of_primes_for_consecutive_values_of_n_optimization(), (-59231, 71));
/// ```
pub fn of_primes_for_consecutive_values_of_n_optimization() -> (i32, u32) {
    let quadratic_formula = |a: i32, b: i32, n: u32| {
        let ni = n as i32;
        ni * ni + a * ni + b
    };
    // The possible maximum value of f(a,b,n) is produced when f(n,1000,999) = n*n + 999n + 1000,
    // because member a, b appear only in addition.
    //
    // Concerning the maximum value of the formula f(n,1000,999),
    // when f(1000,1000,99), certainly it's already stopped to keep producing a prime.
    // I mean, 1000*1000 + 999*1000 + 1000 = 1000*1000+1000*1000, a compound.
    //
    // For it has stopped producing a prime, values greater than 1000*1000+1000*1000,
    // which is 2000000, can be ruled out from consideration. when f(n, a, b) produces a prime,
    // it never reaches to that number.
    fn build_prime_array_of_2_000_000() -> [bool; 2_000_000] {
        let mut matrix = [true; 2_000_000];
        let mut index = 2usize;
        matrix[0] = false;
        matrix[1] = false;
        matrix[4] = false;
        for i in (index * index..matrix.len()).step_by(index) {
            matrix[i] = false;
        }
        index += 1;
        for i in (index * index..matrix.len()).step_by(index) {
            matrix[i] = false;
        }
        index += 2;
        let mut skip_four = false;
        let side_max = 2_000_000f64.sqrt().floor() as usize;
        loop {
            if index > side_max {
                break;
            }
            if matrix[index] {
                for i in (index * index..matrix.len()).step_by(index) {
                    matrix[i] = false;
                }
            }
            if skip_four {
                index += 4;
                skip_four = false;
            } else {
                index += 2;
                skip_four = true;
            }
        }
        matrix
    }

    let mut max_n = 0u32;
    let mut coefficient_a_at_max_n = 0i32;
    let mut coefficient_b_at_max_n = 0i32;
    let check_matrix = build_prime_array_of_2_000_000();

    // It's written "consecutive values of n, starting with n = 0",
    // f(n, a, b) = n^2 + an + b, so
    // f(0, a, b) = 0^2 + a0 + b and it's a prime
    // f(1, a, b) = 1^2 + a1 + b and it's a prime
    // f(2, a, b) = 2^2 + a2 + b and it's a prime .... continues consecutively
    //
    // 1.
    // For f(0, a, b) = 0^2 + a0 + b,
    //                = b
    // and it's a prime.
    //
    // 2.
    // For f(1, a, b) = 1^2 + a1 + b
    //                = 1 + a + b
    // and it's a prime, one of [2, 3, 5, 9 ..]
    // It means 1 + a + b > 1
    // a > -b
    for (b, &is_b_prime) in check_matrix.iter().enumerate() {
        if !is_b_prime {
            continue;
        }
        if b > 1000 {
            break;
        }
        for a in (-(b as isize) + 1)..=999 {
            // Question is saying that in consecutive integer values 0 <= n <= x it's always a prime
            //   n  = [0,     1,     2,     3,     4,     5     ..... x,     x+1      ]
            // f(n) = [prime, prime, prime, prime, prime, prime ..... prime, not prime]
            let mut n = 0u32;
            loop {
                let i = quadratic_formula(a as i32, b as i32, n);
                if i < 0 {
                    break;
                }
                if !check_matrix[i as usize] {
                    break;
                }
                n += 1;
            }
            if n > max_n {
                max_n = n;
                coefficient_a_at_max_n = a as i32;
                coefficient_b_at_max_n = b as i32;
            }
        }
    }
    (coefficient_a_at_max_n * coefficient_b_at_max_n, max_n)
}

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

fn quadratic_formula(n: u32, a: i32, b: u32) -> i32 {
    (n * n) as i32 + a * n as i32 + b as i32
}

struct Record {
    a: i32,
    b: u32,
    number_of_consective_primes: u32,
}

impl Record {
    fn merge(&mut self, a: i32, b: u32, number_of_consective_primes: u32) {
        if number_of_consective_primes > self.number_of_consective_primes {
            self.number_of_consective_primes = number_of_consective_primes;
            self.a = a;
            self.b = b;
        }
    }
    fn product_of_coefficient(&self) -> i32 {
        self.a * self.b as i32
    }
}

fn add(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

// fn foo(b: u32) -> impl FnOnce(i32) -> impl FnOnce(u32) -> i32 {
//     |b| |c| |d| {
//         // "actual" fn body
//         unimplemented!()
//     }
// }

fn build_quadratic_function(b: u32) -> impl Fn(u32, i32) -> i32 {
    move |n: u32, a: i32| (n * n) as i32 + a * n as i32 + b as i32
}

// struct QuadraticFunction {
//     a: i32,
//     b: u32,
//     sieve
// }

// impl QuadraticFunction {
//     fn compute(&self, n: u32) -> i32 {
//         (n * n) as i32 + self.a * n as i32 + self.b as i32
//     }
//     fn scan_n(&self) {

//     }
// }

// 7.5 ms
/// ```rust
/// use self::project_euler::m27::of_primes_for_consecutive_values_of_n_optimization_2;
/// assert_eq!(of_primes_for_consecutive_values_of_n_optimization_2(), (-59231, 71));
/// ```
pub fn of_primes_for_consecutive_values_of_n_optimization_2() -> (i32, u32) {
    let (mut nmax, mut amax, mut bmax) = (1u32, 0i32, 0u32);
    let sieve = Sieve::new(2_000_000);
    for b in sieve.primes(1001) {
        for a in (-(b as i32) + 1)..=999 {
            let mut n = 1;
            let mut v = quadratic_formula(1, a, b);
            if !(v > 1 && sieve.is_prime(v as u32)) {
                continue;
            }
            while {
                n += 1;
                v = quadratic_formula(n, a, b);
                v > 1 && sieve.is_prime(v as u32)
            } {}
            if n > nmax {
                nmax = n;
                amax = a;
                bmax = b;
            }
        }
    }
    (amax * bmax as i32, nmax)
}
// n=0 => f(n,a,b) => b, and b is a prime
