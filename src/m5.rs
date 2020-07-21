/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
/// ```rust
/// use self::project_euler::m5::smallest_positive_number_that_is_evenly_divisible_by_each_20_brute;
/// assert_eq!(smallest_positive_number_that_is_evenly_divisible_by_each_20_brute(), 232792560);
/// ```
pub fn smallest_positive_number_that_is_evenly_divisible_by_each_20_brute() -> u32 {
    let mut num = 1;
    while !(2..=20).all(|x| num % x == 0) {
        num += 1;
    }
    num
}
