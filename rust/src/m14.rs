//!```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- collatz_under_one_million_produces_the_longest_chain --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 12.83s
//!      Running target/release/deps/bench_main-e043d12d7fc498f6
//! Benchmarking collatz_under_one_million_produces_the_longest_chain
//! Benchmarking collatz_under_one_million_produces_the_longest_chain: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 21.6s, or reduce sample count to 20.
//! Benchmarking collatz_under_one_million_produces_the_longest_chain: Collecting 100 samples in estimated 21.620 s (100 iterations)
//! Benchmarking collatz_under_one_million_produces_the_longest_chain: Analyzing
//! collatz_under_one_million_produces_the_longest_chain
//!                         time:   [214.92 ms 215.79 ms 216.66 ms]
//!                         change: [-0.5618% +0.0000% +0.5701%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 3 outliers among 100 measurements (3.00%)
//!   3 (3.00%) high mild
//! mean   [214.92 ms 216.66 ms] std. dev.      [3.7882 ms 5.0590 ms]
//! median [215.06 ms 216.73 ms] med. abs. dev. [3.0104 ms 5.6405 ms]
//!
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 37.5s, or reduce sample count to 10.
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache: Collecting 100 samples in estimated 37.469 s (100 iterations)
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache: Analyzing
//! collatz_under_one_million_produces_the_longest_chain_cache
//!                         time:   [336.58 ms 345.88 ms 355.77 ms]
//!                         change: [-3.8259% +0.0000% +4.2278%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//! mean   [336.58 ms 355.77 ms] std. dev.      [41.719 ms 54.990 ms]
//! median [317.71 ms 330.17 ms] med. abs. dev. [18.865 ms 38.046 ms]
//!
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 50.5s, or reduce sample count to 10.
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec: Collecting 100 samples in estimated 50.541 s (100 iterations)
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec: Analyzing
//! collatz_under_one_million_produces_the_longest_chain_cache_rec
//!                         time:   [374.34 ms 384.61 ms 395.46 ms]
//!                         change: [-3.8364% +0.0000% +4.0405%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 2 outliers among 100 measurements (2.00%)
//!   2 (2.00%) high mild
//! mean   [374.34 ms 395.46 ms] std. dev.      [45.667 ms 61.854 ms]
//! median [353.16 ms 374.13 ms] med. abs. dev. [24.429 ms 52.224 ms]
//!
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 37.1s, or reduce sample count to 10.
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2: Collecting 100 samples in estimated 37.113 s (100 iterations)
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2: Analyzing
//! collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2
//!                         time:   [333.83 ms 343.39 ms 353.06 ms]
//!                         change: [-3.6643% +0.0000% +3.9291%] (p = 0.99 > 0.05)
//!                         No change in performance detected.
//! mean   [333.83 ms 353.06 ms] std. dev.      [43.987 ms 53.480 ms]
//! median [316.96 ms 353.48 ms] med. abs. dev. [40.920 ms 72.711 ms]
//!
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 57.8s, or reduce sample count to 10.
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000: Collecting 100 samples in estimated 57.849 s (100 iterations)
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000: Analyzing
//! collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000
//!                         time:   [575.21 ms 586.73 ms 599.10 ms]
//!                         change: [-2.9403% +0.0000% +2.8790%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 2 outliers among 100 measurements (2.00%)
//!   2 (2.00%) high mild
//! mean   [575.21 ms 599.10 ms] std. dev.      [48.739 ms 73.225 ms]
//! median [553.47 ms 577.25 ms] med. abs. dev. [23.773 ms 56.328 ms]
//!
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_full
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_full: Warming up for 3.0000 s
//!
//! Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.3s, or reduce sample count to 50.
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_full: Collecting 100 samples in estimated 9.2926 s (100 iterations)
//! Benchmarking collatz_under_one_million_produces_the_longest_chain_full: Analyzing
//! collatz_under_one_million_produces_the_longest_chain_full
//!                         time:   [94.612 ms 95.136 ms 95.678 ms]
//!                         change: [-0.7693% +0.0000% +0.8203%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! mean   [94.612 ms 95.678 ms] std. dev.      [2.3903 ms 3.0021 ms]
//! median [93.580 ms 95.483 ms] med. abs. dev. [1.9608 ms 3.9712 ms]
//!```
//! Hashing is slower than performing collatz calculations.
//! https://www.dcode.fr/collatz-conjecture

