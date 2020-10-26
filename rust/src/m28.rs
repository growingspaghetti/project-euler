//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001 --verbose
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 18.14s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec: Warming up for 3.0000 s
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec: Collecting 100 samples in estimated 5.3355 s (1400 iterations)
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec: Analyzing
//! sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec
//!                         time:   [3.7215 ms 3.7760 ms 3.8356 ms]
//! Found 12 outliers among 100 measurements (12.00%)
//!   8 (8.00%) high mild
//!   4 (4.00%) high severe
//! mean   [3.7215 ms 3.8356 ms] std. dev.      [225.13 us 358.45 us]
//! median [3.6091 ms 3.6919 ms] med. abs. dev. [61.616 us 166.99 us]
//!
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box: Warming up for 3.0000 s
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box: Collecting 100 samples in estimated 5.0030 s (671650 iterations)
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box: Analyzing
//! sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box
//!                         time:   [7.3272 us 7.3503 us 7.3768 us]
//! Found 10 outliers among 100 measurements (10.00%)
//!   5 (5.00%) high mild
//!   5 (5.00%) high severe
//! slope  [7.3272 us 7.3768 us] R^2            [0.9674925 0.9670950]
//! mean   [7.3811 us 7.4793 us] std. dev.      [161.14 ns 344.44 ns]
//! median [7.3047 us 7.3509 us] med. abs. dev. [47.086 ns 111.62 ns]
//!
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square: Warming up for 3.0000 s
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square: Collecting 100 samples in estimated 5.0000 s (2242548450 iterations)
//! Benchmarking sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square: Analyzing
//! sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square
//!                         time:   [2.2213 ns 2.2267 ns 2.2328 ns]
//! Found 8 outliers among 100 measurements (8.00%)
//!   5 (5.00%) high mild
//!   3 (3.00%) high severe
//! slope  [2.2213 ns 2.2328 ns] R^2            [0.9836342 0.9834075]
//! mean   [2.2330 ns 2.2541 ns] std. dev.      [36.222 ps 70.075 ps]
//! median [2.2165 ns 2.2276 ns] med. abs. dev. [12.073 ps 27.033 ps]
//! ```

/// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
///
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
///
/// It can be verified that the sum of the numbers on the diagonals is 101.
///
/// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
///
/// ```rust
/// use self::project_euler::m28::sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec;
/// assert_eq!(sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec(), 669171001);
/// ```
pub fn sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_brute_vec() -> u64 {
    let mut matrix = vec![vec![0u32; 1001]; 1001];
    let center = (
        matrix.len() / 2,
        matrix.len() / 2, /* or (matrix.len()-1)/2 + 1 */
    );
    let mut n = 1u32;
    matrix[center.0][center.1] = n;
    let mut xy = center;
    // 1 | → 2, ↓ 3 | ← 4 ← 5, ↑ 6 ↑ 7 | → 8 → 9 → 10, ↓ 11 ↓ 12 ↓ 13 | ← 14 ← 15 ← 16 ← 17, ...., 1001*1001
    let mut step = 1u32;
    'outer: loop {
        // →
        for _ in 0..step {
            n += 1;
            if n > 1001 * 1001 {
                break 'outer;
            }
            xy = (xy.0 + 1, xy.1);
            matrix[xy.0][xy.1] = n;
        }
        // ↓
        for _ in 0..step {
            n += 1;
            if n > 1001 * 1001 {
                break 'outer;
            }
            xy = (xy.0, xy.1 - 1);
            matrix[xy.0][xy.1] = n;
        }
        // ←
        step += 1;
        for _ in 0..step {
            n += 1;
            if n > 1001 * 1001 {
                break 'outer;
            }
            xy = (xy.0 - 1, xy.1);
            matrix[xy.0][xy.1] = n;
        }
        // ↑
        for _ in 0..step {
            n += 1;
            if n > 1001 * 1001 {
                break 'outer;
            }
            xy = (xy.0, xy.1 + 1);
            matrix[xy.0][xy.1] = n;
        }
        step += 1;
    }
    let mut sum = 0u64;
    // ↘
    for (i, v) in matrix.iter().enumerate() {
        sum += v[i] as u64;
    }
    // ↗
    for (i, v) in matrix.iter().rev().enumerate() {
        sum += v[i] as u64;
    }
    // ✕ counted two times
    sum -= matrix[center.0][center.1] as u64;
    sum
}

/// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
///
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
///
/// It can be verified that the sum of the numbers on the diagonals is 101.
///
/// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
///
/// ```rust
/// use self::project_euler::m28::sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box;
/// assert_eq!(sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box(), 669171001);
/// ```
pub fn sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box() -> u64 {
    // It's the square version of https://en.wikipedia.org/wiki/Triangular_number
    //
    // 1 = 1^2
    //
    // 1 2 3
    // 4   5
    // 6 7 8 = 3^2 - 1^1
    //
    // 1  2  3  4  5
    // 6           7
    // 8           9
    //10          11
    //12 13 14 15 16 = 5^2 - 3^2
    //
    // And values counted are four items from the greatest number
    // with the interval of the side length of previous box.
    //
    // 21 22 23 24 25
    // 20          10
    // 19          11
    // 18          12
    // 17 16 15 14 13 = 5^2 - 3^2
    //
    // 5^2
    // "25" 24 23 22 "21" 20 19 18 "17" 16 15 14 "13" 12 11 10
    //     <-skip 3->    <-skip 3->    <-skip 3->
    //
    // 7  8  9
    // 6     2
    // 5  4  3
    // 3^2
    // "9"    8     "7"    6     "5"    4     "3"    2
    //    <-skip 1->   <-skip 1->   <-skip 1->
    let mut sum = 1u64;
    for a in (3u32..=1001).step_by(2) {
        for b in (((a - 2).pow(2) + 1)..=a.pow(2))
            .rev()
            .step_by(a as usize - 1)
        {
            sum += b as u64;
        }
    }
    sum
}

/// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
///
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
///
/// It can be verified that the sum of the numbers on the diagonals is 101.
///
/// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
///
/// ```rust
/// use self::project_euler::m28::sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square;
/// assert_eq!(sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square(), 669171001);
/// ```
pub fn sum_of_the_numbers_on_the_diagonals_in_a_1001_by_1001_box_sum_of_square() -> u64 {
    // See (m6)[./m6.rs]
    //
    // n:   1   2   3   4   5
    // max: 1 3^2 5^2 7^2 9^2 = (2*1-1)^2, (2*2-1)^2, (2*3-1)^2, (2*4-1)^2, (2*5-1)^2, = (2n-1)^2
    //      1   9  25  49  81
    //
    // n:          1  2  3  4 ... 501
    // c                      d
    //   43 44 45 46 47 48 49
    //   42 21 22 23 24 25 26
    //   41 20  7  8  9 10 27
    //   40 19  6  1  2 11 28
    //   39 18  5  4  3 12 29
    //   38 17 16 15 14 13 30
    //   37 36 35 34 33 32 31
    // b                      a
    //
    // In the same manner,
    //     n: 1, 2,  3,  4
    // d=max: 1, 9, 25, 49
    //     c: 1, 7, 21, 43
    //   d-c: 0, 2,  4,  6 = (n-1) * 2
    //
    //     n: 1, 2,  3,  4
    // d=max: 1, 9, 25, 49
    //     b: 1, 5, 17, 37
    //   d-b: 0, 4,  8, 12 = (n-1) * 4
    //
    //     n: 1, 2,  3,  4
    // d=max: 1, 9, 25, 49
    //     a: 1, 3, 13, 31
    //   d-a: 0, 6, 12, 18 = (n-1) * 6
    //
    // f(n) = 4 * (2n-1)^2 - (n-1) * 2 - (n-1) * 4 - (n-1) * 6
    //      = 4 * (4n^2-4n+1) - 2n + 2 - 4n + 4 - 6n + 6
    //      = 16n^2 -16n + 4 -2n + 2 -4n + 4 -6n + 6
    //      = 16n^2 -28n +16
    // sigma(f(n),n..=501)
    //      = 16*(n(n+1)(2n+1)/6) -28*((1+n)n/2) +16n
    let n = 501;
    16 * (n * (n + 1) * (2 * n + 1)) / 6 - 28 * ((1 + n) * n) / 2 + 16 * n - 3 /* 1 was counted four times */
}
