use std::ops::RangeInclusive;

fn is_pandigital(a: u32, b: u32, ab: u32) -> bool {
    let mut bits = 0u16;
    for n in [a, b, ab].iter_mut() {
        while *n > 0 {
            let d = *n % 10;
            *n /= 10;
            bits |= 1 << d;
        }
    }
    bits == 0b1111111110u16
}

#[test]
fn test_is_pandigital() {
    let n = 0b1111111110u16;
    println!("{}", n);
    println!("{}", 0x3fe);
    assert_eq!(is_pandigital(2, 5, 3), false);
    assert_eq!(is_pandigital(39, 186, 7254), true);
}

fn sum_distinct(arr: &mut [u32]) -> u32 {
    arr.sort();
    let mut sum = 0u32;
    if let Some(&n) = arr.get(0) {
        sum += n;
    }
    for i in 1..arr.len() {
        if arr[i - 1] != arr[i] {
            sum += arr[i];
        }
    }
    sum
}

fn explore_pandigital_combinations(
    a: RangeInclusive<u32>,
    b: RangeInclusive<u32>,
    products: &mut Vec<u32>,
) {
    for a in a {
        for b in b.clone() {
            let ab = a * b;
            if ab > 9876 {
                break;
            }
            if is_pandigital(a, b, ab) {
                products.push(ab);
            }
        }
    }
}

//
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
//
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
//
// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
//
// 250 us
// 417 including all
// 2.2 ms without if ab > 9876 {
/// ```rust
/// use self::project_euler::m32::sum_of_all_products_nine_digit_pandigital;
/// assert_eq!(sum_of_all_products_nine_digit_pandigital(), 45228);
/// ```
pub fn sum_of_all_products_nine_digit_pandigital() -> u32 {
    let mut products = Vec::<u32>::new();
    explore_pandigital_combinations(2..=9, 1234..=9876, &mut products);
    explore_pandigital_combinations(12..=98, 123..=987, &mut products);
    // for ((a_start, a_end_eq), (b_start, b_end_eq)) in
    //     [((2, 9), (1234, 9876)), ((12, 98), (123, 987))].iter()
    // {
    //     for a in *a_start..=*a_end_eq {
    //         for b in *b_start..=*b_end_eq {
    //             let ab = a * b;
    //             if ab > 9876 {
    //                 break;
    //             }
    //             if is_pandigital(a, b, ab) {
    //                 products.push(ab);
    //             }
    //         }
    //     }
    // }

    sum_distinct(&mut products)
}
