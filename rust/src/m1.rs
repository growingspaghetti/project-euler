//!```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_all_the_multiples --verbose --baseline new
//!     Finished bench [optimized] target(s) in 0.05s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000: Collecting 100 samples in estimated 5.0038 s (2646200 iterations)
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000: Analyzing
//! sum_of_all_the_multiples_of_3_or_5_below_1000
//!                         time:   [1.8693 us 1.8744 us 1.8801 us]
//!                         change: [-0.7660% +0.0000% +0.8251%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 6 outliers among 100 measurements (6.00%)
//!   4 (4.00%) high mild
//!   2 (2.00%) high severe
//! slope  [1.8693 us 1.8801 us] R^2            [0.9752253 0.9749719]
//! mean   [1.8797 us 1.9011 us] std. dev.      [34.503 ns 74.068 ns]
//! median [1.8682 us 1.8804 us] med. abs. dev. [20.400 ns 37.183 ns]
//!
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_iter
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_iter: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_iter: Collecting 100 samples in estimated 5.0047 s (2691650 iterations)
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_iter: Analyzing
//! sum_of_all_the_multiples_of_3_or_5_below_1000_iter
//!                         time:   [1.8422 us 1.8487 us 1.8561 us]
//!                         change: [-3.4766% +0.0000% +3.6141%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 10 outliers among 100 measurements (10.00%)
//!   3 (3.00%) high mild
//!   7 (7.00%) high severe
//! slope  [1.8422 us 1.8561 us] R^2            [0.9548138 0.9543650]
//! mean   [1.8723 us 1.9678 us] std. dev.      [126.55 ns 332.48 ns]
//! median [1.8390 us 1.8562 us] med. abs. dev. [21.699 ns 41.590 ns]
//!
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series: Warming up for 3.0000 s
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series: Collecting 100 samples in estimated 5.0000 s (2592316500 iterations)
//! Benchmarking sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series: Analyzing
//! sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series
//!                         time:   [1.9155 ns 1.9213 ns 1.9279 ns]
//!                         change: [-0.7086% +0.0000% +0.6915%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 6 outliers among 100 measurements (6.00%)
//!   5 (5.00%) high mild
//!   1 (1.00%) high severe
//! slope  [1.9155 ns 1.9279 ns] R^2            [0.9699201 0.9695511]
//! mean   [1.9302 ns 1.9493 ns] std. dev.      [38.049 ps 58.768 ps]
//! median [1.9146 ns 1.9348 ns] med. abs. dev. [23.570 ps 44.012 ps]
//!```

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// ```rust
/// use self::project_euler::m1::sum_of_all_the_multiples_of_3_or_5_below_1000;
/// assert_eq!(sum_of_all_the_multiples_of_3_or_5_below_1000(), 233168);
/// ```
pub fn sum_of_all_the_multiples_of_3_or_5_below_1000() -> i32 {
    let mut sum = 0;
    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    sum
}

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// ```rust
/// use self::project_euler::m1::sum_of_all_the_multiples_of_3_or_5_below_1000_iter;
/// assert_eq!(sum_of_all_the_multiples_of_3_or_5_below_1000_iter(), 233168);
/// ```
pub fn sum_of_all_the_multiples_of_3_or_5_below_1000_iter() -> i32 {
    (0..1000).filter(|&x| x % 5 == 0 || x % 3 == 0).sum()
}

struct ArithmeticProgression {
    first: i32,
    last: i32,
    diff: i32,
}

impl ArithmeticProgression {
    fn new(first: i32, less_than: i32, diff: i32) -> ArithmeticProgression {
        ArithmeticProgression {
            first: first,
            diff: diff,
            last: {
                let lt_or_equal = less_than - 1;
                lt_or_equal - (lt_or_equal - first) % diff
            },
        }
    }
    fn arithmetic_series(&self) -> i32 {
        let terms = (self.last - self.first) / self.diff + 1;
        (self.first + self.last) * terms / 2
    }
}

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// ```rust
/// use self::project_euler::m1::sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series;
/// assert_eq!(sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series(), 233168);
/// ```
pub fn sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_seriesa() -> i32 {
    // 0  3  6  9 .. 999 = 3 * [0 1 2 3 ... 333] = 3 * (0+333 #doubling) * 334#n / 2#halving
    // 0  5 10 15 .. 995 = 5 * [0 1 2 3 ... 199] = 5 * (0+199 #doubling) * 200#n / 2#halving
    // 0 15 30 45 .. 999-999%15#last = 15 * [0 1 2 3 ... last/15]
    let arithmetic_seriesa = |common_difference: i32| -> i32 {
        let last = 999 - 999 % common_difference;
        let n = last / common_difference + 1;
        last * n / 2
    };
    fn arithmetic_series(lt: i32, diff: i32) -> i32 {
        let lt_or_equal = lt - 1;
        let last = lt_or_equal - lt_or_equal % diff;
        let step = last / diff + 1;
        last * step / 2
    }
    arithmetic_series(1000, 3) + arithmetic_series(1000, 5) - arithmetic_series(1000, 15)
}

///
/// ```rust
/// use self::project_euler::m1::sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series;
/// assert_eq!(sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series(), 233168);
/// ```
pub fn sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series() -> i32 {
    let three = ArithmeticProgression::new(0, 1000, 3).arithmetic_series();
    let five = ArithmeticProgression::new(0, 1000, 5).arithmetic_series();
    let fifteen = ArithmeticProgression::new(0, 1000, 15).arithmetic_series();
    three + five - fifteen
}
