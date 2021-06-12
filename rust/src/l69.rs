// We define the rounded-square-root of a positive integer n as the square root of n rounded to the nearest integer.

// The following procedure (essentially Heron's method adapted to integer arithmetic) finds the rounded-square-root of n:

// Let d be the number of digits of the number n.
// If d is odd, set
// .
// If d is even, set
// .
// Repeat:
fn heron_square_root(x: u64, n: u64) -> u64 {
    (x + (n as f64 / x as f64).floor() as u64) / 2
}

// /// ```rust
// /// use self::project_euler::m255::rounded_square_root;
// /// assert_eq!(rounded_square_root(), 2.3);
// /// ```
// pub fn rounded_square_root() -> f32 {
//     let x100 = 2 * 10u64.pow((5 - 1) / 2); //7 * 10u64.pow((14 - 2) / 2);
//     let mut n = 10_000u64; //10_000_000_000_000u64;
//     let mut count = 0u64;
//     let x2 = &mut (0);
//     loop {
//         *x2 = x100;
//         loop {
//             let x1 = heron_square_root(*x2, n);
//             count += 1;
//             if x1 == *x2 {
//                 break;
//             }
//             *x2 = x1;
//         }
//         n += 1;
//         if n == 100_000 {
//             //10_000_000_000_010u64 {// 100_000_000_000_000u64 {
//             break;
//         }
//     }
//     count as f32 / (100_000 - 10_000u64) as f32 // (100_000_000_000_000u64 - 10_000_000_000_000u64) as f32
// }

// /// ```rust
// /// use self::project_euler::l69::square_root;
// /// assert_eq!(square_root(4), 2);
// /// assert_eq!(square_root(8), 2);
// /// assert_eq!(square_root(9), 3);
// /// ```
// pub fn square_root(ab: u32) -> u32 {
//     let a = &mut 1u32;
//     let b = &mut 1u32;
//     while !(*a * *a <= ab && ab < *b * *b) {
//         *a = *a + ab / *a;
//         *a /= 2;
//     }
//     *b = *a + 1;
//     *a
// }

// pub fn square_root_brute_bench() {
//     for i in 0u32..2147395601 {
//         square_root_brute(i);
//     }
// }

// 1.5 s
/// ```rust
/// use self::project_euler::l69::square_root_brute;
/// assert_eq!(square_root_brute(4), 2);
/// assert_eq!(square_root_brute(8), 2);
/// assert_eq!(square_root_brute(9), 3);
/// ```
pub fn square_root_brute(aa: u32) -> u32 {
    //dbg!(aa);
    let mut a = 0u32;
    while a as u64 * a as u64 <= aa as u64 {
        a += 1;
    }
    a - 1
}

pub fn square_root_brute_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!((i as f64).sqrt() as u32, square_root_brute(i));
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!((i as f64).sqrt() as u32, square_root_brute(i));
    }
}

// 153 ms
/// ```rust
/// use self::project_euler::l69::square_root_newton_32bit_precision;
/// assert_eq!(square_root_newton_32bit_precision(4), 2);
/// assert_eq!(square_root_newton_32bit_precision(8), 2);
/// assert_eq!(square_root_newton_32bit_precision(9), 3);
/// assert_eq!(square_root_newton_32bit_precision(30624), 174);
/// ```
pub fn square_root_newton_32bit_precision(a: u32) -> u32 {
    let mut x = 1f32;
    let mut x_next: f32;
    loop {
        x_next = (x + a as f32 / x) / 2f32;
        if (x - x_next).abs() < 0.1 {
            //if x_next == x || (x_next * x_next - a as f32) < 0.1f32 {
            break x_next as u32;
        }
        x = x_next;
    }
}

pub fn square_root_newton_32bit_precision_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!(
            (i as f32).sqrt() as u32,
            square_root_newton_32bit_precision(i),
            "Error at {}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!(
            (i as f32).sqrt() as u32,
            square_root_newton_32bit_precision(i)
        );
    }
}

/// ```rust
/// use self::project_euler::l69::square_root_newton_integer_64;
/// assert_eq!(square_root_newton_integer_64(4), 2);
/// assert_eq!(square_root_newton_integer_64(8), 2);
/// assert_eq!(square_root_newton_integer_64(9), 3);
/// assert_eq!(square_root_newton_integer_64(30624), 174);
/// ```
pub fn square_root_newton_integer_64(a: u32) -> u32 {
    let mut x = 1u64;
    let mut x_next: u64;
    loop {
        x_next = (x + a as u64 / x) / 2;
        if x == x_next || x_next * x_next <= a as u64 {
            break x_next as u32;
        }
        x = x_next;
    }
}

