/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143(), 6857);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143() -> u64 {
    let mut n = 600851475143u64;
    let mut diviser = 2u64;
    let mut max_factor = 0u64;
    // 16 -> 2 * 8 -> 2 * 4 -> 2 * 2 -> 2 * 1
    // 21 -> 3 * 7 -> 7 * 1
    // 28 -> 2 * 14 -> 2 * 7 -> 7 * 1
    while n != 0 && n != 1 {
        if n % diviser != 0 {
            diviser += 1;
        } else {
            max_factor = n;
            n /= diviser;
        }
    }
    max_factor
}

/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12(), 6857);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12() -> u64 {
    let mut n = 600851475143u64;
    let mut diviser = 3u64;
    let mut max_factor;

    if n % 2 == 0 {
        n /= 2;
        max_factor = 2;
        while n % 2 == 0 {
            n /= 2;
        }
    } else {
        max_factor = 1;
    }

    while n > 1 {
        if n % diviser == 0 {
            n /= diviser;
            max_factor = diviser;
            while n % diviser == 0 {
                n /= diviser
            }
        } else {
            diviser += 2
        }
    }
    max_factor
}

/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
///
/// ```rust
/// use self::project_euler::m3::largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab;
/// assert_eq!(largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab(), 6857);
/// ```
pub fn largest_prime_factor_of_the_number_600851475143_skip_4_6_8_10_12_n_ab() -> u64 {
    let mut n = 600851475143u64;
    let mut diviser = 3u64;
    let mut max_factor;

    if n % 2 == 0 {
        n /= 2;
        max_factor = 2;
        while n % 2 == 0 {
            n /= 2;
        }
    } else {
        max_factor = 1;
    }

    // n = 1 * n || n = a * b
    //  in square,  n = sqrt(n) * sqrt(n)
    //  pattern 1: a = sqrt(n) && b = sqrt(n)
    //  pattern 2: a <= sqrt(n) || b <= sqrt(n)
    //  impossible: a > sqrt(n) && b >= sqrt(n)
    let a = (n as f64).sqrt() as u64;
    while n > 1 && diviser <= a {
        if n % diviser == 0 {
            n /= diviser;
            max_factor = diviser;
            while n % diviser == 0 {
                n /= diviser
            }
        } else {
            diviser += 2
        }
    }
    max_factor
}
