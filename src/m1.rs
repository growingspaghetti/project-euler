/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// ```rust
/// use self::project_euler::m1::sum_of_all_the_multiples_of_3_or_5_below_1000;
/// assert_eq!(sum_of_all_the_multiples_of_3_or_5_below_1000(), 233168);
/// ```
pub fn sum_of_all_the_multiples_of_3_or_5_below_1000() -> i32 {
    let mut sum = 0;
    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    sum
}

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// ```rust
/// use self::project_euler::m1::sum_of_all_the_multiples_of_3_or_5_below_1000_iter;
/// assert_eq!(sum_of_all_the_multiples_of_3_or_5_below_1000_iter(), 233168);
/// ```
pub fn sum_of_all_the_multiples_of_3_or_5_below_1000_iter() -> i32 {
    (0..1000).filter(|&x| x % 5 == 0 || x % 3 == 0).sum()
}

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
///
/// ```rust
/// use self::project_euler::m1::sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series;
/// assert_eq!(sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series(), 233168);
/// ```
pub fn sum_of_all_the_multiples_of_3_or_5_below_1000_arithmetic_series() -> i32 {
    // 0  3  6  9 .. 999 = 3 * [0 1 2 3 ... 333] = 3 * (0+333 #doubling) * 334#n / 2#halving
    // 0  5 10 15 .. 995 = 5 * [0 1 2 3 ... 199] = 5 * (0+199 #doubling) * 200#n / 2#halving
    // 0 15 30 45 .. 999-999%15#last = 15 * [0 1 2 3 ... last/15]
    let arithmetic_series = |common_difference: i32| -> i32 {
        let last = 999 - 999 % common_difference;
        let n = last / common_difference + 1;
        last * n / 2
    };
    arithmetic_series(3) + arithmetic_series(5) - arithmetic_series(15)
}
