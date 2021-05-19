//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- millionth_lexicographic_permutation --verbose
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 17.35s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking millionth_lexicographic_permutation_brute
//! Benchmarking millionth_lexicographic_permutation_brute: Warming up for 3.0000 s
//! Benchmarking millionth_lexicographic_permutation_brute: Collecting 100 samples in estimated 8.0340 s (200 iterations)
//! Benchmarking millionth_lexicographic_permutation_brute: Analyzing
//! millionth_lexicographic_permutation_brute
//!                         time:   [39.100 ms 39.383 ms 39.704 ms]
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high severe
//! mean   [39.100 ms 39.704 ms] std. dev.      [1.0884 ms 2.1053 ms]
//! median [38.592 ms 39.342 ms] med. abs. dev. [795.47 us 1.5771 ms]
//!
//! Benchmarking millionth_lexicographic_permutation_factorial
//! Benchmarking millionth_lexicographic_permutation_factorial: Warming up for 3.0000 s
//! Benchmarking millionth_lexicographic_permutation_factorial: Collecting 100 samples in estimated 5.0003 s (27845700 iterations)
//! Benchmarking millionth_lexicographic_permutation_factorial: Analyzing
//! millionth_lexicographic_permutation_factorial
//!                         time:   [179.99 ns 182.03 ns 184.02 ns]
//! Found 3 outliers among 100 measurements (3.00%)
//!   3 (3.00%) high mild
//! slope  [179.99 ns 184.02 ns] R^2            [0.8459767 0.8463638]
//! mean   [179.45 ns 181.99 ns] std. dev.      [5.2265 ns 7.6822 ns]
//! median [178.10 ns 180.03 ns] med. abs. dev. [3.6078 ns 6.8519 ns]
//! ```
//! See (m15)(./m15.rs)

use std::usize;

/// A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
///
/// 012   021   102   120   201   210
///
/// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
///
///
/// ```rust
/// use self::project_euler::m24::millionth_lexicographic_permutation_brute;
/// assert_eq!(millionth_lexicographic_permutation_brute(), 2783915460);
/// ```
#[allow(clippy::needless_range_loop)]
pub fn millionth_lexicographic_permutation_brute() -> u64 {
    let mut counter = 1u64;
    let check_digit_used = &mut [false; 10];
    for a in 0..=9 {
        if check_digit_used[a] {
            continue;
        }
        check_digit_used[a] = true;
        for b in 0..=9 {
            if check_digit_used[b] {
                continue;
            }
            check_digit_used[b] = true;
            for c in 0..=9 {
                if check_digit_used[c] {
                    continue;
                }
                check_digit_used[c] = true;
                for d in 0..=9 {
                    if check_digit_used[d] {
                        continue;
                    }
                    check_digit_used[d] = true;
                    for e in 0..=9 {
                        if check_digit_used[e] {
                            continue;
                        }
                        check_digit_used[e] = true;
                        for f in 0..=9 {
                            if check_digit_used[f] {
                                continue;
                            }
                            check_digit_used[f] = true;
                            for g in 0..=9 {
                                if check_digit_used[g] {
                                    continue;
                                }
                                check_digit_used[g] = true;
                                for h in 0..=9 {
                                    if check_digit_used[h] {
                                        continue;
                                    }
                                    check_digit_used[h] = true;
                                    for i in 0..=9 {
                                        if check_digit_used[i] {
                                            continue;
                                        }
                                        check_digit_used[i] = true;
                                        for j in 0..=9 {
                                            if check_digit_used[j] {
                                                continue;
                                            }
                                            check_digit_used[j] = true;
                                            if counter == 1_000_000 {
                                                let x = format!(
                                                    "{}{}{}{}{}{}{}{}{}{}",
                                                    a, b, c, d, e, f, g, h, i, j
                                                );
                                                return x.parse::<u64>().unwrap();
                                            }
                                            counter += 1;
                                            check_digit_used[j] = false;
                                        }
                                        check_digit_used[i] = false;
                                    }
                                    check_digit_used[h] = false;
                                }
                                check_digit_used[g] = false;
                            }
                            check_digit_used[f] = false;
                        }
                        check_digit_used[e] = false;
                    }
                    check_digit_used[d] = false;
                }
                check_digit_used[c] = false;
            }
            check_digit_used[b] = false;
        }
        check_digit_used[a] = false;
    }
    panic!();
}

