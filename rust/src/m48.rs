fn mod_pow(mut a: u128, mut exp: u128, m: u128) -> u64 {
    if m == 1 {
        return 0;
    }
    if exp == 0 {
        return 1;
    }
    let mut result = 1;
    a %= m;
    loop {
        if exp % 2 == 1 {
            result *= a;
            result %= m;
        }
        exp >>= 1;
        if exp == 0 {
            break;
        }
        a *= a;
        a %= m;
    }
    result as u64
}
// a = ((a % m) * (a % m)) % m;
// result = ((result % m) * (a % m)) % m;
fn conservative_mod_pow(a: u64, exp: u64, m: u64) -> u64 {
    let mut result = 1;
    for _ in 0..exp {
        result *= a;
        result %= m;
    }
    result
}

// 2.0 ms
/// ```rust
/// use self::project_euler::m48::self_power_1000;
/// assert_eq!(self_power_1000(), 9110846700);
/// ```
pub fn self_power_1000() -> u64 {
    let mut sum = 0u64;
    for n in 1..=1000 {
        sum += conservative_mod_pow(n, n, 10_000_000_000);
        sum %= 10_000_000_000;
    }
    sum
}

/// ```rust
/// use self::project_euler::m48::self_power_1000_128;
/// assert_eq!(self_power_1000_128(), 9110846700);
/// ```
pub fn self_power_1000_128() -> u64 {
    let mut sum = 0u64;
    for n in 1..=1000 {
        sum += mod_pow(n as u128, n as u128, 10_000_000_000);
        sum %= 10_000_000_000;
    }
    sum
}
