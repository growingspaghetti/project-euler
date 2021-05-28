//145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//Note: As 1! = 1 and 2! = 2 are not sums they are not included.

fn build_factorial_tenfold() -> [u32; 10] {
    let mut acc = 1u32;
    let mut factorial_tenfold = [1u32; 10];
    for n in 1..=9 {
        acc *= n;
        factorial_tenfold[n as usize] = acc;
    }
    factorial_tenfold
}

fn digit_range_max(fact_nine: u32) -> u32 {
    let mut digit_min = 1u32;
    let mut fact_sum_max = fact_nine;
    while digit_min < fact_nine {
        digit_min *= 10;
        fact_sum_max += fact_nine;
    }
    fact_sum_max - fact_nine
}

fn match_factorial_sum(target: u32, factorial_tenfold: &[u32; 10]) -> bool {
    let mut digits = target;
    let mut sum = 0;
    while digits > 0 {
        let d = digits % 10;
        sum += factorial_tenfold[d as usize];
        if sum > target {
            return false;
        }
        digits /= 10;
    }
    sum == target
}

fn match_factorial_sum_1000(target: u32, factorial_sum_1000_fold: &[u32; 1000]) -> bool {
    let mut digits = target;
    let mut sum = 0;
    while digits > 0 {
        let d = digits % 1000;
        sum += factorial_sum_1000_fold[d as usize];
        digits /= 1000;
        match digits > 0 {
            true if d < 10 => sum += 2,
            true if d < 100 => sum += 1,
            _ => (),
        }
        if sum > target {
            return false;
        }
    }
    sum == target
}

fn zero_pad_10000(carry: u32, residue: u32, sum: &mut u32) {
    match (carry > 0, residue) {
        (false, _) => (),
        (true, 0..=9) => *sum += 3,
        (true, 10..=99) => *sum += 2,
        (true, 100..=999) => *sum += 1,
        _ => (),
    }
}

fn match_factorial_sum_10000(target: u32, factorial_sum_10000_fold: &[u32; 10000]) -> bool {
    let mut digits = target;
    let mut sum = 0;
    while digits > 0 {
        let d = digits % 10000;
        digits /= 10000;
        sum += factorial_sum_10000_fold[d as usize];
        zero_pad_10000(digits, d, &mut sum);
        if sum > target {
            return false;
        }
    }
    sum == target
}
// match (digits > 0, d) {
//     (false, _) => (),
//     (true, 0..=9) => sum += 3,
//     (true, 10..=99) => sum += 2,
//     (true, 100..=999) => sum += 1,
//     _ => (),
// }
fn factorial_sum_1000_fold(factorial_tenfold: &[u32; 10]) -> [u32; 1000] {
    let mut factorial_sum_1000_fold = [0u32; 1000];
    factorial_sum_1000_fold[0] = 1;
    for i in 1..factorial_sum_1000_fold.len() {
        let mut sum = 0;
        let mut digits = i as u32;
        while digits > 0 {
            let d = digits % 10;
            digits /= 10;
            sum += factorial_tenfold[d as usize];
        }
        factorial_sum_1000_fold[i] = sum;
    }
    factorial_sum_1000_fold
}

fn factorial_sum_10000_fold(factorial_tenfold: &[u32; 10]) -> [u32; 10000] {
    let mut factorial_sum_10000_fold = [1u32; 10000];
    for i in 1..factorial_sum_10000_fold.len() {
        let mut sum = 0;
        let mut digits = i as u32;
        while digits > 0 {
            let d = digits % 10;
            digits /= 10;
            sum += factorial_tenfold[d as usize];
        }
        factorial_sum_10000_fold[i] = sum;
    }
    // for i in 100..factorial_sum_10000_fold.len() {
    //     let mut sum = 0;
    //     sum += factorial_sum_10000_fold[i % 100]; // 00 -> 2, 0 -> 1
    //     sum += factorial_sum_10000_fold[i / 100];
    //     factorial_sum_10000_fold[i] = sum;
    // }
    factorial_sum_10000_fold
}

// 25 ms (single)
// 10 ms (%10000)
/// ```rust
/// use self::project_euler::m34::digit_factorials_abc;
/// assert_eq!(digit_factorials_abc(), 40730);
/// ```
pub fn digit_factorials_abc() -> u32 {
    let factorial_tenfold = build_factorial_tenfold();
    let factorial_sum_10000_fold = factorial_sum_10000_fold(&factorial_tenfold);
    //let factorial_sum_1000_fold = factorial_sum_1000_fold(&factorial_tenfold);
    //(0..=9u32).map(|n| factorial(n)).collect::<Vec<u32>>();
    let digit_range_max = digit_range_max(factorial_tenfold[9]);
    (3..digit_range_max)
        .filter(|&d| match_factorial_sum_10000(d, &factorial_sum_10000_fold)) // match_factorial_sum(d, &factorial_tenfold)
        //.filter(|&d| match_factorial_sum_1000(d, &factorial_sum_1000_fold))
        .sum()
}
