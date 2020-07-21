/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits;
/// assert_eq!(largest_palindrome_product_of_two_3_digits(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let s = i.to_string();
        if s.chars().rev().collect::<String>() == s {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_mod_10;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_mod_10(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_mod_10() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        println!("{} {}", i, rev);
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    // [999     998     997     996]
    // [999-100 998-100 997-100 996-100]
    for x in (100..1000).rev() {
        for y in (100..=x).rev() {
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_mod_10_permutation_pair_tail_cut() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        println!("{} {}", i, rev);
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    // [999     998     997     996]
    // [999-100 998-100 997-100 996-100]
    // area -> -> -> -> -> | -> [in this sequence it's always smaller than largest_palindrome]
    for x in (100..1000).rev() {
        for y in (100..=x).rev() {
            if x * y <= largest_palindrome {
                break;
            }
            if let Some(i) = is_palindrome(x * y) {
                if i > largest_palindrome {
                    largest_palindrome = i
                }
            }
        }
    }
    largest_palindrome
}

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// ```rust
/// use self::project_euler::m4::largest_palindrome_product_of_two_3_digits_factorization;
/// assert_eq!(largest_palindrome_product_of_two_3_digits_factorization(), 906609);
/// ```
pub fn largest_palindrome_product_of_two_3_digits_factorization() -> u32 {
    let is_palindrome = |i: u32| -> Option<u32> {
        let mut rev = 0u32;
        {
            let mut tmp = i;
            while tmp > 0 {
                // rev = new rev 0-9 + old rev left shift [12 -> 120, 1 -> 10, 132 -> 1320]
                rev = tmp % 10 + rev * 10;
                tmp /= 10;
            }
        }
        if i == rev {
            Some(i)
        } else {
            None
        }
    };

    let mut largest_palindrome = 0;
    // [999     998     997     996]
    // [999-100 998-100 997-100 996-100]
    // area -> -> -> -> -> | -> [in this sequence it's always smaller than largest_palindrome]

    // n is in 6 digits < 999*999=998001.
    // n = xyx_xyx = 100_000x + 10_000y + 1_000z + 100z + 10y + 1x
    //   = 100_001x + 10_010y + 1_100z
    //   = 11 * (9091 + 910 + 100)
    // n = a * b
    //   = 11c * b || a * 11c || 11c * 11d
    for x in (100..1000).rev() {
        if x % 11 == 0 {
            for y in (100..=x).rev() {
                if x * y <= largest_palindrome {
                    break;
                }
                if let Some(i) = is_palindrome(x * y) {
                    if i > largest_palindrome {
                        largest_palindrome = i
                    }
                }
            }
        } else {
            // 999 - 9 (999%11) = 990. 989 - 10 (989%11) = 979.
            for y in (100..=x - x % 11).rev().step_by(11) {
                if x * y <= largest_palindrome {
                    break;
                }
                if let Some(i) = is_palindrome(x * y) {
                    if i > largest_palindrome {
                        largest_palindrome = i
                    }
                }
            }
        }
    }
    largest_palindrome
}
