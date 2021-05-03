//! ```
//! ryoji@ubuntu:/media/dev/project-euler$ cargo bench --bench bench_main -- thirteen_adjacent_digits --verbose
//!    Compiling project-euler v0.1.0 (/media/dev/project-euler)
//!     Finished bench [optimized] target(s) in 7.88s
//!      Running target/release/deps/bench_main-67a8f1edf18141ba
//! Benchmarking thirteen_adjacent_digits
//! Benchmarking thirteen_adjacent_digits: Warming up for 3.0000 s
//! Benchmarking thirteen_adjacent_digits: Collecting 100 samples in estimated 5.0058 s (772650 iterations)
//! Benchmarking thirteen_adjacent_digits: Analyzing
//! thirteen_adjacent_digits
//!                         time:   [6.3956 us 6.4163 us 6.4397 us]
//! Found 6 outliers among 100 measurements (6.00%)
//!   3 (3.00%) high mild
//!   3 (3.00%) high severe
//! slope  [6.3956 us 6.4397 us] R^2            [0.9611173 0.9607303]
//! mean   [6.4599 us 6.5906 us] std. dev.      [156.35 ns 507.14 ns]
//! median [6.3958 us 6.4600 us] med. abs. dev. [77.707 ns 158.84 ns]
//!
//! Benchmarking thirteen_adjacent_digits_byte_str
//! Benchmarking thirteen_adjacent_digits_byte_str: Warming up for 3.0000 s
//! Benchmarking thirteen_adjacent_digits_byte_str: Collecting 100 samples in estimated 5.0672 s (348450 iterations)
//! Benchmarking thirteen_adjacent_digits_byte_str: Analyzing
//! thirteen_adjacent_digits_byte_str
//!                         time:   [14.437 us 14.486 us 14.543 us]
//! Found 7 outliers among 100 measurements (7.00%)
//!   4 (4.00%) high mild
//!   3 (3.00%) high severe
//! slope  [14.437 us 14.543 us] R^2            [0.9592471 0.9587423]
//! mean   [14.564 us 14.768 us] std. dev.      [310.89 ns 740.28 ns]
//! median [14.449 us 14.632 us] med. abs. dev. [220.11 ns 363.71 ns]
//! ```

/// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
///
/// 73167176531330624919225119674426574742355349194934
/// 96983520312774506326239578318016984801869478851843
/// 85861560789112949495459501737958331952853208805511
/// 12540698747158523863050715693290963295227443043557
/// 66896648950445244523161731856403098711121722383113
/// 62229893423380308135336276614282806444486645238749
/// 30358907296290491560440772390713810515859307960866
/// 70172427121883998797908792274921901699720888093776
/// 65727333001053367881220235421809751254540594752243
/// 52584907711670556013604839586446706324415722155397
/// 53697817977846174064955149290862569321978468622482
/// 83972241375657056057490261407972968652414535100474
/// 82166370484403199890008895243450658541227588666881
/// 16427171479924442928230863465674813919123162824586
/// 17866458359124566529476545682848912883142607690042
/// 24219022671055626321111109370544217506941658960408
/// 07198403850962455444362981230987879927244284909188
/// 84580156166097919133875499200524063689912560717606
/// 05886116467109405077541002256983155200055935729725
/// 71636269561882670428252483600823257530420752963450
///
/// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
///
/// ```rust
/// use self::project_euler::m8::thirteen_adjacent_digits;
/// let s = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
/// assert_eq!(thirteen_adjacent_digits(s), 23514624000);
/// ```
pub fn thirteen_adjacent_digits(s: &str) -> u64 {
    let digits = s
        .chars()
        .map(|u| u.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();
    let mut max = 0u64;
    for i in 13..digits.len() - 13 {
        let tmp = digits[i..i + 13].iter().product();
        if tmp > max {
            max = tmp;
        }
    }
    max
}

/// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
///
/// 73167176531330624919225119674426574742355349194934
/// 96983520312774506326239578318016984801869478851843
/// 85861560789112949495459501737958331952853208805511
/// 12540698747158523863050715693290963295227443043557
/// 66896648950445244523161731856403098711121722383113
/// 62229893423380308135336276614282806444486645238749
/// 30358907296290491560440772390713810515859307960866
/// 70172427121883998797908792274921901699720888093776
/// 65727333001053367881220235421809751254540594752243
/// 52584907711670556013604839586446706324415722155397
/// 53697817977846174064955149290862569321978468622482
/// 83972241375657056057490261407972968652414535100474
/// 82166370484403199890008895243450658541227588666881
/// 16427171479924442928230863465674813919123162824586
/// 17866458359124566529476545682848912883142607690042
/// 24219022671055626321111109370544217506941658960408
/// 07198403850962455444362981230987879927244284909188
/// 84580156166097919133875499200524063689912560717606
/// 05886116467109405077541002256983155200055935729725
/// 71636269561882670428252483600823257530420752963450
///
/// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
///
/// ```rust
/// use self::project_euler::m8::thirteen_adjacent_digits_byte_str;
/// let s = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
/// assert_eq!(thirteen_adjacent_digits_byte_str(s), 23514624000);
/// ```
pub fn thirteen_adjacent_digits_byte_str(s: &str) -> u64 {
    let digits = s
        .bytes()
        .map(|b| std::str::from_utf8(&[b]).unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut max = 0u64;
    for i in 13..digits.len() - 13 {
        let tmp = digits[i..i + 13].iter().product();
        if tmp > max {
            max = tmp;
        }
    }
    max
}

///
/// ```rust
/// use self::project_euler::m8::thirteen_adjacent_digits_byte_ascii;
/// let s = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
/// assert_eq!(thirteen_adjacent_digits_byte_ascii(s), 23514624000);
/// ```
pub fn thirteen_adjacent_digits_byte_ascii(s: &str) -> u64 {
    let zero = '0' as u64;
    let digits = s.chars().map(|c| c as u64 - zero).collect::<Vec<u64>>();
    // let mut max = 0u64;
    // for i in 0..digits.len() - 13 {
    //     let tmp = digits[i..i + 13].iter().product();
    //     if tmp > max {
    //         max = tmp;
    //     }
    // }
    (0..digits.len() - 13)
        .into_iter()
        .map(|i| digits[i..i + 13].iter().product())
        .reduce(|max, tmp| std::cmp::max(max, tmp))
        .unwrap()
    //max
}

///
/// ```rust
/// use self::project_euler::m8::thirteen_adjacent_digits_byte_ascii_rayon;
/// let s = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
/// assert_eq!(thirteen_adjacent_digits_byte_ascii_rayon(s), 23514624000);
/// ```
pub fn thirteen_adjacent_digits_byte_ascii_rayon(s: &str) -> u64 {
    use rayon::prelude::*;
    let zero = '0' as u64;
    let digits = s.chars().map(|c| c as u64 - zero).collect::<Vec<u64>>();
    (0..digits.len() - 13)
        .into_par_iter()
        .map(|i| digits[i..i + 13].iter().product())
        .reduce(|| 0, |max, tmp| std::cmp::max(max, tmp))
}

/// ```rust
/// use self::project_euler::m8::thirteen_adjacent_digits_byte_ascii_memo;
/// let s = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
/// assert_eq!(thirteen_adjacent_digits_byte_ascii_memo(s), 23514624000);
/// ```
pub fn thirteen_adjacent_digits_byte_ascii_memo(s: &str) -> u64 {
    let zero = '0' as u64;
    let digits = s.chars().map(|c| c as u64 - zero).collect::<Vec<u64>>();
    let mut max = 0u64;
    let mut memo = digits[0..12].iter().product();
    for i in 12..digits.len() {
        let tmp = memo * digits[i];
        max = std::cmp::max(max, tmp);
        memo = if digits[i - 12] == 0 {
            digits[i - 11..=i].iter().product()
        } else {
            tmp / digits[i - 12]
        };
    }
    max
}

/// ```rust
/// use self::project_euler::m8::thirteen_adjacent_digits_byte_ascii_memo_arr;
/// let s = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
/// assert_eq!(thirteen_adjacent_digits_byte_ascii_memo_arr(s), 23514624000);
/// ```
pub fn thirteen_adjacent_digits_byte_ascii_memo_arr(s: &str) -> u64 {
    let zero = '0' as u64;
    let vec = s.chars().map(|c| c as u64 - zero).collect::<Vec<u64>>();
    let arr = vec.as_slice();
    let mut max = 0u64;
    let mut memo = arr[0..12].iter().product();
    for i in 12..arr.len() {
        let tmp = memo * arr[i];
        if tmp > max {
            max = tmp;
        }
        memo = if arr[i - 12] == 0 {
            (&arr[i - 11..=i]).iter().product()
        } else {
            tmp / arr[i - 12]
        };
    }
    max
}
