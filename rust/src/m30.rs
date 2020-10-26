//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_fifth_powers --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 20.36s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking sum_of_fifth_powers
//! Benchmarking sum_of_fifth_powers: Warming up for 3.0000 s
//! Benchmarking sum_of_fifth_powers: Collecting 100 samples in estimated 6.7555 s (300 iterations)
//! Benchmarking sum_of_fifth_powers: Analyzing
//! sum_of_fifth_powers     time:   [22.278 ms 22.395 ms 22.518 ms]
//!                         change: [-0.7370% +0.0000% +0.7723%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   4 (4.00%) high mild
//! mean   [22.278 ms 22.518 ms] std. dev.      [506.67 us 712.74 us]
//! median [22.102 ms 22.399 ms] med. abs. dev. [388.00 us 696.22 us]
//!
//! Benchmarking sum_of_fifth_powers_precise
//! Benchmarking sum_of_fifth_powers_precise: Warming up for 3.0000 s
//! Benchmarking sum_of_fifth_powers_precise: Collecting 100 samples in estimated 5.1316 s (500 iterations)
//! Benchmarking sum_of_fifth_powers_precise: Analyzing
//! sum_of_fifth_powers_precise
//!                         time:   [10.141 ms 10.193 ms 10.248 ms]
//!                         change: [-0.6990% +0.0000% +0.7711%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//! mean   [10.141 ms 10.248 ms] std. dev.      [232.07 us 313.84 us]
//! median [10.042 ms 10.243 ms] med. abs. dev. [198.64 us 372.03 us]
//! ```

/// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
///
/// 1634 = 1^4 + 6^4 + 3^4 + 4^4
/// 8208 = 8^4 + 2^4 + 0^4 + 8^4
/// 9474 = 9^4 + 4^4 + 7^4 + 4^4
///
/// As 1 = 14 is not a sum it is not included.
///
/// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
///
/// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
///
/// ```rust
/// use self::project_euler::m30::sum_of_fifth_powers;
/// assert_eq!(sum_of_fifth_powers(4), 19316);
/// assert_eq!(sum_of_fifth_powers(5), 443839);
/// ```
pub fn sum_of_fifth_powers(exp: u32) -> u32 {
    //      n |   delta in n | sum of fifth power
    //      9 |           +0 |         9^5 = 59049
    //     99 |          +90 |     9^5+9^5 = above + 59049
    //    999 |         +900 | 9^5+9+5+9^5 = above + 59049
    //   9999          +9000
    //  99999         +90000 > +59049
    // 100000 > 59049
    // 999999        +900000 > +59049
    //1000000
    let max = 10i32.pow((9i32.pow(exp) as f32).log10().ceil() as u32 + 1) as u32;
    let mut total = 0u32;
    for n in 2..max {
        let mut tmp = n;
        let mut sum = 0u32;
        while tmp > 0 {
            sum += (tmp % 10).pow(exp);
            tmp /= 10;
        }
        if n == sum {
            total += sum;
        }
    }
    total
}

/// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
///
/// 1634 = 1^4 + 6^4 + 3^4 + 4^4
/// 8208 = 8^4 + 2^4 + 0^4 + 8^4
/// 9474 = 9^4 + 4^4 + 7^4 + 4^4
///
/// As 1 = 14 is not a sum it is not included.
///
/// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
///
/// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
///
/// ```rust
/// use self::project_euler::m30::sum_of_fifth_powers_precise;
/// assert_eq!(sum_of_fifth_powers_precise(4), 19316);
/// assert_eq!(sum_of_fifth_powers_precise(5), 4438394);
/// ```
pub fn sum_of_fifth_powers_precise(exp: u32) -> u32 {
    //      n |   delta in n | sum of fifth power
    //      9 |           +0 |         9^5 = 59049
    //     99 |          +90 |     9^5+9^5 = above + 59049
    //    999 |         +900 | 9^5+9+5+9^5 = above + 59049
    //   9999          +9000
    //  99999         +90000 > +59049
    // 100000 > 59049
    // 999999        +900000 > +59049
    //1000000
    let mut n = 10u32;
    let mut sum_pow = 9u32.pow(exp);
    while n < sum_pow {
        n *= 10;
        sum_pow += 9u32.pow(exp);
    }
    //       (10  59_049) initial item
    //      10^1       9
    //  (100_000 295_245) previous item
    // 1_000_000 354_294
    //      10^6  999999
    // println!("{} {}", n, sum_pow);
    // sum cannot be greather than 354_294, so when n above 354_294, n == sum always becomes false

    let max = sum_pow;
    let mut total = 0u32;
    for n in 2..=max {
        let mut tmp = n;
        let mut sum = 0u32;
        while tmp > 0 {
            sum += (tmp % 10).pow(exp);
            tmp /= 10;
        }
        if n == sum {
            total += sum;
        }
    }
    total
}