use std::usize;

///
///
/// The following iterative sequence is defined for the set of positive integers:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
///
///
///```rust
/// use self::project_euler::m14::collatz_under_one_million_produces_the_longest_chain;
/// assert_eq!(collatz_under_one_million_produces_the_longest_chain(), (837799, 525));
///```
pub fn collatz_under_one_million_produces_the_longest_chain() -> (u64, u64) {
    fn collatz(num: u64) -> Option<u64> {
        if num == 1 {
            None
        } else if num % 2 == 0 {
            Some(num / 2)
        } else {
            Some(3 * num + 1)
        }
    };
    let mut longest_step = 0u64;
    let mut max_num = 0u64;
    for i in 1..1_000_000u64 {
        let mut step = 0u64;
        let mut to_compute = i;
        while let Some(n) = collatz(to_compute) {
            step += 1;
            to_compute = n;
        }
        step += 1; // last 4 -> 2 -> "1"
        if longest_step < step {
            longest_step = step;
            max_num = i;
        }
    }
    (max_num, longest_step)
}

///
///
/// The following iterative sequence is defined for the set of positive integers:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
///
///
///```rust
/// use self::project_euler::m14::collatz_under_one_million_produces_the_longest_chain_cache;
/// assert_eq!(collatz_under_one_million_produces_the_longest_chain_cache(), (837799, 525));
///```
pub fn collatz_under_one_million_produces_the_longest_chain_cache() -> (u64, u64) {
    fn collatz(num: u64) -> Option<u64> {
        if num == 1 {
            None
        } else if num % 2 == 0 {
            Some(num / 2)
        } else {
            Some(3 * num + 1)
        }
    };
    use std::collections::HashMap;
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(1_000_000);
    let mut longest_step = 0u64;
    let mut max_num = 0u64;
    for i in 1..1_000_000u64 {
        let mut step = 0u64;
        let mut to_compute = i;
        while let Some(n) = collatz(to_compute) {
            if let Some(recorded_step) = map.get(&n) {
                step += recorded_step;
                break;
            }
            step += 1;
            to_compute = n;
        }
        step += 1; // last 4 -> 2 -> "1"
        map.insert(i, step);
        if longest_step < step {
            longest_step = step;
            max_num = i;
        }
    }
    (max_num, *map.get(&max_num).unwrap())
}

///
///
/// The following iterative sequence is defined for the set of positive integers:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
///
///
///```rust
/// use self::project_euler::m14::collatz_under_one_million_produces_the_longest_chain_cache_rec;
/// assert_eq!(collatz_under_one_million_produces_the_longest_chain_cache_rec(), (837799, 525));
///```
pub fn collatz_under_one_million_produces_the_longest_chain_cache_rec() -> (u64, u64) {
    use std::collections::HashMap;
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(1_000_000);
    map.insert(1, 1);

    fn collatz(num: u64, reg: &mut HashMap<u64, u64>) -> (Option<u64>, u64) {
        if num == 1 {
            (None, 1)
        } else if num % 2 == 0 {
            let next = num / 2;
            if let Some(&recorded_step) = reg.get(&next) {
                (None, recorded_step)
            } else {
                let c = collatz(next, reg);
                //reg.insert(next, c.1 + 1);
                (c.0, c.1 + 1)
            }
        } else {
            let next = 3 * num + 1;
            if let Some(&recorded_step) = reg.get(&next) {
                (None, recorded_step)
            } else {
                let c = collatz(next, reg);
                //reg.insert(next, c.1 + 1);
                (c.0, c.1 + 1)
            }
        }
    };

    let mut longest_step = 0u64;
    let mut max_num = 0u64;
    for i in 2..1_000_000u64 {
        let (_, step) = collatz(i, &mut map);
        map.insert(i, step + 1);
        if longest_step < step {
            longest_step = step;
            max_num = i;
        }
    }
    (max_num, *map.get(&max_num).unwrap())
}