pub fn square_root_newton_integer_64_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!(
            (i as f64).sqrt() as u32,
            square_root_newton_integer_64(i),
            "Error at {}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!((i as f64).sqrt() as u32, square_root_newton_integer_64(i));
    }
}

// 258 ms
/// ```rust
/// use self::project_euler::l69::square_root_newton_integer_32;
/// assert_eq!(square_root_newton_integer_32(4), 2);
/// assert_eq!(square_root_newton_integer_32(8), 2);
/// assert_eq!(square_root_newton_integer_32(9), 3);
/// assert_eq!(square_root_newton_integer_32(30624), 174);
/// ```
pub fn square_root_newton_integer_32(a: u32) -> u32 {
    let mut x = 1u32;
    let mut x_next: u32;
    loop {
        x_next = (x + a / x) / 2;
        if x == x_next || x_next == 0 || (x_next < u32::MAX / x_next && x_next * x_next <= a) {
            break x_next;
        }
        x = x_next;
    }
}

pub fn square_root_newton_integer_32_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!(
            (i as f32).sqrt() as u32,
            square_root_newton_integer_32(i),
            "Error at {}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!(
            // it's f64.
            (i as f64).sqrt() as u32,
            square_root_newton_integer_32(i),
            "Error at {}",
            i
        );
    }
}

// 203 ms
/// ```rust
/// use self::project_euler::l69::square_root_newton_64bit_precision;
/// assert_eq!(square_root_newton_64bit_precision(4), 2);
/// assert_eq!(square_root_newton_64bit_precision(8), 2);
/// assert_eq!(square_root_newton_64bit_precision(9), 3);
/// assert_eq!(square_root_newton_64bit_precision(30624), 174);
/// ```
pub fn square_root_newton_64bit_precision(a: u32) -> u32 {
    let mut x = 1f64;
    let mut x_next: f64;
    loop {
        x_next = (x + a as f64 / x) / 2f64;
        if (x - x_next).abs() < 0.1 {
            break x_next as u32;
        }
        x = x_next;
    }
}

pub fn square_root_newton_64bit_precision_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!(
            (i as f64).sqrt() as u32,
            square_root_newton_64bit_precision(i),
            "Error at {}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!(
            (i as f64).sqrt() as u32,
            square_root_newton_64bit_precision(i)
        );
    }
}

// 115 ms
/// ```rust
/// use self::project_euler::l69::square_root_digit_by_digit;
/// assert_eq!(square_root_digit_by_digit(4), 2, "4");
/// assert_eq!(square_root_digit_by_digit(8), 2, "8");
/// assert_eq!(square_root_digit_by_digit(9), 3, "9");
/// assert_eq!(square_root_digit_by_digit(30624), 174);
/// ```
pub fn square_root_digit_by_digit(mut a: u32) -> u32 {
    let mut digits = vec![];
    while a > 0 {
        digits.push(a % 100);
        a /= 100;
    }
    let (mut x, mut divisor, mut carry) = (0u32, 0u32, 0u32);
    for &n in digits.iter().rev() {
        carry = carry * 100 + n;
        divisor *= 10;
        x *= 10;
        subtract_largest_block_from_carry(&mut carry, &mut divisor, &mut x);
    }
    x
}

fn subtract_largest_block_from_carry(carry: &mut u32, divisor: &mut u32, x: &mut u32) {
    for i in (0u32..10).rev() {
        let block = (*divisor + i) * i;
        if block > *carry {
            continue;
        }
        *carry -= block;
        *divisor += i * 2;
        *x += i;
        break;
    }
}

pub fn square_root_digit_by_digit_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!(
            (i as f64).sqrt() as u32,
            square_root_digit_by_digit(i),
            "Error at {}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!((i as f64).sqrt() as u32, square_root_digit_by_digit(i));
    }
}

/// ```rust
/// use self::project_euler::l69::square_root_digit_by_digit_binary;
/// assert_eq!(square_root_digit_by_digit_binary(4), 2, "4");
/// assert_eq!(square_root_digit_by_digit_binary(8), 2, "8");
/// assert_eq!(square_root_digit_by_digit_binary(9), 3, "9");
/// assert_eq!(square_root_digit_by_digit_binary(30624), 174);
/// ```
pub fn square_root_digit_by_digit_binary(mut a: u32) -> u32 {
    let mut x = 0u32;
    let mut bit: u32 = 0b01000000000000000000000000000000;
    while bit > a {
        bit >>= 2;
    }
    while bit != 0 {
        let block = x + bit;
        x >>= 1;
        if a >= block {
            a -= block;
            x += bit;
        }
        bit >>= 2;
    }
    x
}

