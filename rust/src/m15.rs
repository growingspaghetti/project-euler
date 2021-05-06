//!```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- routes_are_there_through_a_ --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//! Finished bench [optimized] target(s) in 0.05s
//! Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through: Collecting 100 samples in estimated 5.0041 s (5757000 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_walk_through: Analyzing
//! routes_are_there_through_a_20_20_grid_walk_through
//!                    time:   [861.71 ns 865.02 ns 868.81 ns]
//!                    change: [-0.9824% +0.0000% +1.0046%] (p = 1.00 > 0.05)
//!                    No change in performance detected.
//! Found 11 outliers among 100 measurements (11.00%)
//! 6 (6.00%) high mild
//! 5 (5.00%) high severe
//! slope  [861.71 ns 868.81 ns] R^2            [0.9497085 0.9491258]
//! mean   [866.92 ns 879.65 ns] std. dev.      [20.603 ns 43.963 ns]
//! median [857.31 ns 865.61 ns] med. abs. dev. [7.7015 ns 16.662 ns]
//!
//! Benchmarking routes_are_there_through_a_20_20_grid_combination
//! Benchmarking routes_are_there_through_a_20_20_grid_combination: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_combination: Collecting 100 samples in estimated 5.0000 s (2593669900 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_combination: Analyzing
//! routes_are_there_through_a_20_20_grid_combination
//!                    time:   [1.9110 ns 1.9170 ns 1.9237 ns]
//!                    change: [-0.7170% +0.0000% +0.7033%] (p = 1.00 > 0.05)
//!                    No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//! 4 (4.00%) high mild
//! 1 (1.00%) high severe
//! slope  [1.9110 ns 1.9237 ns] R^2            [0.9730688 0.9727294]
//! mean   [1.9230 ns 1.9423 ns] std. dev.      [36.045 ps 63.408 ps]
//! median [1.9034 ns 1.9231 ns] med. abs. dev. [13.032 ps 37.264 ps]
//!
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers: Collecting 100 samples in estimated 5.0007 s (5863050 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers: Analyzing
//! routes_are_there_through_a_20_20_grid_combination_small_numbers
//!                    time:   [845.60 ns 848.75 ns 852.27 ns]
//!                    change: [-0.9554% +0.0000% +0.9305%] (p = 1.00 > 0.05)
//!                    No change in performance detected.
//! Found 7 outliers among 100 measurements (7.00%)
//! 6 (6.00%) high mild
//! 1 (1.00%) high severe
//! slope  [845.60 ns 852.27 ns] R^2            [0.9620602 0.9615935]
//! mean   [851.68 ns 863.38 ns] std. dev.      [17.942 ns 43.125 ns]
//! median [842.65 ns 850.19 ns] med. abs. dev. [6.7854 ns 16.410 ns]
//!
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers_series
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers_series: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers_series: Collecting 100 samples in estimated 5.0006 s (19508150 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_small_numbers_series: Analyzing
//! routes_are_there_through_a_20_20_grid_combination_small_numbers_series
//!                    time:   [253.91 ns 254.73 ns 255.63 ns]
//!                    change: [-0.7901% +0.0000% +0.7313%] (p = 1.00 > 0.05)
//!                    No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//! 3 (3.00%) high mild
//! 2 (2.00%) high severe
//! slope  [253.91 ns 255.63 ns] R^2            [0.9714490 0.9711834]
//! mean   [255.27 ns 258.12 ns] std. dev.      [4.5618 ns 10.125 ns]
//! median [253.13 ns 255.44 ns] med. abs. dev. [2.4928 ns 5.0815 ns]
//!
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square: Warming up for 3.0000 s
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square: Collecting 100 samples in estimated 5.0000 s (61832200 iterations)
//! Benchmarking routes_are_there_through_a_20_20_grid_combination_perfect_square: Analyzing
//! routes_are_there_through_a_20_20_grid_combination_perfect_square
//!                    time:   [80.339 ns 80.565 ns 80.805 ns]
//!                    change: [-0.6864% +0.0000% +0.6664%] (p = 0.99 > 0.05)
//!                    No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//! 2 (2.00%) high mild
//! 3 (3.00%) high severe
//! slope  [80.339 ns 80.805 ns] R^2            [0.9787235 0.9785826]
//! mean   [80.667 ns 81.427 ns] std. dev.      [1.2798 ns 2.5809 ns]
//! median [80.018 ns 80.826 ns] med. abs. dev. [660.10 ps 1.5764 ns]
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

//  865 ns
/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_walk_through;
/// assert_eq!(routes_are_there_through_a_20_20_grid_walk_through(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_walk_through() -> u64 {
    // // https://en.wikipedia.org/wiki/Lattice_path
    // // [↑ .... ↑; 20]
    // // [→ .... →; 20]
    // // NE lattice paths. [0][0] to fill [20][20] step number is determined as 40.
    // // Amass the number of possible paths to reach and fill a point from [0],[0] to [20][20]
    // let mut lattice = [[0u64;21];21 /*initial point cell is 0,0 and do +=20*/];

    // // lattice[0][0] = 1; // apparently no other way to start.
    // // apparently to reach the east end [0],[20], to keep going east is the one and only way.
    // // in the same mannar, to reach the south end [20], [0], to keep going south is the one and only way.
    // for y in 0..21 {
    //     for x in 0..21 {
    //         let up_cell = if y > 0 {
    //             lattice.get(y - 1).and_then(|row| row.get(x))
    //         } else {
    //             None
    //         };
    //         let left_cell = if x > 0 {
    //             lattice.get(y).and_then(|row| row.get(x - 1))
    //         } else {
    //             None
    //         };
    //         lattice[y][x] = match (up_cell, left_cell) {
    //             (Some(&a), Some(&b)) => a + b,
    //             (None, Some(&b)) => b,
    //             (Some(&a), None) => a,
    //             _ => 1,
    //         };
    //     }
    // }
    // lattice[20][20]
    let mut lattice = [[0u64; 22]; 22];
    lattice[1][1] = 1;
    for y in 1..lattice.len() {
        for x in 1..lattice.len() {
            lattice[y][x] += lattice[y - 1][x] + lattice[y][x - 1];
        }
    }
    lattice[21][21]
}

// 1.98 ns
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

// 254 ns
/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_combination_small_numbers_series;
/// assert_eq!(routes_are_there_through_a_20_20_grid_combination_small_numbers_series(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_combination_small_numbers_series() -> u64 {
    // https://mathworld.wolfram.com/BinomialCoefficient.html
    // nCk (n k) 20+20C20 (40 20) n! / k! * n-k! = 40! / (20! * 20!)
    //
    // n! / k!(n-k)! = n-0 * n-1 * n-2 * .. * n-k+1(excluding 1..=k but 0..k) / k!
    //
    // nCk = pre: nCk-1 * ((n-k+1)/k)
    //  and it means
    //    =  nCk-2 * ((n-k-1+1)/k-1) * ((n-k+1)/k)
    //    =  nCk-3 * ((n-k-2+1)/k-2) * ((n-k-1+1)/k-1) * ((n-k+1)/k)
    // 40C20 = 40C19 * (40-20+1/20)
    // 40C19 = 40C18 * (40-19+1/19)
    // 40C2  = 40C2  * (40-2+1/2)
    // 40C1  = 40C0  * (40-1+1/1) = 40 anyway
    // 40C0  = 1 (take all is the one and only way)
    fn nck(n: u128, k: u128) -> f64 {
        if k == 1 {
            return n as f64;
        }
        nck(n, k - 1) * ((n - k + 1) as f64 / k as f64)
    }
    nck(40, 20) as u64
}