#[test]
fn main() {
    assert_eq!(
        collatz_under_one_million_produces_the_longest_chain_cache_rec(),
        (837799, 525)
    );
}

///
///
/// The following iterative sequence is defined for the set of positive integers:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
///
///
///```rust
/// use self::project_euler::m14::collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2;
/// assert_eq!(collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2(), (837799, 525));
///```
pub fn collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2() -> (u64, u64) {
    use std::collections::HashMap;
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(1_000_000);
    map.insert(1, 1);

    fn collatz(num: u64, reg: &mut HashMap<u64, u64>) -> (Option<u64>, u64) {
        if num == 1 {
            (None, 1)
        } else if num % 2 == 0 {
            let next = num / 2;
            if let Some(&recorded_step) = reg.get(&next) {
                (None, recorded_step)
            } else {
                let c = collatz(next, reg);
                //reg.insert(next, c.1 + 1);
                (c.0, c.1 + 1)
            }
        } else {
            // when n%2 != 0, collatz(3n+1) proceeds to collatz(3n+1/2) as (3n+1)%2=0
            //      then collatz(3n+1/2%2=0 ? 3n+1/2/2 : 3(3n+1/2)+1)
            // giving collatz(3n+1/2) directly instead of collatz(3n+1), one step was skipped.
            let next = (3 * num + 1) / 2;
            if let Some(&recorded_step) = reg.get(&next) {
                (None, recorded_step)
            } else {
                let c = collatz(next, reg);
                //reg.insert(next, c.1 + 1);
                (c.0, c.1 + 2 /* 1+1 for collatz(3n+1) was skipped */)
            }
        }
    };

    let mut longest_step = 0u64;
    let mut max_num = 0u64;
    for i in 2..1_000_000u64 {
        let (_, step) = collatz(i, &mut map);
        map.insert(i, step + 1);
        if longest_step < step {
            longest_step = step;
            max_num = i;
        }
    }
    (max_num, *map.get(&max_num).unwrap())
}

///
///
/// The following iterative sequence is defined for the set of positive integers:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
///
///
///```rust
/// use self::project_euler::m14::collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000;
/// assert_eq!(collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000(), (837799, 525));
///```
pub fn collatz_under_one_million_produces_the_longest_chain_cache_rec_3n_1_2_skip_500_000(
) -> (u64, u64) {
    use std::collections::HashMap;
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(1_000_000);
    map.insert(1, 1);

    fn collatz(num: u64, reg: &mut HashMap<u64, u64>) -> (Option<u64>, u64) {
        if num == 1 {
            (None, 1)
        } else if num % 2 == 0 {
            let next = num / 2;
            if let Some(&recorded_step) = reg.get(&next) {
                (None, recorded_step)
            } else {
                let c = collatz(next, reg);
                //reg.insert(next, c.1 + 1);
                (c.0, c.1 + 1)
            }
        } else {
            let next = (3 * num + 1) / 2;
            if let Some(&recorded_step) = reg.get(&next) {
                (None, recorded_step)
            } else {
                let c = collatz(next, reg);
                //reg.insert(next, c.1 + 1);
                (c.0, c.1 + 2)
            }
        }
    };

    // when number is i%2=0 [2, 4, 6 .. 500_000 .. 999998],
    // condition num%2=0, collatz(num) proceeds to collatz(num/2)
    //      and steps are         a+1         <-               a
    // therefore,         collatz(2) is 1 more step advanced than collatz(1),
    //                    collatz(4) is 1 more step advanced than collatz(2),
    //              collatz(500_000) is 1 more step advanced than collatz(250_000),
    //            collatz(1_000_000) is 1 more step advanced than collatz(500_000).
    // for we consider collatz(999_999), even numbers from 2..999998/2 are ruled out.
    //  it's true for the rest as well.
    //               collatz(999998) is 1 more step advanced than collatz(499999).
    let mut longest_step = 0u64;
    let mut max_num = 0u64;
    for i in 500_000..1_000_000u64 {
        let (_, step) = collatz(i, &mut map);
        map.insert(i, step + 1);
        if longest_step < step {
            longest_step = step;
            max_num = i;
        }
    }
    (max_num, *map.get(&max_num).unwrap())
}

