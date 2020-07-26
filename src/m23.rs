//! See [m21](./m21.rs)

///
/// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
///
/// A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
///
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
///
/// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
///
///
///
/// ```rust
/// use self::project_euler::m23::sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers;
/// assert_eq!(sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers(), 4179871);
/// ```
pub fn sum_of_integers_which_cannot_be_written_as_the_sum_of_two_abundant_numbers() -> u64 {
    fn proper_divisors_sigma_one(num: u64) -> u64 {
        if num < 2 {
            panic!()
        }
        crate::m21::divisor_function_sigma_one_function(num) - num
    };

    let mut abundant_numbers: Vec<u64> = vec![];
    for a in 2..28123u64 {
        let d = proper_divisors_sigma_one(a);
        if d > a {
            abundant_numbers.push(a);
        }
    }

    let check_array = &mut [false; 28123];
    abundant_numbers
        .iter()
        .for_each(|&d| check_array[d as usize] = true);

    let mut sum = 0u64;
    fn is_sum_of_two_abundant_numbers(
        n: u64,
        abundant_numbers: &[u64],
        check_array: &[bool],
    ) -> bool {
        for &a in abundant_numbers {
            // n = sum(a,b) || sum(b,a)
            // when a reached the half point of n, the other half of n is already pair searched.
            if n / 2 < a {
                break;
            }
            let b = n - a;
            if check_array[b as usize] {
                return true;
            }
        }
        false
    }
    for i in 1..28123u64 {
        if !is_sum_of_two_abundant_numbers(i, &abundant_numbers, check_array) {
            sum += i;
        }
    }
    sum
}
