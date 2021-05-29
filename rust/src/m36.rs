// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

// (Please note that the palindromic number, in either base, may not include leading zeros.)

fn is_palindrome(a: u32) -> bool {
    if a % 2 == 0 && a % 11 != 0 {
        return false;
    }
    let mut t = a.clone();
    let mut b = 0u32;
    while t > 0 {
        b *= 10;
        b += t % 10;
        t /= 10;
    }
    a == b
}

fn is_double_based_palindrome(a: u32) -> bool {
    if a % 2 == 0 {
        return false;
    }
    let mut t = a.clone();
    let mut b = 0u32;
    while t > 0 {
        b <<= 1;
        b |= t & 1;
        t >>= 1;
    }
    a == b
}

// 4.9 ms
/// ```rust
/// use self::project_euler::m36::double_based_palindromes_sum;
/// assert_eq!(double_based_palindromes_sum(), 872187);
/// ```
pub fn double_based_palindromes_sum() -> u32 {
    (1..1_000_000)
        .step_by(2)
        .filter(|&n| is_palindrome(n))
        .filter(|&n| is_double_based_palindrome(n))
        .sum()
}

fn reverse_digits(a: u32) -> u32 {
    let mut t = a.clone();
    let mut b = 0u32;
    while t > 0 {
        b *= 10;
        b += t % 10;
        t /= 10;
    }
    b
}

fn generate_even_and_odd_palindromes(mut n: u32) -> (u32, u32) {
    let mut ep = n.clone();
    let mut op = n.clone();
    op /= 10;
    while n > 0 {
        ep *= 10;
        op *= 10;
        ep += n % 10;
        op += n % 10;
        n /= 10;
    }
    (ep, op)
}

// 10 -> rev -> 01 -> pal 11
// fn generate_even_digit_palindrome(n: u32) -> u32 {
//     let mut n = reverse_digits(n);
//     let mut p = n.clone();
//     while n > 0 {
//         p *= 10;
//         p += n % 10;
//         n /= 10;
//     }
//     p
// }

// fn generate_odd_digit_palindrome(n: u32) -> u32 {
//     let mut n = reverse_digits(n);
//     let mut p = n.clone();
//     p /= 10;
//     while n > 0 {
//         p *= 10;
//         p += n % 10;
//         n /= 10;
//     }
//     p
// }

// 17 us
/// ```rust
/// use self::project_euler::m36::double_based_palindromes_sum_gen;
/// assert_eq!(double_based_palindromes_sum_gen(), 872187);
/// ```
pub fn double_based_palindromes_sum_gen() -> u32 {
    let mut sum = 0u32;
    let half = 10u32.pow(1_000_000f32.log10() as u32 / 2);
    for n in 1..half {
        let (ep, op) = generate_even_and_odd_palindromes(n);
        // [ep, op]
        //     .iter()
        //     .filter(|&p| is_double_based_palindrome(*p))
        //     .for_each(|&p| sum += p);
        if is_double_based_palindrome(ep) {
            sum += ep;
        }
        if is_double_based_palindrome(op) {
            sum += op;
        }
    }
    sum
}
// if is_double_based_palindrome(ep) {
//     sum += ep;
// }
// if is_double_based_palindrome(op) {
//     sum += op;
// }
