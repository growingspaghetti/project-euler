// <p>There are exactly ten ways of selecting three from five, 12345:</p>
// <p class="center">123, 124, 125, 134, 135, 145, 234, 235, 245, and 345</p>
// <p>In combinatorics, we use the notation, $\displaystyle \binom 5 3 = 10$.</p>
// <p>In general, $\displaystyle \binom n r = \dfrac{n!}{r!(n-r)!}$, where $r \le n$, $n! = n \times (n-1) \times ... \times 3 \times 2 \times 1$, and $0! = 1$.
// </p>
// <p>It is not until $n = 23$, that a value exceeds one-million: $\displaystyle \binom {23} {10} = 1144066$.</p>
// <p>How many, not necessarily distinct, values of $\displaystyle \binom n r$ for $1 \le n \le 100$, are greater than one-million?</p>

const ONE_MILLION: u32 = 1_000_000;

// 11 us
/// ```rust
/// use self::project_euler::m53::binom_greater_than_one_million;
/// assert_eq!(binom_greater_than_one_million(), 4075);
/// ```
pub fn binom_greater_than_one_million() -> u32 {
    //const BLANK: [u32; 101] = [0u32; 101];
    let mut count = 0u32;
    let mut prev = [0u32; 101]; //Vec::<u32>::with_capacity(101);
    let mut line = [0u32; 101]; //Vec::<u32>::with_capacity(101);
    prev[0] = 1;
    for _ in 0..100 {
        line[0] = 1;
        for (i, &v) in prev.iter().enumerate() {
            if v == 0 {
                break;
            }
            let binom = v + prev[i + 1];
            line[i + 1] = if binom > ONE_MILLION {
                count += 1;
                ONE_MILLION
            } else {
                binom
            }
        }
        prev.copy_from_slice(&line);
    }
    //println!("{:?}", line);
    count
}

// /// ```rust
// /// use self::project_euler::m53::binom_greater_than_one_million_symmetry;
// /// assert_eq!(binom_greater_than_one_million_symmetry(), 4075);
// /// ```
// pub fn binom_greater_than_one_million_symmetry() -> u32 {
//     let mut count = 0u32;
//     let mut prev = [0u32; 101];
//     let mut line = [0u32; 101];
//     prev[0] = 1;
//     for x in 1usize..=100 {
//         line[0] = 1;
//         for i in 0..=x / 2 {
//             let binom = prev[i] + prev[i + 1];
//             if i == x / 2 && x %2 != 0{

//             } 
//             line[i + 1] = if binom > ONE_MILLION {
//                 subtotal += 1;
//                 ONE_MILLION
//             } else {
//                 binom
//             }
//         }
//         count += 2 * subtotal;
//         if subtotal > 0 && x % 2 != 0 {
//             count -= 1;
//         }
//         println!("{:?}", line);
//         prev.copy_from_slice(&line);
//     }
//     //println!("{:?}", line);
//     count
// }
