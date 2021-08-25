// <p>The 5-digit number, 16807=7<sup>5</sup>, is also a fifth power. Similarly, the 9-digit number, 134217728=8<sup>9</sup>, is a ninth power.</p>
// <p>How many <i>n</i>-digit positive integers exist which are also an <i>n</i>th power?</p>

/// ```rust
/// use self::project_euler::m63::power_digit_counts_1;
/// assert_eq!(power_digit_counts_1(), 49);
/// ```
pub fn power_digit_counts_1() -> u8 {
    let mut sum = 0u8;
    for a in 1..=9 {
        let exp = 1f32 / (1f32 - (a as f32).log10());
        println!("{}", exp);
        sum += exp as u8;
    }
    println!("{}", sum);
    sum
}

/// ```rust
/// use self::project_euler::m63::power_digit_counts_2;
/// assert_eq!(power_digit_counts_2(), 49);
/// ```
pub fn power_digit_counts_2() -> u8 {
    let mut count = 0u8;
    for a in 1u128..=9 {
        for exp in 1.. {
            let n = a.pow(exp);
            if ((n as f32).log10() as u32 + 1) < exp {
                break;
            }
            println!("{}", n);
            count += 1;
        }
    }
    println!("{}", count);
    count
}

// fn main() {
//     let mut count = 0u8;
//     for a in 1u128..=9 {
//         for exp in 1.. {
//             let n = a.pow(exp);
//             if n < 10u128.pow(exp - 1) {
//                 break;
//             }
//             println!("{} ^ {} \t = {}", a, exp, n);
//             count += 1;
//         }
//     }
//     println!("{}", count);
// }