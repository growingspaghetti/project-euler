use std::usize;

// It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.

// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.

fn histogram(mut n: u32, digits: &mut [u8; 10]) {
    //digits.iter_mut().for_each(|x| *x = 0);
    digits.copy_from_slice(&BLANK);
    while n > 0 {
        let d = (n % 10) as usize;
        digits[d] += 1;
        n /= 10;
    }
}

const BLANK: [u8; 10] = [0u8; 10];

fn explorarion(place: u32, digit_matrix: &mut [[u8; 10]; 6]) -> Option<u32> {
    'next: for x in place..place * 10 / 6 {
        histogram(x, &mut digit_matrix[0]);
        for a in 2usize..=6 {
            histogram(a as u32 * x, &mut digit_matrix[a - 1]);
            if digit_matrix[0] != digit_matrix[a - 1] {
                continue 'next;
            }
        }
        return Some(x);
    }
    None
}

// 1.3 ms
/// ```rust
/// use self::project_euler::m52::exactly_the_same_digits_6x;
/// assert_eq!(exactly_the_same_digits_6x(), 142857);
/// ```
pub fn exactly_the_same_digits_6x() -> u32 {
    let mut digit_matrix = [[0u8; 10]; 6];
    let mut place = 100u32;
    loop {
        //digit_matrix.copy_from_slice(&BLANK[..]);
        match explorarion(place, &mut digit_matrix) {
            Some(n) => return n,
            None => place *= 10,
        }
    }
    // for e in 0u32.. {
    //     'next_digit: for x in 10u32.pow(e)..10u32.pow(e + 1) / 6 {
    //         histogram(x, &mut digit_matrix[0]);
    //         for a in 2u32..=6 {
    //             histogram(a * x, &mut digit_matrix[a as usize - 1]);
    //             if digit_matrix[a as usize - 2] != digit_matrix[a as usize - 1] {
    //                 continue 'next_digit;
    //             }
    //         }
    //         println!("{:?}", digit_matrix);
    //         return x;
    //     }
    // }
}
