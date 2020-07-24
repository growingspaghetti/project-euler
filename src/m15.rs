//!```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- routes_are_there_through_a_ --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 13.38s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through: Collecting 100 samples in estimated 5.0003 s (5479250 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through: Analyzing
//! routes_are_there_through_a_20_20_grid_walk_through
//!                         time:   [875.91 ns 882.23 ns 889.69 ns]
//!                         change: [-2.2238% +0.0000% +2.1750%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 2 outliers among 100 measurements (2.00%)
//!   2 (2.00%) high severe
//! slope  [875.91 ns 889.69 ns] R^2            [0.7513561 0.7496487]
//! mean   [903.87 ns 933.07 ns] std. dev.      [50.366 ns 97.510 ns]
//! median [878.30 ns 922.12 ns] med. abs. dev. [32.585 ns 73.029 ns]
//!
//! Benchmarking routes_are_there_through_a_20_20_grid_combination
//! Benchmarking routes_are_there_through_a_20_20_grid_combination: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_combination: Collecting 100 samples in estimated 5.0000 s (2547699750 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_combination: Analyzing
//! routes_are_there_through_a_20_20_grid_combination
//!                         time:   [1.9286 ns 1.9371 ns 1.9466 ns]
//!                         change: [-1.0789% +0.0000% +1.0714%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//!   2 (2.00%) high mild
//!   3 (3.00%) high severe
//! slope  [1.9286 ns 1.9466 ns] R^2            [0.9173010 0.9166915]
//! mean   [1.9490 ns 1.9778 ns] std. dev.      [56.254 ps 89.220 ps]
//! median [1.9198 ns 1.9659 ns] med. abs. dev. [33.320 ps 78.824 ps]
//!
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers: Collecting 100 samples in estimated 5.0037 s (5762050 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers: Analyzing
//! routes_are_there_through_a_20_20_grid_combination_small_numbers
//!                         time:   [857.41 ns 861.84 ns 866.81 ns]
//!                         change: [-0.9182% +0.0000% +0.9109%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 7 outliers among 100 measurements (7.00%)
//!   6 (6.00%) high mild
//!   1 (1.00%) high severe
//! slope  [857.41 ns 866.81 ns] R^2            [0.9414973 0.9407726]
//! mean   [862.48 ns 873.72 ns] std. dev.      [21.501 ns 35.551 ns]
//! median [852.68 ns 864.21 ns] med. abs. dev. [12.150 ns 24.515 ns]
//!
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square: Collecting 100 samples in estimated 5.0002 s (61332250 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square: Analyzing
//! routes_are_there_through_a_20_20_grid_combination_perfect_square
//!                         time:   [80.940 ns 81.343 ns 81.790 ns]
//!                         change: [-0.6828% +0.0000% +0.6899%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 3 outliers among 100 measurements (3.00%)
//!   2 (2.00%) high mild
//!   1 (1.00%) high severe
//! slope  [80.940 ns 81.790 ns] R^2            [0.9394077 0.9386556]
//! mean   [81.339 ns 82.138 ns] std. dev.      [1.6115 ns 2.4606 ns]
//! median [80.639 ns 81.631 ns] med. abs. dev. [1.0131 ns 2.0575 ns]
//! ```
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/6/66/Walking_on_Pascal%27s_Triangle_SVG.svg/529px-Walking_on_Pascal%27s_Triangle_SVG.svg.png)
//!
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/08bdf0fff474c26293414f9eb01ab4bc73ef941f)
//!
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/e328311df048233471b8d30ff78aac527298646f)

/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_10_10_grid_random;
/// assert_eq!(routes_are_there_through_a_10_10_grid_random(), 184756);
/// ```
/// Note: this takes 5 minutes even with 10x10 grid.
pub fn routes_are_there_through_a_10_10_grid_random() -> u64 {
    use std::collections::HashSet;
    // [↓ .... ↓; 20]
    // [→ .... →; 20]
    let mut options: Vec<&char> = vec![];
    let mut routes: HashSet<String> = HashSet::new();
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    for _ in 0..3000000u64 {
        for _ in 0..10 {
            options.push(&'↓');
            options.push(&'→');
        }
        options.shuffle(&mut rng);
        let s: String = (&options).iter().copied().collect();
        routes.insert(s);
        options.clear();
    }
    //println!("{:?}", routes);
    routes.len() as u64
}

