//!```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- difference_between_the_sum_of_the_squares --verbose --baseline new
//!     Finished bench [optimized] target(s) in 0.04s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking difference_between_the_sum_of_the_squares
//! Benchmarking difference_between_the_sum_of_the_squares: Warming up for 3.0000 s
//! Benchmarking difference_between_the_sum_of_the_squares: Collecting 100 samples in estimated 5.0000 s (932033050 iterations)
//! Benchmarking difference_between_the_sum_of_the_squares: Analyzing
//! difference_between_the_sum_of_the_squares
//!                         time:   [5.5059 ns 5.5783 ns 5.6627 ns]
//!                         change: [-1.6132% +0.0000% +1.6657%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 14 outliers among 100 measurements (14.00%)
//!   5 (5.00%) high mild
//!   9 (9.00%) high severe
//! slope  [5.5059 ns 5.6627 ns] R^2            [0.5889933 0.5853011]
//! mean   [5.5156 ns 5.6455 ns] std. dev.      [261.63 ps 386.99 ps]
//! median [5.4516 ns 5.4970 ns] med. abs. dev. [142.02 ps 224.85 ps]
//!
//! Benchmarking difference_between_the_sum_of_the_squares_square_pyramidal_number
//! Benchmarking difference_between_the_sum_of_the_squares_square_pyramidal_number: Warming up for 3.0000 s
//! Benchmarking difference_between_the_sum_of_the_squares_square_pyramidal_number: Collecting 100 samples in estimated 5.0000 s (1268065100 iterations)
//! Benchmarking difference_between_the_sum_of_the_squares_square_pyramidal_number: Analyzing
//! difference_between_the_sum_of_the_squares_square_pyramidal_number
//!                         time:   [3.9528 ns 4.0562 ns 4.1694 ns]
//!                         change: [-3.6205% +0.0000% +3.6170%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 11 outliers among 100 measurements (11.00%)
//!   7 (7.00%) high mild
//!   4 (4.00%) high severe
//! slope  [3.9528 ns 4.1694 ns] R^2            [0.2419630 0.2403256]
//! mean   [3.9754 ns 4.1820 ns] std. dev.      [341.41 ps 689.39 ps]
//! median [3.8469 ns 3.9374 ns] med. abs. dev. [130.94 ps 283.44 ps]
//! ```
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/75139653ff382d459e6c26b8f943c21f7457b62a)

/// The sum of the squares of the first ten natural numbers is,
/// 1^2+2^2+...+10^2=385
///
/// The square of the sum of the first ten natural numbers is,
/// (1+2+...+10)^2=552=3025
///
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025−385=2640
///
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
///
/// ```rust
/// use self::project_euler::m6::difference_between_the_sum_of_the_squares;
/// assert_eq!(difference_between_the_sum_of_the_squares(100), 25164150);
/// ```
pub fn difference_between_the_sum_of_the_squares(n: u64) -> u64 {
    let square_of_sum: u64 = (1..=n).sum::<u64>().pow(2);
    let sum_of_squares: u64 = (1..=n).map(|x| x.pow(2)).sum();
    if square_of_sum > sum_of_squares {
        square_of_sum - sum_of_squares
    } else {
        sum_of_squares - square_of_sum
    }
}

struct Sequence {
    n: u64,
}

impl Sequence {
    fn sum_of_squares(&self) -> u64 {
        self.n * (self.n + 1) * (2 * self.n + 1) / 6
    }
    fn sum(&self) -> u64 {
        (1 + self.n) * self.n / 2
    }
}

/// ```rust
/// use self::project_euler::m6::difference_between_the_sum_of_the_squares_square_pyramidal_number_struct;
/// assert_eq!(difference_between_the_sum_of_the_squares_square_pyramidal_number_struct(100), 25164150);
/// ```
pub fn difference_between_the_sum_of_the_squares_square_pyramidal_number_struct(n: u64) -> u64 {
    let s = Sequence{n:n};
    let sum = s.sum();
     sum * sum - s.sum_of_squares()
}


/// The sum of the squares of the first ten natural numbers is,
/// 1^2+2^2+...+10^2=385
///
/// The square of the sum of the first ten natural numbers is,
/// (1+2+...+10)^2=552=3025
///
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025−385=2640
///
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
///
/// ```rust
/// use self::project_euler::m6::difference_between_the_sum_of_the_squares_square_pyramidal_number;
/// assert_eq!(difference_between_the_sum_of_the_squares_square_pyramidal_number(100), 25164150);
/// ```
pub fn difference_between_the_sum_of_the_squares_square_pyramidal_number(n: u64) -> u64 {
    // [0 1 2 3 .. 100] init = 0, last = 100, n = 101
    let square_of_sum: u64 = (n * (n + 1) / 2u64).pow(2);
    // https://en.wikipedia.org/wiki/Summation#Powers_and_logarithm_of_arithmetic_progressions
    // https://en.wikipedia.org/wiki/Square_pyramidal_number
    // [1^2 2^2 3^2 4^2 .. 100^2]
    // n * (n+1) * (2n+1) / 6 -> 100 * 101 * 201 / 6
    let sum_of_squares: u64 = n * (n + 1) * (2 * n + 1) / 6;
    if square_of_sum > sum_of_squares {
        square_of_sum - sum_of_squares
    } else {
        sum_of_squares - square_of_sum
    }
}