///
/// A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
///
/// 012   021   102   120   201   210
///
/// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
///
///
///```rust
/// use self::project_euler::m24::millionth_lexicographic_permutation_factorial;
/// assert_eq!(millionth_lexicographic_permutation_factorial(), 2783915460);
///```
pub fn millionth_lexicographic_permutation_factorial() -> u64 {
    // n=3:[0,1,2] permutation of all is nPr = 3P3 = 3! = 3 * 2 * 1 = 6
    //
    // n=10:[0,1,2,3,4,5,6,7,8,9] permutation of all is 10*9*8*7*6*5*4*3*2*1 = 10! = 3,628,800
    // n=9: [x][any 9 digits]     permutation of all is    9*8*7*6*5*4*3*2*1 =  9! =   362,880
    // n=8                                                   8*7*6*5*4*3*2*1 =  8! = 9!/9 = 362,880/9 = 40,320

    // [1..3,628,800] in the list of permutation,
    // [0xxxxxxxxx:1..=362880,1xxxxxxxxx:362880+1..=362880*2,2xxxxxxxxx:362880*2+1..=362880*3,..,9xxxxxxxxx:362880*9+1..=362880*10]
    //                                                       <- 1_000_000 is in this range ->
    // So that the first digit is determined to be 2. Now n=9:[0,1,3,4,5,6,7,8,9] are left.
    //
    // [362880*2..=362880*3] has 9 sections = 40320*9.
    // [0xxxxxxxx:362880*2+1..=362880*2+40320,               // 0th
    //  1xxxxxxxx:362880*2+40320+1..=362880*2+40320*2,       // 1st
    //  3xxxxxxxx:362880*2+40320*2+1..=362880*2+40320*3,     // 2nd
    //  4xxxxxxxx:362880*2+40320*3+1..=362880*2+40320*4,     // 3rd
    //  5xxxxxxxx:362880*2+40320*4+1..=362880*2+40320*5,     // 4th
    //  6xxxxxxxx:362880*2+40320*5+1..=362880*2+40320*6,     // 5th
    //  7xxxxxxxx:362880*2+40320*6+1..=362880*2+40320*7,..]  // 6th /*[967681..=1008000]*/
    // 1_000_000 is in the 6th range of n=9:[0,1,3,4,5,6,7,8,9] counting from 0,
    // and so that the second digit is determined to be 7.
    //
    //  n=8:[0,1,3,4,5,6,8,9] are left and the range [362880*2+40320*6+1..=362880*2+40320*7] will be devidable by 8.
    fn factorial(n: u64) -> u64 {
        match n {
            0 | 1 => 1,
            _ => factorial(n - 1) * n,
        }
    }
    let mut answer: Vec<u8> = vec![];
    let mut digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut permutation_number = 0u64;
    for i in (0..digits.len()).rev() {
        let mut range = 0usize;
        let range_width = factorial(i as u64);
        while permutation_number < 1_000_000 {
            permutation_number += range_width;
            range += 1;
        }
        let d = digits.remove(range - 1);
        answer.push(d);
        permutation_number -= range_width;
    }
    answer.iter().fold(0u64, |sum, &d| sum * 10 + d as u64)
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
}

///```rust
/// use self::project_euler::m24::millionth_lexicographic_permutation_factorial_2;
/// assert_eq!(millionth_lexicographic_permutation_factorial_2(), 2783915460);
///```
pub fn millionth_lexicographic_permutation_factorial_2() -> u64 {
    // 0123456789 is 0th
    let mut reminder = 1_000_000u64 - 1;
    let mut items_with_order = vec![0u64, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut millionth_element = 0u64;
    for weight in (0..items_with_order.len()).rev() {
        let unit = factorial(weight as u64);
        let quot = reminder / unit;
        reminder -= quot * unit;
        println!("{} * {}; ...{}; {:?}", unit, quot, reminder, items_with_order);
        millionth_element *= 10;
        millionth_element += items_with_order.remove(quot as usize);
    }
    millionth_element
}
