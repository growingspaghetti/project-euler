// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

// {20,48,52}, {24,45,51}, {30,40,50}

// For which value of p ≤ 1000, is the number of solutions maximised?

// 30 ms
/// ```rust
/// use self::project_euler::m39::perimeter_of_a_right_angle_triangle_1000;
/// assert_eq!(perimeter_of_a_right_angle_triangle_1000(), 840);
/// ```
pub fn perimeter_of_a_right_angle_triangle_1000() -> u32 {
    let mut counts = [0u8; 1001];
    for c in 3..=997 {
        let mut b = 2;
        while b < 1000 - c && b < c {
            let mut a = 1;
            while a <= 1000 - c - b && a < b {
                if c * c == b * b + a * a {
                    counts[c + b + a] += 1;
                }
                a += 1;
            }
            b += 1;
        }
    }
    let (p, _) = counts
        .iter()
        .enumerate()
        .reduce(|(ap, a), (bp, b)| if *a > *b { (ap, a) } else { (bp, b) })
        .unwrap();
    println!("{:?}", counts);
    p as u32
}

// 364 us
/// ```rust
/// use self::project_euler::m39::perimeter_of_a_right_angle_triangle_1000_2;
/// assert_eq!(perimeter_of_a_right_angle_triangle_1000_2(), 840);
/// ```
pub fn perimeter_of_a_right_angle_triangle_1000_2() -> u32 {
    let mut max = 0;
    let mut maxp = 0;
    for p in (2..=1000).step_by(2) {
        let mut c = 0u32;
        for a in 2..p / 3 as u32 {
            if (p * (p - 2 * a)) % (2 * (p - a)) == 0 {
                c += 1;
            }
        }
        if c > max {
            max = c;
            maxp = p;
        }
    }
    maxp
}

fn gcd(mut a: usize, mut b: usize) -> usize {
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


// 67 us
/// ```rust
/// use self::project_euler::m39::perimeter_of_a_right_angle_triangle_1000_4;
/// assert_eq!(perimeter_of_a_right_angle_triangle_1000_4(), 840);
/// ```
pub fn perimeter_of_a_right_angle_triangle_1000_4() -> u32 {
    let mut counts = [0u8; 1001];
    for p in (12..=1000).step_by(2) {
        for m in 2..=(((p - 2) / 2) as f32).sqrt() as usize {
            for n in ((if m % 2 == 0 { 1 } else { 2 })..m).step_by(2) {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                if a + b + c == p && gcd(m, n) == 1 {
                    for k in (p..=1000).step_by(p) {
                        counts[k] += 1;
                    }
                }
            }
        }
    }
    let (p, _) = counts
        .iter()
        .enumerate()
        .reduce(|(ap, a), (bp, b)| if *a > *b { (ap, a) } else { (bp, b) })
        .unwrap();
    p as u32
}


mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(count_euclid(), count_brute());
    }
}

fn count_euclid() -> [u8; 1001] {
    let mut counts = [0u8; 1001];
    for p in (6..=1000).step_by(2) {
        for m in 2..=(((p - 2) / 2) as f32).sqrt() as usize {
            for n in ((if m % 2 == 0 { 1 } else { 2 })..m).step_by(2) {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                if a + b + c == p && gcd(m, n) == 1 {
                    for k in (p..=1000).step_by(p) {
                        counts[k] += 1;
                    }
                }
            }
        }
    }
    counts
}

fn count_brute() -> [u8; 1001] {
    let mut counts = [0u8; 1001];
    for c in 3..=997 {
        let mut b = 2;
        while b < 1000 - c && b < c {
            let mut a = 1;
            while a <= 1000 - c - b && a < b {
                if c * c == b * b + a * a {
                    counts[c + b + a] += 1;
                }
                a += 1;
            }
            b += 1;
        }
    }
    counts
}