struct Combination {
    n: f64,
    k: f64,
}

impl Combination {
    fn _recursion(&self, n: f64, k: f64) -> f64 {
        if k == 1f64 {
            return n;
        }
        let prev = self._recursion(n, k - 1f64);
        prev * (n - k + 1f64) / k
    }
    fn num(&self) -> f64 {
        self._recursion(self.n, self.k)
    }
}

// 83 ns
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
    let mut p = 1f64;
    (1..=k as isize)
        .map(|x| x as f64)
        .map(|x| (k + x) / x)
        .product::<f64>();
    product.ceil() as u64
}

fn gcd(mut a: u8, mut b: u8) -> u8 {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    assert!(b != 0);
    let r = a % b;
    if r == 0 {
        return b;
    }
    gcd(b, r)
}

fn factorial(n: u8) -> u128 {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n as u128,
    }
}

fn reduce(a: &mut u8, b: &mut u8) {
    assert!(*a > 0 && *b > 0);
    if *a == 1 || *b == 1 {
        return;
    }
    let mut cd = gcd(*a, *b);
    if cd != 1 {
        while {
            *a /= cd;
            *b /= cd;
            if *a == 1 || *b == 1 {
                return;
            }
            cd = gcd(*a, *b);
            cd != 1
        } {}
    }
}

