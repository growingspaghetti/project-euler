//! ```                                                                                                                                                                                                 
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- index_of_fibonacci_sequence_to_contain_1000 --verbose --baseline new
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//! 
//!     Finished bench [optimized] target(s) in 13.82s
//!      Running target/release/deps/bench_main-6ef44f4b2c4834b9
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_brute
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_brute: Warming up for 3.0000 s
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_brute: Collecting 100 samples in estimated 5.2094 s (2300 iterations)
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_brute: Analyzing
//! index_of_fibonacci_sequence_to_contain_1000_digits_brute
//!                         time:   [2.2339 ms 2.2420 ms 2.2512 ms]
//!                         change: [-0.5449% +0.0000% +0.5293%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 6 outliers among 100 measurements (6.00%)
//!   3 (3.00%) high mild
//!   3 (3.00%) high severe
//! mean   [2.2339 ms 2.2512 ms] std. dev.      [31.455 us 55.586 us]
//! median [2.2201 ms 2.2331 ms] med. abs. dev. [15.503 us 31.494 us]
//! 
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio: Warming up for 3.0000 s
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio: Collecting 100 samples in estimated 5.0155 s (277750 iterations)
//! Benchmarking index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio: Analyzing
//! index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio
//!                         time:   [17.165 us 17.320 us 17.503 us]
//!                         change: [-4.1599% +0.0000% +4.1446%] (p = 1.00 > 0.05)
//!                         No change in performance detected.
//! Found 13 outliers among 100 measurements (13.00%)
//!   2 (2.00%) high mild
//!   11 (11.00%) high severe
//! slope  [17.165 us 17.503 us] R^2            [0.6722996 0.6698393]
//! mean   [18.121 us 19.222 us] std. dev.      [1.6798 us 4.1062 us]
//! median [17.670 us 18.265 us] med. abs. dev. [808.00 ns 1.4746 us]
//! ```
//!
//! See (m2)[./m2.rs] and [m13](./m13.rs). [m20](./m20.rs), [m16](./m16.rs)
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/1/1e/Fibonacci_tiling_of_the_plane_and_approximation_to_Golden_Ratio.gif/400px-Fibonacci_tiling_of_the_plane_and_approximation_to_Golden_Ratio.gif)
//!
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/0f9f0e030b8296d94804279eaef970c27581c0a8)
//!
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/ccab7a6cd419ca36abdddee5f576e9e63220f88f)
//!
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/de8aaf6a6b60f0f58cc274515efd7f177bd65802)
//!
//! ![](https://wikimedia.org/api/rest_v1/media/math/render/svg/a531a6c08c01aacc84fb8ab1311ab471c6b22820)

