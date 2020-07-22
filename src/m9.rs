//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- pythagorean_triplet_a --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!     Finished bench [optimized] target(s) in 8.07s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking pythagorean_triplet_and_1000
//! Benchmarking pythagorean_triplet_and_1000: Warming up for 3.0000 s
//! Benchmarking pythagorean_triplet_and_1000: Collecting 100 samples in estimated 5.3323 s (75750 iterations)
//! Benchmarking pythagorean_triplet_and_1000: Analyzing
//! pythagorean_triplet_and_1000
//!                         time:   [69.862 us 70.057 us 70.267 us]
//!                         change: [-0.8730% +0.0000% +0.7507%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   2 (2.00%) high mild
//!   2 (2.00%) high severe
//! slope  [69.862 us 70.267 us] R^2            [0.9691470 0.9689691]
//! mean   [70.473 us 71.299 us] std. dev.      [1.2769 us 2.9670 us]
//! median [69.986 us 70.550 us] med. abs. dev. [745.15 ns 1.4324 us]
//!
//! Benchmarking pythagorean_triplet_and_1000_triangle_means_500
//! Benchmarking pythagorean_triplet_and_1000_triangle_means_500: Warming up for 3.0000 s
//! Benchmarking pythagorean_triplet_and_1000_triangle_means_500: Collecting 100 samples in estimated 5.4439 s (55550 iterations)
//! Benchmarking pythagorean_triplet_and_1000_triangle_means_500: Analyzing
//! pythagorean_triplet_and_1000_triangle_means_500
//!                         time:   [96.729 us 97.027 us 97.351 us]
//!                         change: [-0.6017% +0.0000% +0.5919%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 2 outliers among 100 measurements (2.00%)
//!   2 (2.00%) high mild
//! slope  [96.729 us 97.351 us] R^2            [0.9743006 0.9740681]
//! mean   [97.425 us 98.257 us] std. dev.      [1.8037 us 2.4409 us]
//! median [96.893 us 98.013 us] med. abs. dev. [1.3411 us 2.5117 us]
//!
//! Benchmarking pythagorean_triplet_and_1000_triangle_euclid
//! Benchmarking pythagorean_triplet_and_1000_triangle_euclid: Warming up for 3.0000 s
//! Benchmarking pythagorean_triplet_and_1000_triangle_euclid: Collecting 100 samples in estimated 5.0001 s (17250800 iterations)
//! Benchmarking pythagorean_triplet_and_1000_triangle_euclid: Analyzing
//! pythagorean_triplet_and_1000_triangle_euclid
//!                         time:   [281.47 ns 282.46 ns 283.61 ns]
//!                         change: [-0.7822% +0.0000% +0.7649%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 7 outliers among 100 measurements (7.00%)
//!   4 (4.00%) high mild
//!   3 (3.00%) high severe
//! slope  [281.47 ns 283.61 ns] R^2            [0.9628056 0.9622413]
//! mean   [283.62 ns 286.79 ns] std. dev.      [5.1932 ns 10.996 ns]
//! median [281.83 ns 283.69 ns] med. abs. dev. [2.8776 ns 5.2314 ns]
//! ```
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/c5f23769007c289d0faf2092cbb00a4cb5630f1d)

/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2
///
/// For example, 32 + 42 = 9 + 16 = 25 = 52.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.
///
/// ```rust
/// use self::project_euler::m9::pythagorean_triplet_and_1000;
/// assert_eq!(pythagorean_triplet_and_1000(1000), 31875000);
/// ```
pub fn pythagorean_triplet_and_1000(sum: u32) -> u32 {
    // a < b < c <= if a^2 < b^2 < c^2
    // a, b=[a+1.. && <c], c=1000-b-a
    let mut a = 1u32;
    let mut b = 2u32;
    let mut c;
    'outer: loop {
        'inner: loop {
            c = sum - a - b;
            if b >= c {
                break 'inner;
            }
            if a * a + b * b == c * c {
                break 'outer a * b * c;
            }
            b += 1;
        }
        a += 1;
        b = a + 1;
    }
}

/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2
///
/// For example, 32 + 42 = 9 + 16 = 25 = 52.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.
///
/// ```rust
/// use self::project_euler::m9::pythagorean_triplet_and_1000;
/// assert_eq!(pythagorean_triplet_and_1000(1000), 31875000);
/// ```
pub fn pythagorean_triplet_and_1000_triangle_means_500(sum: u32) -> u32 {
    // a < b < c <= if a^2 < b^2 < c^2
    // a, b=[a+1.. && <c], c=1000-b-a
    let mut a = 1u32;
    let mut b = 2u32;
    let mut c;
    'outer: loop {
        'inner: loop {
            c = sum - a - b;
            if b >= 500 || b >= c {
                break 'inner;
            }
            if a * a + b * b == c * c {
                break 'outer a * b * c;
            }
            b += 1;
        }
        a += 1;
        b = a + 1;
    }
}

/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2
///
/// For example, 32 + 42 = 9 + 16 = 25 = 52.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.
///
/// ```rust
/// use self::project_euler::m9::pythagorean_triplet_and_1000_triangle_euclid;
/// assert_eq!(pythagorean_triplet_and_1000_triangle_euclid(1000), 31875000);
/// ```
pub fn pythagorean_triplet_and_1000_triangle_euclid(sum: u32) -> u32 {
    // https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple
    // a = m^2 - n^2, b = 2mn, c = m^2 + n^2
    // because b = 2mn && b<1000, both m, n are < 500
    // because c = m^2 + n^2 && c<1000, both m, n are < sqrt(1000)=31.6
    let mut m = 1u32;
    let mut n = 1u32;

    'outer: loop {
        'inner: loop {
            if n > m || n > (sum as f32).sqrt().floor() as u32 {
                break 'inner;
            }
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            if a + b + c == sum && a * a + b * b == c * c {
                break 'outer a * b * c;
            }
            n += 1;
        }
        m += 1;
        n = 1;
    }
}