// 446 ns
/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_combination_2;
/// assert_eq!(routes_are_there_through_a_20_20_grid_combination_2(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_combination_2() -> u64 {
    struct Permutation {
        numerator: u8,
        denominator: Vec<u8>,
        _numerator_elem: Vec<u8>,
        _denominator_elem: Vec<u8>,
    }

    impl Default for Permutation {
        fn default() -> Permutation {
            Permutation {
                numerator: 0,
                denominator: vec![],
                _numerator_elem: vec![],
                _denominator_elem: vec![],
            }
        }
    }

    impl Permutation {
        fn reduce(&mut self) {
            if let Some(d0) = self.denominator.pop() {
                for i in (d0..self.numerator).rev() {
                    self._numerator_elem.push(i + 1);
                }
            }
            for &d in self.denominator.iter() {
                for i in 2..=d {
                    self._denominator_elem.push(i);
                }
            }
            // for n in self._denominator_elem.iter_mut() {
            //     for d in self._numerator_elem.iter_mut() {
            //         reduce(n, d);
            //     }
            // }
        }
        fn num(&mut self) -> u64 {
            let mut denominator_elem_shrink: Vec<u8> = vec![];
            let mut ans = 1u64;
            for n0 in &self._numerator_elem {
                ans *= *n0 as u64;
                if let Some(d0) = self._denominator_elem.pop() {
                    if ans % d0 as u64 == 0 {
                        ans /= d0 as u64;
                    } else {
                        denominator_elem_shrink.push(d0);
                    }
                };
            }
            // println!("{:?}", self._numerator_elem);
            // println!("{:?}", self._denominator_elem);
            //println!("{:?}", denominator_elem_shrink);
            // let numerator: u64 = self._numerator_elem.iter().map(|&v| v as u64).product();
            // let denominator: u64 = self._denominator_elem.iter().map(|&v| v as u64).product();
            // numerator / denominator
            let d: u64 = denominator_elem_shrink.iter().map(|&v| v as u64).product();
            ans / d
        }
    }

    struct Grid {
        width: u8,
        height: u8,
    }

    impl Grid {
        fn routes(&self) -> u64 {
            let mut p = Permutation {
                numerator: self.width + self.height,
                denominator: vec![self.width, self.height],
                ..Default::default()
            };
            p.reduce();
            p.num()
        }
    }

    Grid {
        width: 20,
        height: 20,
    }
    .routes()
}

// 182 ns
/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_combination_3;
/// assert_eq!(routes_are_there_through_a_20_20_grid_combination_3(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_combination_3() -> u64 {
    struct Permutation {
        numerator: u8,
        denominator: Vec<u8>,
        _numerator_elem: Vec<u8>,
    }

    impl Default for Permutation {
        fn default() -> Permutation {
            Permutation {
                numerator: 0,
                denominator: vec![],
                _numerator_elem: vec![],
            }
        }
    }

    impl Permutation {
        fn reduce(&mut self) {
            if let Some(d0) = self.denominator.pop() {
                for i in d0..self.numerator {
                    self._numerator_elem.push(i + 1);
                }
            }
        }
        fn num(&mut self) -> u64 {
            (self
                ._numerator_elem
                .iter()
                .map(|&v| v as u128)
                .product::<u128>()
                / factorial(20)) as u64
        }
    }

    struct Grid {
        width: u8,
        height: u8,
    }

    impl Grid {
        fn routes(&self) -> u64 {
            let mut p = Permutation {
                numerator: self.width + self.height,
                denominator: vec![self.width, self.height],
                ..Default::default()
            };
            p.reduce();
            p.num()
        }
    }

    Grid {
        width: 20,
        height: 20,
    }
    .routes()
}

struct Permutation {
    numerator: u8,
    denominator: u8,
}

impl Permutation {
    fn num(&self) -> u128 {
        let mut acc = 1u128;
        for i in self.denominator + 1..self.numerator + 1 {
            acc *= i as u128;
        }
        acc
    }
}

struct Grid {
    width: u8,
    height: u8,
}

impl Grid {
    fn routes(&self) -> u64 {
        let a = Permutation {
            numerator: self.width + self.height,
            denominator: self.width,
        }
        .num();
        let b = factorial(self.height);
        (a / b) as u64
    }
}

// 2.0 ns
/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
/// ![](https://projecteuler.net/project/images/p015.png)
/// How many such routes are there through a 20×20 grid?
///
/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_combination_4;
/// assert_eq!(routes_are_there_through_a_20_20_grid_combination_4(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_combination_4() -> u64 {
    Grid {
        width: 20,
        height: 20,
    }
    .routes()
}

fn nCk(n: u8, k: u8) -> u64 {
    if k == 0 || k == n {
        return 1;
    }
    nCk(n - 1, k - 1) + nCk(n - 1, k)
}

/// ```rust
/// use self::project_euler::m15::routes_are_there_through_a_20_20_grid_rec;
/// assert_eq!(routes_are_there_through_a_20_20_grid_rec(), 137846528820);
/// ```
pub fn routes_are_there_through_a_20_20_grid_rec() -> u64 {
    nCk(40, 20)
}
