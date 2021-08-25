// <p>The cube, 41063625 (345<sup>3</sup>), can be permuted to produce two other cubes: 56623104 (384<sup>3</sup>) and 66430125 (405<sup>3</sup>). In fact, 41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.</p>
// <p>Find the smallest cube for which exactly five permutations of its digits are cube.</p>

mod tests {
    use super::*;

    #[test]
    fn test_() {
        assert_eq!(cubic_permutations_1(), 127035954683);
    }
}

fn freq(mut n: u64) -> u64 {
    let mut freq = 0u64;
    while n > 0 {
        freq += 0b000001 << ((n % 10) * 6);
        n /= 10;
    }
    freq
}

use std::collections::HashMap;

// // 1.2 ms
// /// ```rust
// /// use self::project_euler::m62::cubic_permutations_1;
// /// assert_eq!(cubic_permutations_1(), 127035954683);
// /// ```
// pub fn cubic_permutations_1() -> u64 {
//     let mut counts: HashMap<u64, (u8, u16)> = HashMap::new();
//     for n in 346u16.. {
//         let cube = n as u64 * n as u64 * n as u64;
//         let freq = freq(cube);
//         if let Some((count, ref n)) = counts.get_mut(&freq) {
//             if *count == 4 {
//                 return *n as u64 * *n as u64 * *n as u64;
//             }
//             *count += 1;
//             continue;
//         }
//         counts.insert(freq, (1, n));
//     }
//     unreachable!();
// }

fn min(counts: &HashMap<u64, (u8, u16)>, count: u8) -> Option<&u16> {
    counts
        .iter()
        .filter(|(_, (c, _))| *c == count)
        .map(|(_, (_, n))| n)
        .min()
}
// let min = counts
// .iter()
// .filter(|(_, (c, _))| *c == 5)
// .map(|(_, (_, n))| n)
// .min();
//if digits / cube < 1 || digits == cube {
// let min = counts
//     .clone()
//     .into_values()
//     .filter(|(count, _)| *count == 5)
//     .map(|(_, n)| n)
//     .min();

// 1.1 ms
/// ```rust
/// use self::project_euler::m62::cubic_permutations_1;
/// assert_eq!(cubic_permutations_1(), 127035954683);
/// ```
pub fn cubic_permutations_1() -> u64 {
    let mut counts: HashMap<u64, (u8, u16)> = HashMap::new();
    let mut digits = 1u64;
    while digits < 346u64.pow(3) {
        digits *= 10;
    }
    for n in 346u16.. {
        let cube = (n as u64).pow(3);
        if digits as f32 / cube as f32 <= 1f32 {
            if let Some(&n) = min(&counts, 5) {
                return (n as u64).pow(3);
            }
            counts.clear();
            digits *= 10;
        }
        let freq = freq(cube);
        if let Some((count, _)) = counts.get_mut(&freq) {
            *count += 1;
            continue;
        }
        counts.insert(freq, (1, n));
    }
    unreachable!();
}
