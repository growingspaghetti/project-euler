//! ```txt
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- the_sum_of_the_digits_of_the_number_2_1000 --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!
//!     Finished bench [optimized] target(s) in 12.83s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000: Warming up for 3.0000 s
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000: Collecting 100 samples in estimated 5.7839 s (20200 iterations)
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000: Analyzing
//! the_sum_of_the_digits_of_the_number_2_1000
//!                         time:   [283.34 us 284.56 us 285.95 us]
//!                         change: [-1.2851% +0.0000% +1.2948%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 6 outliers among 100 measurements (6.00%)
//!   3 (3.00%) high mild
//!   3 (3.00%) high severe
//! slope  [283.34 us 285.95 us] R^2            [0.9390493 0.9383370]
//! mean   [285.41 us 290.60 us] std. dev.      [6.8072 us 19.416 us]
//! median [282.32 us 285.49 us] med. abs. dev. [3.2318 us 6.8528 us]
//!
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_carry_bool
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_carry_bool: Warming up for 3.0000 s
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_carry_bool: Collecting 100 samples in estimated 7.3790 s (15150 iterations)
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_carry_bool: Analyzing
//! the_sum_of_the_digits_of_the_number_2_1000_carry_bool
//!                         time:   [483.19 us 484.29 us 485.49 us]
//!                         change: [-0.6183% +0.0000% +0.6416%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   4 (4.00%) high severe
//! slope  [483.19 us 485.49 us] R^2            [0.9871628 0.9870360]
//! mean   [485.38 us 489.56 us] std. dev.      [7.0898 us 13.961 us]
//! median [482.54 us 484.80 us] med. abs. dev. [3.3513 us 7.2363 us]
//!
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula: Warming up for 3.0000 s
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula: Collecting 100 samples in estimated 5.7724 s (20200 iterations)
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula: Analyzing
//! the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula
//!                         time:   [283.13 us 284.10 us 285.21 us]
//!                         change: [-0.8212% +0.0000% +0.8364%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 9 outliers among 100 measurements (9.00%)
//!   5 (5.00%) high mild
//!   4 (4.00%) high severe
//! slope  [283.13 us 285.21 us] R^2            [0.9699036 0.9694045]
//! mean   [284.80 us 288.14 us] std. dev.      [5.4569 us 11.684 us]
//! median [282.56 us 284.59 us] med. abs. dev. [2.5724 us 4.9776 us]
//!
//!
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_u64
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_u64: Warming up for 3.0000 s
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_u64: Collecting 100 samples in estimated 5.0500 s (90900 iterations)
//! Benchmarking the_sum_of_the_digits_of_the_number_2_1000_u64: Analyzing
//! the_sum_of_the_digits_of_the_number_2_1000_u64
//!                         time:   [55.296 us 55.454 us 55.633 us]
//! Found 8 outliers among 100 measurements (8.00%)
//!   3 (3.00%) high mild
//!   5 (5.00%) high severe
//! slope  [55.296 us 55.633 us] R^2            [0.9742054 0.9739005]
//! mean   [55.532 us 56.144 us] std. dev.      [1.0392 us 2.0781 us]
//! median [55.131 us 55.380 us] med. abs. dev. [362.65 ns 727.26 ns]
//! ```
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/e2ff173bf9e26cc3f8eafa25f0d4d99f14491334)
//! See [m13](./m13.rs)

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
///
///```rust
/// use self::project_euler::m16::the_sum_of_the_digits_of_the_number_2_1000;
/// assert_eq!(the_sum_of_the_digits_of_the_number_2_1000(), 1366);
///```
pub fn the_sum_of_the_digits_of_the_number_2_1000() -> u32 {
    fn multiply_by_two(input: &mut std::vec::Vec<u8>) {
        let mut carry = 0u8;
        for i in (0..input.len()).rev() {
            let double = input[i] * 2 + carry;
            input[i] = double % 10;
            carry = double / 10;
        }
        if carry > 0 {
            input.insert(0, carry /* note, always one */);
        }
    }
    let mut number: Vec<u8> = vec![1 /* 2^0 */];
    for _ in 1..=1000 {
        multiply_by_two(&mut number);
    }
    number.iter().map(|&n| n as u32).sum()
}

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
///
///```rust
/// use self::project_euler::m16::the_sum_of_the_digits_of_the_number_2_1000_carry_bool;
/// assert_eq!(the_sum_of_the_digits_of_the_number_2_1000_carry_bool(), 1366);
///```
pub fn the_sum_of_the_digits_of_the_number_2_1000_carry_bool() -> u32 {
    fn multiply_by_two(input: &mut std::vec::Vec<u8>) {
        let mut carry = false;
        for i in (0..input.len()).rev() {
            let double = input[i] * 2 + if carry { 1 } else { 0 };
            input[i] = double % 10;
            carry = double / 10 > 0;
        }
        if carry {
            input.insert(0, 1);
        }
    }
    let mut number: Vec<u8> = vec![1 /* 2^0 */];
    for _ in 1..=1000 {
        multiply_by_two(&mut number);
    }
    number.iter().map(|&n| n as u32).sum()
}

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
///
///```rust
/// use self::project_euler::m16::the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula;
/// assert_eq!(the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula(), 1366);
///```
pub fn the_sum_of_the_digits_of_the_number_2_1000_change_of_base_formula() -> u32 {
    // https://en.wikipedia.org/wiki/List_of_logarithmic_identities#Changing_the_base
    // log10(2^1000) = log2(2^1000)/log2(10)
    //               = 1000 / log2(10)
    //               = 1000 / (log10(10)/log10(2))
    //               = 1000 * log10(2)/log10(10)
    //               = 1000 * log10(2) to calculate, it becomes 301. answer N = a * 10^301 (for a*10^0..=301, 302 places)
    fn multiply_by_two(input: &mut std::vec::Vec<u8>) {
        let mut carry = 0u8;
        for i in (0..input.len()).rev() {
            let double = input[i] * 2 + carry;
            input[i] = double % 10;
            carry = double / 10;
        }
        if carry > 0 {
            input.insert(0, carry /* note, always one */);
        }
    }
    let number_of_decimal_places = (1000f32 * 2f32.log10()) as usize + 1;
    let mut number = Vec::with_capacity(number_of_decimal_places);
    number.push(1);
    for _ in 1..=1000 {
        multiply_by_two(&mut number);
    }
    number.iter().map(|&n| n as u32).sum()
}