///
///
/// The following iterative sequence is defined for the set of positive integers:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
///
///
///```rust
/// use self::project_euler::m14::collatz_under_one_million_produces_the_longest_chain_full;
/// assert_eq!(collatz_under_one_million_produces_the_longest_chain_full(), (837799, 525));
///```
pub fn collatz_under_one_million_produces_the_longest_chain_full() -> (u64, u64) {
    fn collatz(num: u64) -> (Option<u64>, u64) {
        if num == 1 {
            (None, 1)
        } else if num % 2 == 0 {
            (Some(num / 2), 1)
        } else {
            (Some((3 * num + 1) / 2), 2)
        }
    }
    let mut longest_step = 0u64;
    let mut max_num = 0u64;
    for i in 500_000..1_000_000u64 {
        let mut step = 0u64;
        let mut to_compute = i;
        'inner: loop {
            let (next, tmp_step) = collatz(to_compute);
            step += tmp_step;
            if let Some(n) = next {
                to_compute = n;
            } else {
                break 'inner;
            }
        }
        if longest_step < step {
            longest_step = step;
            max_num = i;
        }
    }
    (max_num, longest_step)
}

struct Position {
    initial_number: u64,
    number_of_steps: u32,
}

struct Collatz {
    init_num: u64,
    next: Option<u64>,
    step_accum: u32,
    cache: Vec<u32>,
    max: Position,
}

impl Collatz {
    fn update_records(&mut self) {
        if let Some(s) = self.cache.get_mut(self.init_num as usize) {
            *s = self.step_accum;
        }
        if self.max.number_of_steps < self.step_accum {
            self.max.number_of_steps = self.step_accum;
            self.max.initial_number = self.init_num;
        }
    }
    fn move_along(&mut self) -> Option<u64> {
        let n = self.next?;
        if n == 1 {
            self.step_accum += 1;
            self.next = None;
            self.update_records();
            return self.next;
        }
        match self.cache.get(n as usize) {
            Some(r) if *r != 0 => {
                self.step_accum += 1 + *r;
                self.next = None;
                self.update_records();
                return self.next;
            }
            _ => (),
        }
        if n % 2 != 0 {
            self.step_accum += 2;
            self.next = Some((3 * n + 1) / 2);
            return self.next;
        }
        self.step_accum += 1;
        self.next = Some(n / 2);
        self.next
    }
    fn init_next(&mut self) {
        self.next = if self.init_num % 2 == 0 {
            Some(self.init_num / 2)
        } else {
            Some(3 * self.init_num + 1)
        }
    }
}

// 43.6 ms
///```rust
/// use self::project_euler::m14::collatz_under_one_million_produces_the_longest_chain_full_2;
/// assert_eq!(collatz_under_one_million_produces_the_longest_chain_full_2(), (837799, 525));
///```
pub fn collatz_under_one_million_produces_the_longest_chain_full_2() -> (u64, u32) {
    let mut c = Collatz {
        init_num: 0,
        next: None,
        step_accum: 0,
        cache: vec![0; 1_000_000],
        max: Position {
            initial_number: 1,
            number_of_steps: 0,
        },
    };

    for i in 500_000..1_000_000u64 {
        c.init_num = i;
        c.init_next();
        c.step_accum = 0;
        while c.move_along().is_some() {}
    }

    (c.max.initial_number, c.max.number_of_steps + 1)
}
// let mut longest_number_of_steps = 0u32;
// let mut max_num = 0u64;
// for (i, &v) in c.cache.iter().enumerate() {
//     if v > longest_number_of_steps {
//         longest_number_of_steps = v;
//         max_num = i as u64;
//     }
// }
// (max_num, longest_number_of_steps + 1)
