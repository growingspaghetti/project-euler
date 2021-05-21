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
/////assert_eq!(sum_of_fifth_powers_precise(4), 19316);
/// assert_eq!(sum_of_fifth_powers_precise(5), 443839);
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
    //println!("{} {}", n, sum_pow);
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
            println!("{} {}", n, sum);
            total += sum;
        }
    }
    total
}

fn match_pow_sum(target: u32, pow_sum_999_fold: &[u32]) -> bool {
    let mut digits = target;
    let mut sum = 0;
    while digits > 0 {
        let d = digits % 1000;
        digits /= 1000;
        if d == 0 {
            continue;
        }
        sum += pow_sum_999_fold[d as usize - 1];
        if sum > target {
            return false;
        }
    }
    sum == target
}

fn pow_sum_999_fold(power_ninefold: &[u32; 9]) -> [u32; 999] {
    let mut pow_sum_999_fold = [0u32; 999];
    for i in 1..pow_sum_999_fold.len() {
        let mut sum = 0;
        let mut digits = i as u32;
        while digits > 0 {
            let d = digits % 10;
            digits /= 10;
            if d == 0 {
                continue;
            }
            sum += power_ninefold[d as usize - 1];
        }
        pow_sum_999_fold[i - 1] = sum;
    }
    pow_sum_999_fold
}

fn digit_range_max(powed_nine: u32) -> u32 {
    let mut digit_min = 1u32;
    let mut pow_sum_max = powed_nine;
    while digit_min < pow_sum_max {
        digit_min *= 10;
        pow_sum_max += powed_nine;
    }
    pow_sum_max - powed_nine
}

// 3.9 ms -> 1.5 ms
/// ```rust
/// use self::project_euler::m30::sum_of_fifth_powers_precise_2;
/// assert_eq!(sum_of_fifth_powers_precise_2(4), 19316);
/// assert_eq!(sum_of_fifth_powers_precise_2(5), 443839);
/// ```
pub fn sum_of_fifth_powers_precise_2(e: u32) -> u32 {
    let mut power_ninefold = [0u32; 9];
    (1..=9u32).for_each(|n| power_ninefold[n as usize - 1] = n.pow(e));
    let pow_sum_999_fold = pow_sum_999_fold(&power_ninefold);
    let digits_max = digit_range_max(power_ninefold[8]);
    (2..=digits_max)
        .filter(|&d| match_pow_sum(d, &pow_sum_999_fold))
        .sum()
}

fn match_exp_sum3(target: u32, power_ninefold: &[u32], exp: u32) -> bool {
    let log = (target as f32).log10() as u32;
    let mut sum = 0;
    for n in 0..=log {
        let ten_pow_n = 10u32.pow(n);
        sum += ((target % (ten_pow_n * 10) - target % ten_pow_n)/ ten_pow_n).pow(exp);
    }
    sum == target
}

// fn match_exp_sum3(target: u32, power_ninefold: &[u32]) -> bool {
//     let mut digits = target;
//     let mut sum = 0;
//     while digits > 0 {
//         let d = digits % 10;
//         digits /= 10;
//         if d != 0 {
//             sum += power_ninefold[d as usize - 1];
//             if sum > target {
//                 return false;
//             }
//         }
//     }
//     sum == target
// }

fn digit_range_max3(powed_nine: u32) -> u32 {
    let mut digit_min = 1u32;
    let mut exp_sum_max = powed_nine;
    while digit_min < exp_sum_max {
        digit_min *= 10;
        exp_sum_max += powed_nine;
    }
    exp_sum_max - powed_nine
}

// 3.9 ms
/// ```rust
/// use self::project_euler::m30::sum_of_fifth_powers_precise_3;
/// assert_eq!(sum_of_fifth_powers_precise_3(4), 19316);
/// assert_eq!(sum_of_fifth_powers_precise_3(5), 443839);
/// ```
pub fn sum_of_fifth_powers_precise_3(e: u32) -> u32 {
    let power_ninefold = (1..=9u32).map(|n| n.pow(e)).collect::<Vec<u32>>();
    let digits_max = digit_range_max3(power_ninefold[8]);
    (2..=digits_max)
        .filter(|&d| match_exp_sum3(d, &power_ninefold, e))
        .sum()
}