/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_walk_through;
/// assert_eq!(routes_are_there_through_a_20_20_grid_walk_through(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_walk_through() -> u64 {
    // https://en.wikipedia.org/wiki/Lattice_path
    // [↑ .... ↑; 20]
    // [→ .... →; 20]
    // NE lattice paths. [0][0] to fill [20][20] step number is determined as 40.
    // Amass the number of possible paths to reach and fill a point from [0],[0] to [20][20]
    let mut lattice = [[0u64;21];21 /*initial point cell is 0,0 and do +=20*/];

    // lattice[0][0] = 1; // apparently no other way to start.
    // apparently to reach the east end [0],[20], to keep going east is the one and only way.
    // in the same mannar, to reach the south end [20], [0], to keep going south is the one and only way.
    for y in 0..21 {
        for x in 0..21 {
            let up_cell = if y > 0 {
                lattice.get(y - 1).and_then(|row| row.get(x))
            } else {
                None
            };
            let left_cell = if x > 0 {
                lattice.get(y).and_then(|row| row.get(x - 1))
            } else {
                None
            };
            lattice[y][x] = match (up_cell, left_cell) {
                (Some(&a), Some(&b)) => a + b,
                (None, Some(&b)) => b,
                (Some(&a), None) => a,
                _ => 1,
            };
        }
    }
    lattice[20][20]
}

/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_combination;
/// assert_eq!(routes_are_there_through_a_20_20_grid_combination(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_combination() -> u64 {
    fn factorial(n: u128) -> u128 {
        match n {
            0 | 1 => 1,
            _ => factorial(n - 1) * n,
        }
    }

    // 40P20 https://www.calculatorsoup.com/calculators/discretemathematics/permutations.php
    // 335367096786357081410764800000 still too huge for u64
    // https://www.javatpoint.com/program-to-print-the-permutation-of-the-given-number
    // 5P2 = 5! / 3! =   5   4
    //                 5-0 5-1
    fn npr(n: u128, r: u128) -> u128 {
        let mut product = 1u128;
        for i in 0..r {
            product *= n - i;
        }
        product
    }

    // 5P2 = 5! / 3!
    fn divide_factorials_to_npr(a: u128, b: u128) -> (u128, u128) {
        (a, a - b)
    }

    // https://mathworld.wolfram.com/BinomialCoefficient.html
    // nCk (n k) 20+20C20 (40 20) n! / k! * n-k! = 40! / (20! * 20!)
    fn nck(n: u128, k: u128) -> u128 {
        let nk_in_npr = divide_factorials_to_npr(n, k);
        npr(nk_in_npr.0, nk_in_npr.1) / factorial(n - k)
    }

    nck(40, 20) as u64
}

/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_combination_small_numbers;
/// assert_eq!(routes_are_there_through_a_20_20_grid_combination_small_numbers(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_combination_small_numbers() -> u64 {
    // https://mathworld.wolfram.com/BinomialCoefficient.html
    // nCk (n k) 20+20C20 (40 20) n! / k! * n-k! = 40! / (20! * 20!)
    //
    // n! / k!(n-k)! = n-0 * n-1 * n-2 * .. * n-k+1(excluding 1..=k but 0..k) / k!
    fn nck(n: u128, k: u128) -> u128 {
        let mut k_series = vec![];
        for i in (2..=k).rev() {
            k_series.push(i);
        }
        let mut sum = 1u128;
        for i in 0..k {
            sum *= n - i;
            'inner: while let Some(d) = k_series.last() {
                if sum % d == 0 {
                    sum /= d;
                    k_series.pop();
                } else {
                    break 'inner;
                }
            }
        }
        while let Some(d) = k_series.pop() {
            sum /= d;
        }
        sum
    }
    nck(40, 20) as u64
}

/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_combination_perfect_square;
/// assert_eq!(routes_are_there_through_a_20_20_grid_combination_perfect_square(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_combination_perfect_square() -> u64 {
    // when spacial case 2nCn, Capital Pi function can be used.
    // i:1..=n CP (n + 1) / i
    let k = 20f64;
    let mut product = 1f64;
    for i in 1..=k as i32 {
        product *= (k + i as f64) / i as f64
    }
    product.ceil() as u64
}
