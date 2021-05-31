// Take the number 192 and multiply it by each of 1, 2, and 3:

// 192 × 1 = 192
// 192 × 2 = 384
// 192 × 3 = 576

// By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)

// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).

// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?

fn is_pandigital(a: u32, b: u32) -> bool {
    let mut bits = 0u16;
    for n in [a, b].iter_mut() {
        while *n > 0 {
            let d = *n % 10;
            *n /= 10;
            bits |= 1 << d;
        }
    }
    bits == 0b1111111110u16
}

// 6.9 us
/// ```rust
/// use self::project_euler::m38::pandigital_9_digit_number_concatenated_product;
/// assert_eq!(pandigital_9_digit_number_concatenated_product(), 932718654);
/// ```
pub fn pandigital_9_digit_number_concatenated_product() -> u32 {
    for n in (9000..=9876).rev() {
        if is_pandigital(n, n * 2) {
            return n * 100_000 + 2 * n;
        }
    }
    918_273_645
}

// 2.1 us
/// ```rust
/// use self::project_euler::m38::pandigital_9_digit_number_concatenated_product_97;
/// assert_eq!(pandigital_9_digit_number_concatenated_product_97(), 932718654);
/// ```
pub fn pandigital_9_digit_number_concatenated_product_97() -> u32 {
    for n in (9183..=(19000 / 2) as u32).rev() {
        if is_pandigital(n, n * 2) {
            return n * 100_000 + 2 * n;
        }
    }
    918_273_645
}