/// The Fibonacci sequence is defined by the recurrence relation:
///
/// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
///
/// Hence the first 12 terms will be:
///
/// F1 = 1
/// F2 = 1
/// F3 = 2
/// F4 = 3
/// F5 = 5
/// F6 = 8
/// F7 = 13
/// F8 = 21
/// F9 = 34
/// F10 = 55
/// F11 = 89
/// F12 = 144
///
/// The 12th term, F12, is the first term to contain three digits.
///
/// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
///
/// ```rust
/// use self::project_euler::m25::index_of_fibonacci_sequence_to_contain_1000_digits_brute;
/// assert_eq!(index_of_fibonacci_sequence_to_contain_1000_digits_brute(), 4782);
/// ```
pub fn index_of_fibonacci_sequence_to_contain_1000_digits_brute() -> u64 {
    fn add(a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut sum = if a.len() < b.len() {
            a.to_vec()
        } else {
            b.to_vec()
        };
        let longer_one = if a.len() < b.len() { b } else { a };
        // capacity difference
        for _ in 0..(longer_one.len() - sum.len()) {
            sum.push(0);
        }
        let mut carry = 0u64;
        for (i, &v) in longer_one.iter().enumerate() {
            sum[i] += v + carry;
            if sum[i] > 1_000_000_000 {
                carry = sum[i] / 1_000_000_000;
                sum[i] -= 1_000_000_000 * carry;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            sum.push(carry);
        }
        sum
    }

    let fib = |prepre: &[u64], pre: &[u64]| -> Vec<u64> {
        match (prepre, pre) {
            (&[0], &[0]) => vec![1u64], /* F1 */
            // (0, 1) => (1, vec[1])    /* F2 */,
            // (1, 1) => (1, vec[2])    /* F3 */,
            // (1, 2) => (2, vec[3])    /* F4 */,
            // (2, 3) => (3, vec[5])    /* F5 */,
            _ => add(prepre, pre),
        }
    };

    let mut prepre = vec![0u64];
    let mut pre = vec![0u64];
    let mut f = 1u64;
    loop {
        let n = fib(&prepre, &pre);
        prepre = pre;
        pre = n;
        // each container has 9 digits.
        // pre.len()=111 -> 999 digits
        if pre.len() > 111 {
            //[250627816, 29218636, 934512787, 384736771, 348473874, 820492520, 487983821, 159616877, 661581327, 753792891, 151619317, 576357666, 350944456, 688339109, 945940566, 491885830, 77614812, 207051164, 562917905, 660000723, 986297640, 297350643, 718693835, 288616808, 659868456, 113205765, 834864543, 309773217, 6894330, 863795048, 611008992, 435428212, 306083544, 428582425, 523425950, 584714548, 39447629, 113651657, 245493937, 186255550, 88000110, 190259846, 310184224, 488154256, 646513903, 665796581, 778373229, 919410967, 460579954, 595173518, 167755051, 773697023, 365532164, 912503231, 474103736, 393439617, 70098024, 766945691, 911707619, 357447470, 369480641, 124521900, 666185435, 895486902, 847068202, 175397706, 48260706, 602129934, 543229119, 551467837, 39506137, 559657452, 381792235, 612965931, 13201478, 491120525, 821825548, 726117715, 526447243, 290192870, 609723067, 1985432, 503439547, 113074699, 837148943, 553966422, 259186759, 666192149, 87579391, 273781753, 774442750, 433051234, 186144069, 748092482, 30461262, 959512996, 466672569, 957484336, 888177591, 657804694, 582489792, 621889147, 604694040, 375314520, 13235203, 632151665, 83683896, 457396885, 764980584, 382758936, 70066266, 1]
            //println!("{:?}", pre);
            break f;
        }
        f += 1;
    }
}

/// The Fibonacci sequence is defined by the recurrence relation:
///
/// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
///
/// Hence the first 12 terms will be:
///
/// F1 = 1
/// F2 = 1
/// F3 = 2
/// F4 = 3
/// F5 = 5
/// F6 = 8
/// F7 = 13
/// F8 = 21
/// F9 = 34
/// F10 = 55
/// F11 = 89
/// F12 = 144
///
/// The 12th term, F12, is the first term to contain three digits.
///
/// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
///
/// ```rust
/// use self::project_euler::m25::index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio;
/// assert_eq!(index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio(), 4782);
/// ```
pub fn index_of_fibonacci_sequence_to_contain_1000_digits_logarithm_1_618_golden_ratio() -> u64 {
    // F(n) is actually {((1+sqrt(5))/2)^n-((1-sqrt(5))/2)^n}/sqrt(5)
    //                    <----- a ------>  <------ b ----->  <- c ->
    // The second part, b, (1-sqrt(5))/2 is about 0.618.
    // So when n is so huge -((1-sqrt(5))/2)^n becomes practically 0:
    //   [0.618 -> 0.618*0.618=0.3819 -> 0.618*0.618*0.618=0.2360 -> 0.14 -> 0.09 ..]
    // Thus, F(n) =
    //   (1+sqrt(5))/2)^n/sqrt(5)
    //
    // F(n) >= 10^1000
    //   log10(F(n))                                >= log10(10^1000)
    //   log10((1+sqrt(5))/2)^n/sqrt(5))            >= 1000
    //   log10((1+sqrt(5))/2)^n)   - log10(sqrt(5)) >= 1000
    // n*log10((1+sqrt(5))/2)        - 0.5*log10(5) >= 1000
    // n*log10(1+sqrt(5))-n*log10(2) - 0.5*log10(5) >= 1000

    let mut n = 1f64;
    loop {
        let f = n * (1f64 + 5f64.sqrt()).log10() - n * 2f64.log10() - 0.5 * 5f64.log10();
        if f.ceil() as u32 > 999u32 {
            break n as u64;
        }
        n += 1f64;
    }
}
