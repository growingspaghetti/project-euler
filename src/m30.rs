/// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
///
/// 1634 = 1^4 + 6^4 + 3^4 + 4^4
/// 8208 = 8^4 + 2^4 + 0^4 + 8^4
/// 9474 = 9^4 + 4^4 + 7^4 + 4^4
///
/// As 1 = 14 is not a sum it is not included.
///
/// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
///
/// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
///
/// ```rust
/// use self::project_euler::m30::sum_of_fifth_powers;
/// assert_eq!(sum_of_fifth_powers(4), 19316);
/// assert_eq!(sum_of_fifth_powers(5), 443839);
/// ```
pub fn sum_of_fifth_powers(exp: u32) -> u32 {
    //      n |   delta in n | sum of fifth power
    //      9 |           +0 |         9^5 = 59049
    //     99 |          +90 |     9^5+9^5 = above + 59049
    //    999 |         +900 | 9^5+9+5+9^5 = above + 59049
    //   9999          +9000
    //  99999         +90000 > +59049
    // 100000 > 59049
    // 999999        +900000 > +59049
    //1000000
    let max = 10i32.pow((9i32.pow(exp) as f32).log10().ceil() as u32 + 1) as u32;
    let mut total = 0u32;
    for n in 2..max {
        let mut tmp = n;
        let mut sum = 0u32;
        while tmp > 0 {
            sum += (tmp % 10).pow(exp);
            tmp /= 10;
        }
        if n == sum {
            total += sum;
        }
    }
    total
}