/// ```rust
/// use self::project_euler::m16::the_sum_of_the_digits_of_the_number_2_1000_u64;
/// assert_eq!(the_sum_of_the_digits_of_the_number_2_1000_u64(), 1366);
/// ```
pub fn the_sum_of_the_digits_of_the_number_2_1000_u64() -> u64 {
    fn multiply_by_two(containers: &mut std::vec::Vec<u64>) {
        let mut carry = 0u64;
        for i in 0..containers.len() {
            containers[i] = containers[i] * 2 + carry;
            if containers[i] > 1_000_000_000_000_000_000 {
                // use the fact that carry is always 1 and never be any other numbers
                containers[i] -= 1_000_000_000_000_000_000;
                carry = 1;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            containers.push(1);
        }
    }

    let mut containers: Vec<u64> = vec![];
    containers.push(1);

    let mut sum = 0u64;
    for _ in 1..=1000 {
        multiply_by_two(&mut containers);
    }
    for &container in &containers {
        let mut tmp = container;
        while tmp > 0 {
            let d = tmp % 10;
            sum += d;
            tmp /= 10;
        }
    }
    // [386837205668069376, 167660429831652624, 914571196477686542, 559165543946077062, 983581941267398767, 877954182153046474, 421074605062371141, 934567774824230985, 984577574698574803, 871452856923140435, 946729175531468251, 983788156958581275, 510511249361224931, 336074437503883703, 18105614048117055, 673209484250490600, 10715086071862]
    // println!("{:?}", containers);
    sum
}

// Rust tip.
// https://stackoverflow.com/a/59123630
// https://stackoverflow.com/questions/57637354/how-to-run-for-loop-on-elements-of-a-vector-and-change-the-vector-inside-the-for
// for container in containers {
// 	*container = *container * 2 + carry;
// 	if *container > 10_000_000_000 {
// 		// use the fact that carry is always 1 and never be any other numbers
// 		*container -= 10_000_000_000;
// 		carry = 1;
// 	} else {
// 		carry = 0;
// 	}
// }
// if carry > 0 { // error here
// 	containers.push(1);
// }
