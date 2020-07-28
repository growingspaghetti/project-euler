/// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
///
/// 1/2	= 	0.5
/// 1/3	= 	0.(3)
/// 1/4	= 	0.25
/// 1/5	= 	0.2
/// 1/6	= 	0.1(6)
/// 1/7	= 	0.(142857)
/// 1/8	= 	0.125
/// 1/9	= 	0.(1)
/// 1/10	= 	0.1
///
/// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
///
/// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
///
/// ```rust
/// use self::project_euler::m26::the_longest_recurring_cycle;
/// assert_eq!(the_longest_recurring_cycle(), (983, 982));
/// ```
pub fn the_longest_recurring_cycle() -> (u32, u32) {
    let mut number_having_longest_recurring_cycle = 0u32;
    let mut longest_recurring_cycle = 0u32;
    // apparently 1.000 can be evenly devided by 2a and 5a
    for divisor in (1..1000u32).step_by(2) {
        if divisor % 5 == 0 {
            continue;
        }
        let mut dividend = 1;
        let mut dividends: Vec<u32> = vec![];
        loop {
            dividend %= divisor;
            if dividend == 0 {
                break;
            }
            if dividends.contains(&dividend) {
                let pos = dividends.iter().position(|&v| v == dividend).unwrap();
                let recurring_cycle = (dividends.len() - pos) as u32;
                if recurring_cycle > longest_recurring_cycle {
                    number_having_longest_recurring_cycle = divisor;
                    longest_recurring_cycle = recurring_cycle;
                }
                //println!("{} {:?}", d, dividends);
                break;
            }
            dividends.push(dividend);
            dividend *= 10;
        }
    }
    (
        number_having_longest_recurring_cycle,
        longest_recurring_cycle,
    )
}
