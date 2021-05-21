//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- how_many_different_ways_can_2_be_made --verbose --baseline new
//!
//!     Finished bench [optimized] target(s) in 0.05s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking how_many_different_ways_can_2_be_made_brute
//! Benchmarking how_many_different_ways_can_2_be_made_brute: Warming up for 3.0000 s
//! Benchmarking how_many_different_ways_can_2_be_made_brute: Collecting 100 samples in estimated 5.2304 s (1500 iterations)
//! Benchmarking how_many_different_ways_can_2_be_made_brute: Analyzing
//! how_many_different_ways_can_2_be_made_brute
//!                         time:   [3.4793 ms 3.4913 ms 3.5041 ms]
//!                         change: [-0.5051% +0.0000% +0.5087%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 7 outliers among 100 measurements (7.00%)
//!   7 (7.00%) high mild
//! mean   [3.4793 ms 3.5041 ms] std. dev.      [51.848 us 74.091 us]
//! median [3.4626 ms 3.4856 ms] med. abs. dev. [32.115 us 57.887 us]
//!
//! Benchmarking how_many_different_ways_can_2_be_made
//! Benchmarking how_many_different_ways_can_2_be_made: Warming up for 3.0000 s
//! Benchmarking how_many_different_ways_can_2_be_made: Collecting 100 samples in estimated 5.0143 s (1323100 iterations)
//! Benchmarking how_many_different_ways_can_2_be_made: Analyzing
//! how_many_different_ways_can_2_be_made
//!                         time:   [3.7353 us 3.7455 us 3.7555 us]
//!                         change: [-0.6680% +0.0000% +0.6980%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 3 outliers among 100 measurements (3.00%)
//!   2 (2.00%) high mild
//!   1 (1.00%) high severe
//! slope  [3.7353 us 3.7555 us] R^2            [0.9801675 0.9801906]
//! mean   [3.7329 us 3.7681 us] std. dev.      [43.126 ns 141.32 ns]
//! median [3.7190 us 3.7526 us] med. abs. dev. [29.202 ns 56.456 ns]
//!
//! Benchmarking how_many_different_ways_can_2_be_made_rec
//! Benchmarking how_many_different_ways_can_2_be_made_rec: Warming up for 3.0000 s
//! Benchmarking how_many_different_ways_can_2_be_made_rec: Collecting 100 samples in estimated 5.0083 s (300 iterations)
//! Benchmarking how_many_different_ways_can_2_be_made_rec: Analyzing
//! how_many_different_ways_can_2_be_made_rec
//!                         time:   [16.603 ms 16.639 ms 16.679 ms]
//!                         change: [-0.3266% +0.0000% +0.3254%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//!   3 (3.00%) high mild
//!   2 (2.00%) high severe
//! mean   [16.603 ms 16.679 ms] std. dev.      [145.67 us 245.68 us]
//! median [16.568 ms 16.631 ms] med. abs. dev. [87.376 us 173.14 us]
//!
//! Benchmarking how_many_different_ways_can_2_be_made_rec_memo
//! Benchmarking how_many_different_ways_can_2_be_made_rec_memo: Warming up for 3.0000 s
//! Benchmarking how_many_different_ways_can_2_be_made_rec_memo: Collecting 100 samples in estimated 5.0875 s (212100 iterations)
//! Benchmarking how_many_different_ways_can_2_be_made_rec_memo: Analyzing
//! how_many_different_ways_can_2_be_made_rec_memo
//!                         time:   [25.214 us 25.304 us 25.393 us]
//!                         change: [-0.5730% +0.0000% +0.5335%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   2 (2.00%) high mild
//!   2 (2.00%) high severe
//! slope  [25.214 us 25.393 us] R^2            [0.9717930 0.9718079]
//! mean   [25.215 us 25.415 us] std. dev.      [353.19 ns 662.21 ns]
//! median [25.100 us 25.334 us] med. abs. dev. [273.04 ns 453.04 ns]
//! ```
//! See (m18)[m18.rs], (m15)[m15.rs]
//! ![](https://i2.wp.com/algorithms.tutorialhorizon.com/files/2015/06/Coin-Change-Problem.jpg)

/// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
///
/// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
///
/// It is possible to make £2 in the following way:
///
/// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
///
/// How many different ways can £2 be made using any number of coins?
///
///
/// ```rust
/// use self::project_euler::m31::how_many_different_ways_can_2_be_made_brute;
/// assert_eq!(how_many_different_ways_can_2_be_made_brute(), 73682);
/// ```
#[rustfmt::skip]
#[allow(clippy::identity_op)]
pub fn how_many_different_ways_can_2_be_made_brute() -> u32 {
    let target = 200u32;
    let mut ways = 0u32;
    for a in 0..=target/200 {
        for b in 0..=(target-a*200)/100 {
            for c in 0..=(target-a*200-b*100)/50 {
                for d in 0..=(target-a*200-b*100-c*50)/20 {
                    for e in 0..=(target-a*200-b*100-c*50-d*20)/10 {
                        for f in 0..=(target-a*200-b*100-c*50-d*20-e*10)/5 {
                            for g in 0..=(target-a*200-b*100-c*50-d*20-e*10-f*5)/2 {
                                for h in 0..=(target-a*200-b*100-c*50-d*20-e*10-f*5-g*2)/1 {
                                    if target-a*200-b*100-c*50-d*20-e*10-f*5-g*2-h*1 == 0 {
                                        ways += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ways
}

fn combination(payment: usize, coins: &[usize]) -> usize {
    assert!(payment > 0);
    println!("Payment of {} with coins {:?}", payment, &coins);
    let mut c = 0usize;
    next_coin(payment, &coins, String::new(), &mut c);
    println!("Number of combinations:{}\n", c);
    c
}

fn next_coin(payment: usize, coins: &[usize], path: String, comb: &mut usize) {
    if let Some(co) = coins.first() {
        dig(payment, *co, &coins[1..], path, comb);
    } else if payment == 0 {
        println!("{} |", path);
        *comb += 1;
    }
}

fn dig(payment: usize, co: usize, coins: &[usize], path: String, comb: &mut usize) {
    let num = payment / co;
    for n in 0..=num {
        let mut path = path.clone();
        if n == 0 {
            path.push_str("|           ");
        } else {
            path.push_str(format!("| {:4}x{:4} ", n, co).as_str());
        }
        next_coin(payment - co * n, coins, path, comb);
    }
}

/// ```rust
/// use self::project_euler::m31::how_many_different_ways_can_2_be_made_brute_print;
/// assert_eq!(how_many_different_ways_can_2_be_made_brute_print(), 5);
/// ```
pub fn how_many_different_ways_can_2_be_made_brute_print() {
    let target = 20usize;
    let mut coins = [1usize, 2, 5, 10, 20, 50, 100, 200];
    coins.reverse();
    let mut comb = 0usize;
    print_comb(target, &coins, String::new(), &mut comb);
}

/// https://www.youtube.com/watch?v=tduLvFbqRXE "Programming Interview:Algorithm:Dynamic Programming: Coin Changing Problem"
/// ```rust
/// use self::project_euler::m31::change_of_five_cents;
/// assert_eq!(change_of_five_cents(), 5);
/// ```
pub fn change_of_five_cents() -> u8 {
    // denomination of coins are 1p 2p 3p for paying sum 5p
    // 1. 1p alone sum([1,1,1,1,1])
    // 2.          sum([1,1,1, 2 ])
    // 3.          sum([1,1,  3  ])
    // 4.          sum([ 2 ,  3  ])
    // 5.          sum([ 2 , 2 ,1])
    //
    // available path to reach to sum
    // 1. denomination of coins are 1p 2p 3p for paying sum 0p
    //        1p      2p      3p
    //    sum([]) sum([]) sum([])
    //
    // 2. denomination of coins are 1p paying sum 1p to sum 5p
    //  a sum([1])
    //  b sum([1,1])
    //  c sum([1,1,1])
    //  d sum([1,1,1,1])
    //  e sum([1,1,1,1,1])
    //
    // 3. denomination of coins are 2p paying sum 1p to sum 5p
    //  a sum([]) + 2.a sum([1])
    //  b sum([2]), sum([]) + 2.b sum([1,1])
    //  c sum([2]) + 2.a sum([1]), sum([]) + 2.c sum([1,1,1])
    //  d sum([2,2]), sum([2]) + 2.b sum([1,1]), sum([]) + 2.d sum([1,1,1,1])
    //  e sum([2,2]) + 2.a sum([1]), sum([2]) + 2.c sum([1,1,1]), sum([]) + 2.e sum([1,1,1,1,1])
    //
    // 4. denomination of coins are 3p paying sum 1p to sum 5p
    //  a sum([]) + 2.a sum([1])
    //  b sum([]) + 2.b sum([1,1]), sum([]) + 3.b sum([2])
    //  c sum([3]), sum([]) + 2.c sum([1,1,1]), sum([]) + 3.c sum([2,1])
    //  d sum([3]) + 2.a sum([1]), sum([]) + 3.d sum([2,2]), sum([]) + 3.d sum([2,1,1]), sum([]) + 2.d sum([1,1,1,1])
    //  e sum([3,2]), sum([3]) + 2.b sum([1,1]), sum([2,2]) + 2.a sum([1]), sum([2]) + 2.c sum[1,1,1], 2.e sum([1,1,1,1,1])
    //
    // In a table notation,
    //           <------------------------------ x ---------------------------------->
    //  \target      0        1        2           3          4              5
    // | option | sum(0) | sum(1) | sum(2) |    sum(3)   |    sum(4)   |    sum(5)   |   ^
    // |     d0 |      0 |    inf |    inf |         inf |         inf |         inf | 0 |
    // |     d1 | none 1 |  all 1 |  all 1 |       all 1 |       all 1 |       all 1 | 1 |
    // |     d2 | none 1 |  [1][1]|1+[2][1]|[1][0]+[3][1]|     1+[2][2]|     1+[3][2]| 2 y
    // |     d3 | none 1 |  [1][1]|  [2][2]|     1+[3][2]|[1][0]+[4][2]|[2][1]+[5][2]| 3 |
    // -- Targeting at sum(n) with coins[0:] y in your hands.
    let mut matrix = [[0u8; 6]; 4];
    // denominator 1, coin I
    for sum in 0..=5 {
        matrix[1][sum] = 1;
    }
    // denominator 2
    for sum in 0..=5 {
        // without using coin II
        let without_pattern = matrix[1][sum];
        // use coin II
        let with_pattern = if sum >= 2 { matrix[2][sum - 2] } else { 0 };
        matrix[2][sum] = with_pattern + without_pattern;
    }
    // denominator 3
    for sum in 0..=5 {
        // without using coin III
        let without_pattern = matrix[2][sum];
        // use coin III
        let with_pattern = if sum >= 3 { matrix[3][sum - 3] } else { 0 };
        matrix[3][sum] = with_pattern + without_pattern;
    }
    for r in matrix.iter() {
        // [0, 0, 0, 0, 0, 0]
        // [1, 1, 1, 1, 1, 1]
        // [1, 1, 2, 2, 3, 3]
        // [1, 1, 2, 3, 4, 5]
        println!("{:?}", r);
    }
    let mut max = 0u8;
    for r in matrix.iter() {
        let &m = r.iter().max().unwrap();
        if m > max {
            max = m;
        }
    }
    max
    // An easy case to consider is sum(3) with coins d1,d2,d3
    //    sum0 sum1 sum2 sum3
    // d0    0    i    i    i
    // d1    1
    //
    //    sum0 sum1 sum2 sum3
    // d0    0    0    0    0
    // d1    1    1    1    1
    // d2    1
    // d3    1
    //
    //    sum0 sum1 sum2 sum3
    // d0    0    0    0    0
    // d1    1    1    1    1
    // d2    1    1    2    2
    // d3    1    1    2    3
    //
    //    sum0 sum1 sum2 sum3
    // d0    0    0    0    0
    // d1    1 sum0+^  1    1
    // d2    1    1 sum0+^  2
    // d3    1    1    2 sum0+^ .. sum([1,1,1]),sum([2,1]),sum([3])
    //
    //    sum0 sum1 sum2 sum3
    // d0    0    0    0    0
    // d1    1 sum0+^  1    1
    // d2    1    ^ sum0+^ sum1+^
    // d3    1    ^    ^ sum0+^ .. sum([1,1,1]),sum([2,1]),sum([3])
}

/// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
///
/// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
///
/// It is possible to make £2 in the following way:
///
/// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
///
/// How many different ways can £2 be made using any number of coins?
///
///
/// ```rust
/// use self::project_euler::m31::how_many_different_ways_can_2_be_made;
/// assert_eq!(how_many_different_ways_can_2_be_made(), 73682);
/// ```
pub fn how_many_different_ways_can_2_be_made() -> u32 {
    // https://en.wikipedia.org/wiki/Change-making_problem
    // See mini question above.
    let coins: [u8; 9] = [0, 1, 2, 5, 10, 20, 50, 100, 200];
    let mut matrix = [[0u32; 201]; 9];
    // denominator 1, coin I
    for sum in 0..=200 {
        matrix[1][sum] = 1;
    }
    for d in 2..coins.len() {
        let coin = coins[d] as usize;
        for sum in 0..=200 {
            // without using this coin, coin[d]
            let without_pattern = matrix[d - 1][sum];
            // use this coin, coin[d], after using this coin
            let with_pattern = if sum >= coin {
                matrix[d][sum - coin]
            } else {
                0
            };
            matrix[d][sum] = with_pattern + without_pattern;
        }
    }
    let mut max = 0u32;
    for r in matrix.iter() {
        let &m = r.iter().max().unwrap();
        if m > max {
            max = m;
        }
    }
    max
}

use std::cmp::Ordering;

fn coin_change(payment: usize, coins: &[usize]) -> u32 {
    let mut table = vec![vec![0u32; payment]; coins.len()];
    for c in 0..table.len() {
        for p in 0..table[c].len() {
            let v_no = if c == 0 { 0u32 } else { table[c - 1][p] };
            let v_we = match (p + 1).partial_cmp(&(coins[c])).expect("NaNs") {
                Ordering::Less => 0u32,
                Ordering::Equal => 1u32,
                Ordering::Greater => table[c][p - coins[c]],
            };
            table[c][p] += v_no + v_we;
        }
    }
    table[coins.len() - 1][payment - 1]
}

// 4.7 us
/// ```rust
/// use self::project_euler::m31::how_many_different_ways_can_2_be_made_2;
/// assert_eq!(how_many_different_ways_can_2_be_made_2(), 73682);
/// ```
pub fn how_many_different_ways_can_2_be_made_2() -> u32 {
    let payment = 200usize;
    let coins = [1usize, 2, 5, 10, 20, 50, 100, 200];
    coin_change(payment, &coins)
}

/// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
///
/// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
///
/// It is possible to make £2 in the following way:
///
/// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
///
/// How many different ways can £2 be made using any number of coins?
///
///
/// ```rust
/// use self::project_euler::m31::how_many_different_ways_can_2_be_made_rec;
/// assert_eq!(how_many_different_ways_can_2_be_made_rec(), 73682);
/// ```
pub fn how_many_different_ways_can_2_be_made_rec() -> u32 {
    fn preceding_coin(coin: u8) -> Option<u8> {
        match coin {
            1 => None,
            2 => Some(1),
            5 => Some(2),
            10 => Some(5),
            20 => Some(10),
            50 => Some(20),
            100 => Some(50),
            200 => Some(100),
            _ => panic!(),
        }
    }
    fn ways(sum: u8, coin: Option<u8>) -> u32 {
        if coin == None {
            return 0;
        }
        if sum == 0 {
            return 1;
        }
        let c = coin.unwrap();
        let without_pattern = ways(sum, preceding_coin(c));
        let with_pattern = if sum >= c { ways(sum - c, coin) } else { 0 };
        with_pattern + without_pattern
    }
    ways(200, Some(200))
}

/// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
///
/// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
///
/// It is possible to make £2 in the following way:
///
/// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
///
/// How many different ways can £2 be made using any number of coins?
///
///
/// ```rust
/// use self::project_euler::m31::how_many_different_ways_can_2_be_made_rec_memo;
/// assert_eq!(how_many_different_ways_can_2_be_made_rec_memo(), 73682);
/// ```
pub fn how_many_different_ways_can_2_be_made_rec_memo() -> u32 {
    fn preceding_coin(coin: u8) -> Option<u8> {
        match coin {
            1 => None,
            2 => Some(1),
            5 => Some(2),
            10 => Some(5),
            20 => Some(10),
            50 => Some(20),
            100 => Some(50),
            200 => Some(100),
            _ => panic!(),
        }
    }
    fn ways(sum: u8, coin: Option<u8>, memo: &mut Vec<Vec<i32>>) -> u32 {
        if coin == None {
            return 0;
        }
        if sum == 0 {
            return 1;
        }
        let c = coin.unwrap();
        match memo[c as usize][sum as usize] {
            -1 => (),
            d => return d as u32,
        }
        let without_pattern = ways(sum, preceding_coin(c), memo);
        let with_pattern = if sum >= c {
            ways(sum - c, coin, memo)
        } else {
            0
        };
        let combination = with_pattern + without_pattern;
        memo[c as usize][sum as usize] = combination as i32;
        combination
    }
    let mut memo = vec![vec![-1; 201]; 201];
    ways(200, Some(200), &mut memo)
}
