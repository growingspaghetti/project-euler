//! See [m16](./m16.rs)

/// n! means n × (n − 1) × ... × 3 × 2 × 1
///
/// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
/// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
///
/// Find the sum of the digits in the number 100!
///
/// ```rust
/// use self::project_euler::m20::sum_of_the_digits_in_the_number_factorial_100;
/// assert_eq!(sum_of_the_digits_in_the_number_factorial_100(), 6485);
/// ```
#[allow(clippy::needless_range_loop)]
pub fn sum_of_the_digits_in_the_number_factorial_100() -> u64 {
    fn multiply_by_num(containers: &mut std::vec::Vec<u64>, num: u64) {
        let mut carry = 0u64;
        for i in 0..containers.len() {
            containers[i] = containers[i] * num + carry;
            if containers[i] > 1_000_000_000_000_000 {
                carry = containers[i] / 1_000_000_000_000_000;
                containers[i] -= 1_000_000_000_000_000 * carry;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            containers.push(carry);
        }
    }
    let mut containers: Vec<u64> = vec![];
    containers.push(1);

    let mut sum = 0u64;
    for i in 0..100u64 {
        // println!("{} {:?}", 100 - i, containers);
        multiply_by_num(&mut containers, 100 - i);
    }
    for &container in &containers {
        let mut tmp = container;
        while tmp > 0 {
            let d = tmp % 10;
            sum += d;
            tmp /= 10;
        }
    }
    // [0, 916864000000000, 223758251185210, 286253697920827, 941463976156518, 599993229915608, 468592963895217, 715968264381621, 238856266700490, 443944152681699, 93326215]
    // println!("{:?}", containers);
    sum
}
