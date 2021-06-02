// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.

// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:

//     d2d3d4=406 is divisible by 2
//     d3d4d5=063 is divisible by 3
//     d4d5d6=635 is divisible by 5
//     d5d6d7=357 is divisible by 7
//     d6d7d8=572 is divisible by 11
//     d7d8d9=728 is divisible by 13
//     d8d9d10=289 is divisible by 17

// Find the sum of all 0 to 9 pandigital numbers with this property.

fn is_divisible(n: u64, depth: u8) -> bool {
    if depth < 3 {
        return true;
    }
    let p = match depth {
        3 => 2,
        4 => 3,
        5 => 5,
        6 => 7,
        7 => 11,
        8 => 13,
        9 => 17,
        _ => panic!(),
    };
    (n % 1000) % p == 0
}

fn has_duplication(mut n: u64) -> bool {
    let mut count = vec![0u8; 10];
    while n > 0 {
        let d = n % 10;
        count[d as usize] += 1;
        n /= 10;
    }
    count.iter().any(|&c| c > 1)
}

fn consume(usable_items: &Vec<u8>, accumulated_num: u64, drain: &mut Vec<u64>, depth: u8) {
    if usable_items.len() == 0 {
        drain.push(accumulated_num);
        return;
    }
    for i in 0..usable_items.len() {
        let mut items = usable_items.clone();
        let mut num = accumulated_num.clone();
        let n = items.remove(i);
        num *= 10;
        num += n as u64;
        if !is_divisible(num, depth) {
            continue;
        }
        consume(&items, num, drain, depth + 1);
    }
}

fn permutations() -> Vec<u64> {
    let items = (0..=9).into_iter().collect::<Vec<u8>>();
    let mut drain = vec![];
    consume(&items, 0, &mut drain, 0);
    drain
}

fn is_last_three_digits_divisible(n: u32, d: u8) -> bool {
    assert!(n > 99);
    n / 1000 % d as u32 == 0
}

fn is_divisible_by_three(d3: u8, d4: u8, d5: u8) -> bool {
    (d3 + d4 + d5) % 3 == 0
}

fn build_ghij(hijs: &[u32]) {
    for &hij in hijs {}
}

fn is_digits_unique(n: u32) -> bool {
    assert!(n >= 100 && n < 1000);
    let a = n % 10;
    let b = n / 10 % 10;
    let c = n / 100;
    a != b && b != c && c != a
}

// 2.5 ms
/// ```rust
/// use self::project_euler::m43::pandigital_numbers_divisible_substrings;
/// assert_eq!(pandigital_numbers_divisible_substrings(), 16695334890);
/// ```
pub fn pandigital_numbers_divisible_substrings() -> u64 {
    let a = permutations();
    a.iter().filter(|&n| *n > 999_999_999).sum::<u64>()
}

fn is_divisible_with_level(n: u64, depth: u8) -> bool {
    if depth < 2 || depth == 9 {
        return true;
    }
    let p = match depth {
        2 => 17,
        3 => 13,
        4 => 11,
        5 => 7,
        6 => 5,
        7 => 3,
        8 => 2,
        _ => panic!(),
    };
    (n / 10u64.pow(depth as u32 - 2)) % p == 0
}

fn consume_with_level(
    usable_items: &Vec<u8>,
    accumulated_num: u64,
    drain: &mut Vec<u64>,
    depth: u8,
) {
    if usable_items.len() == 0 {
        drain.push(accumulated_num);
        return;
    }
    for i in 0..usable_items.len() {
        let mut items = usable_items.clone();
        let mut num = accumulated_num.clone();
        let n = items.remove(i);
        let weight = 10u64.pow(depth as u32);
        num += weight * n as u64;
        if !is_divisible_with_level(num, depth) {
            continue;
        }
        consume_with_level(&items, num, drain, depth + 1);
    }
}

fn permutations_with_level() -> Vec<u64> {
    let items = (0..=9).into_iter().collect::<Vec<u8>>();
    let mut drain = vec![];
    consume_with_level(&items, 0 as u64, &mut drain, 0);
    drain
}

// 55 us
/// ```rust
/// use self::project_euler::m43::pandigital_numbers_divisible_substrings_level;
/// assert_eq!(pandigital_numbers_divisible_substrings_level(), 16695334890);
/// ```
pub fn pandigital_numbers_divisible_substrings_level() -> u64 {
    permutations_with_level()
        .iter()
        .filter(|&n| *n > 999_999_999)
        .sum::<u64>()
}
