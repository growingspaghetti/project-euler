
// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    assert!(b != 0);
    let r = a % b;
    if r == 0 {
        return b;
    }
    gcd(b, r)
}

/// ```rust
/// use self::project_euler::m33::four_non_trivial_fractions_product_denominator;
/// assert_eq!(four_non_trivial_fractions_product_denominator(), 100);
/// ```
pub fn four_non_trivial_fractions_product_denominator() -> u32 {
    let mut numerator = 1u32;
    let mut denominator = 1u32;
    // for c in 1u32..=9 {
    //     for d in 1u32..c {
    //         for b in d..c {
    //             if (10 * d + b) * c == (10 * c + d) * b && b != c {
    //                 println!("{} / {}; {}{}/{}{}", b, c, d, b, c, d);
    //                 numerator *= b;
    //                 denominator *= c;
    //             }
    //         }
    //     }
    // }
    for a in 1u32..=9 {
        for c in a..=9 {
            for d in a..c {
                if (10 * a + c) * d == (10 * c + d) * a && a != d {
                    numerator *= a;
                    denominator *= d;
                }
            }
        }
    }

    let cd = gcd(numerator, denominator);
    println!("{} {} {}", numerator, denominator, cd);
    denominator / cd
}
    // for a in 1u32..=9 {
    //     for c in 1u32..=9 {
    //         for d in a..=9 {
    //             if (10 * a + c) * d == (10 * c + d) * a {
    //                 println!("{} / {}; {}{}/{}{}", a, d, a, c, c, d);
    //                 numerator *= a;
    //                 denominator *= d;
    //             }
    //         }
    //     }
    // }