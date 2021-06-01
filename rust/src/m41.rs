// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

// What is the largest n-digit pandigital prime that exists?

fn consume(usable_items: &Vec<u32>, accumulated_num: u32, drain: &mut Vec<u32>) {
    if usable_items.len() == 0 {
        drain.push(accumulated_num);
        return;
    }
    for i in 0..usable_items.len() {
        let mut items = usable_items.clone();
        let mut num = accumulated_num.clone();
        let n = items.remove(i);
        num *= 10;
        num += n;
        consume(&items, num, drain);
    }
}

fn permutation(n: u32) -> Vec<u32> {
    let items = (1..=n).into_iter().rev().collect::<Vec<u32>>();
    let capacity = items.iter().map(|&i| i).product::<u32>() as usize;
    let mut drain = Vec::with_capacity(capacity);
    consume(&items, 0, &mut drain);
    drain
}


fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 {
        return true;
    }
    for d in &[2u32, 3, 5] {
        if n % *d == 0 {
            return false;
        }
    }
    let side = (n as f32).sqrt() as u32;
    let mut d = 5u32;
    for i in [2u32, 4].iter().cycle() {
        if d > side {
            break;
        }
        d += *i;
        if n % d == 0 {
            return false;
        }
    }
    true
}

// 377 us
/// ```rust
/// use self::project_euler::m41::largest_pandigital_prime;
/// assert_eq!(largest_pandigital_prime(), 7652413);
/// ```
pub fn largest_pandigital_prime() -> u32 {
    for &d in [7, 4].iter() {
        for n in permutation(d) {
            if is_prime(n) {
                return n;
            }
        }
    }
    panic!("No prime pandigital was found!");
}