pub fn square_root_digit_by_digit_binary_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!(
            (i as f64).sqrt() as u32,
            square_root_digit_by_digit_binary(i),
            "Error at {}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!(
            (i as f64).sqrt() as u32,
            square_root_digit_by_digit_binary(i)
        );
    }
}

use std::cmp::Ordering;

// 476 ms
/// ```rust
/// use self::project_euler::l69::square_root_bisection;
/// assert_eq!(square_root_bisection(1), 1, "1");
/// assert_eq!(square_root_bisection(4), 2, "4");
/// assert_eq!(square_root_bisection(8), 2, "8");
/// assert_eq!(square_root_bisection(9), 3, "9");
/// assert_eq!(square_root_bisection(4488), 66, "4488");
/// assert_eq!(square_root_bisection(30624), 174);
/// ```
pub fn square_root_bisection(a: u32) -> u32 {
    let (mut top, mut bottom) = (a.clone() as f64, 0f64);
    loop {
        if top - bottom < 0.00001 {
            break top as u32;
        }
        let median = bottom + (top - bottom) / 2f64;
        match (&(a as f64)).partial_cmp(&(median * median)).unwrap() {
            Ordering::Less => top = median,
            Ordering::Equal => return median as u32,
            Ordering::Greater => bottom = median,
        }
        // //dbg!("{} {} {}", top, median, bottom);
        // if median * median < a as f64 {
        //     bottom = median;
        // } else {
        //     top = median;
        // }
    }
}

pub fn square_root_bisection_bench() {
    for i in 0u32..2_000_000 {
        assert_eq!(
            (i as f64).sqrt() as u32,
            square_root_bisection(i),
            "Error at {}",
            i
        );
    }
    for i in 2_147_390_000u32..2_147_395_601 {
        assert_eq!((i as f64).sqrt() as u32, square_root_bisection(i));
    }
}

mod tests {
    use super::*;

    #[test]
    fn internal() {
        square_root_brute_bench();
    }

    #[test]
    fn test_square_root_newton_32bit_precision_bench() {
        square_root_newton_32bit_precision_bench();
    }

    #[test]
    fn test_square_root_newton_64bit_precision_bench() {
        square_root_newton_64bit_precision_bench();
    }

    #[test]
    fn test_square_root_newton_integer_64bit_precision_bench() {
        square_root_newton_integer_64_bench();
    }

    #[test]
    fn test_square_root_newton_integer_32bit_precision_bench() {
        square_root_newton_integer_32_bench();
    }

    #[test]
    fn test_square_root_digit_by_digit_bench() {
        square_root_digit_by_digit_bench();
    }

    #[test]
    fn test_square_root_digit_by_digit_binary_bench() {
        square_root_digit_by_digit_binary_bench();
    }
    #[test]
    fn test_square_root_bisection_bench() {
        square_root_bisection_bench();
    }
}

pub fn my_sqrt(x: i32) -> i32 {
    let threshold = 0.01f64;
    let mut x0 = 10.0f64;
    let mut y = x0 as f64 * x0 as f64 - x as f64;
    while y.abs() > threshold {
        let slant = 2f64 * x0;
        let b = y - slant * x0;
        x0 = -1f64 * b / slant;
        y = x0 as f64 * x0 as f64 - x as f64;
    }
    x0 as i32
}

fn heron_square_root_2(x: f64, n: f64) -> f64 {
    (x + n / x) / 2f64
}
fn my_sqrt_2(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let digits = (x as f64).log10() as u32 + 1;
    let x0 = if digits % 2 == 0 {
        7 * 10i32.pow((digits - 2) / 2)
    } else {
        2 * 10i32.pow((digits - 1) / 2)
    };
    let mut y = heron_square_root_2(x0 as f64, x as f64);
    let threshold = 0.01f64;
    while y * y > threshold {
        y = heron_square_root_2(y as f64, x as f64);
    }
    x0
}
/*
impl Solution {

    fn heron_square_root(x: f64, n: f64) -> f64 {
        (x + n / x) / 2f64
    }
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let digits = (x as f64).log10() as u32 + 1;
        let x0 = if digits % 2 == 0 {
            7 * 10i32.pow((digits - 2) / 2)
        } else {
            2 * 10i32.pow((digits - 1) / 2)
        };
        let mut y = heron_square_root(x0 as f64, x as f64);
        let threshold = 0.01f64;
        while y * y - x as f64 > threshold {
            y = heron_square_root(y, x as f64);
        }
        y as i32
    }
    }
    */

pub fn my_sqrt_3(x: i32) -> i32 {
    let mut s = 1u64;
    let mut s2 = 1u64;
    while {
        s = (x as u64 / s + s) / 2;
        s2 = s + 1;
        !(s * s <= x as u64 && (x as u64) < s2 * s2)
    } {}
    s as i32
}
