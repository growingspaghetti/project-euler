//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_even_fibonacci_sequence --verbose --baseline new
//!     Finished bench [optimized] target(s) in 0.05s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000: Warming up for 3.0000 s
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000: Collecting 100 samples in estimated 5.0000 s (108670950 iterations)
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000: Analyzing
//! sum_of_even_fibonacci_sequence_less_than_4000000
//!                         time:   [45.654 ns 45.805 ns 45.969 ns]
//!                         change: [-0.8993% +0.0000% +0.8491%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 6 outliers among 100 measurements (6.00%)
//!   4 (4.00%) high mild
//!   2 (2.00%) high severe
//! slope  [45.654 ns 45.969 ns] R^2            [0.9686866 0.9684106]
//! mean   [45.949 ns 46.519 ns] std. dev.      [897.27 ps 1.9823 ns]
//! median [45.580 ns 45.898 ns] med. abs. dev. [449.50 ps 893.22 ps]
//!
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8: Warming up for 3.0000 s
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8: Collecting 100 samples in estimated 5.0000 s (153454350 iterations)
//! Benchmarking sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8: Analyzing
//! sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8
//!                         time:   [32.412 ns 32.516 ns 32.630 ns]
//!                         change: [-1.4989% +0.0000% +1.3904%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 8 outliers among 100 measurements (8.00%)
//!   6 (6.00%) high mild
//!   2 (2.00%) high severe
//! slope  [32.412 ns 32.630 ns] R^2            [0.9685742 0.9682990]
//! mean   [32.646 ns 33.314 ns] std. dev.      [743.10 ps 2.5098 ns]
//! median [32.300 ns 32.583 ns] med. abs. dev. [303.09 ps 648.22 ps]
//! ```

/// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
///
///   1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
///
/// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
///
/// ```rust
/// use self::project_euler::m2::sum_of_even_fibonacci_sequence_less_than_4000000;
/// assert_eq!(sum_of_even_fibonacci_sequence_less_than_4000000(), 4613732);
/// ```
pub fn sum_of_even_fibonacci_sequence_less_than_4000000() -> i64 {
    let fib = |prepre: i64, pre: i64| -> (i64, i64) {
        match (prepre, pre) {
            (0, 0) => (0, 1),
            // (0, 1) => (1, 1),
            // (1, 1) => (1, 2),
            // (1, 2) => (2, 3),
            // (2, 3) => (3, 5),
            // (3, 5) => (5, 8),
            _ => (pre, prepre + pre),
        }
    };

    let mut sum = 0;
    let mut prepre = 0i64;
    let mut pre = 0i64;
    while pre < 4_000_000 {
        if pre % 2 == 0 {
            sum += pre;
        }
        {
            let tuple = fib(prepre, pre);
            prepre = tuple.0;
            pre = tuple.1;
        }
        //println!("num {}", pre);
    }
    sum
}

/// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
///
///   (0, 1,)1,| 2, 3, 5,| 8, 13, 21,| 34, 55, 89, ...
///
/// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
///
/// ```rust
/// use self::project_euler::m2::sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8;
/// assert_eq!(sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8(), 4613732);
/// ```
pub fn sum_of_even_fibonacci_sequence_less_than_4000000_011_235_8() -> i64 {
    let fib = |preprepre: i64, prepre: i64, pre: i64| -> (i64, i64, i64) {
        match (preprepre, prepre, pre) {
            (0, 1, 1) => (2, 3, 5),
            // (2,  3,  5) => ( 8, 13, 21),
            // (8, 13, 21) => (34, 55, 89),
            _ => {
                let post = prepre + pre;
                let postpost = pre + post;
                let postpostpost = post + postpost;
                (post, postpost, postpostpost)
            }
        }
    };

    let mut sum = 0;
    let mut preprepre = 0i64;
    let mut prepre = 1i64;
    let mut pre = 1i64;
    while preprepre < 4_000_000 {
        if preprepre % 2 == 0 {
            sum += preprepre;
        }
        {
            let tuple = fib(preprepre, prepre, pre);
            preprepre = tuple.0;
            prepre = tuple.1;
            pre = tuple.2
        }
        //println!("num {}", preprepre);
    }
    sum
}