///
/// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
///
/// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
///
/// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
///
///
/// ```rust
/// use self::project_euler::m17::english_one_to_one_thousand;
/// assert_eq!(english_one_to_one_thousand(), 21124);
/// ```
pub fn english_one_to_one_thousand() -> u64 {
    fn count_words(i: usize) -> u32 {
        let zero_tp_19 = [
            "",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
        ];
        let twenty_ninety = [
            "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
        ];
        match i {
            0..=19 => zero_tp_19[i].len() as u32,
            20..=99 => {
                let p1 = i % 10;
                let p2 = i / 10;
                (twenty_ninety[p2].len() + zero_tp_19[p1].len()) as u32
            }
            _ if i % 100 == 0 && i != 1000 => {
                let d = i / 100;
                (zero_tp_19[d].len() + "hundred".len()) as u32
            }
            101..=999 => {
                let p3 = i / 100;
                let p2p1 = i - p3 * 100;
                (zero_tp_19[p3].len() + "hundred".len() + "and".len()) as u32 + count_words(p2p1)
            }
            1000 => ("one".len() + "thousand".len()) as u32,
            _ => 0,
        }
    };
    let mut sum = 0u32;
    for i in 1..=1000 {
        sum += count_words(i);
    }
    sum as u64
}

//Rust tip - I couldn't make it work
// let build_words = |i:usize| -> &[String] {
// 	match i {
// 		0..=19 => &[zero_tp_19[i].to_string()],
// 		20..=99 => {
// 			let p1 = i % 10;
// 			let p2 = i / 10;
// 			&[twenty_ninety[p2].to_string(), zero_tp_19[p1].to_string()]
// 		},
// 		_ if i % 100 == 0 => {
// 			let d = i / 100;
// 			&[zero_tp_19[d].to_string(), hundred.to_string()]
// 		},
// 		101..=999 => {
// 			let p3 = i / 100;
// 			let p2p1 = i - p3 * 100;
// 			match p2p1 {
// 				1..=19 => &[zero_tp_19[p3].to_string(), hundred.to_string(), and.to_string(), zero_tp_19[p2p1].to_string()],
// 				20..=99 => {
// 					let p1 = p2p1 % 10;
// 					let p2 = p2p1 / 10;
// 					&[zero_tp_19[p3].to_string(), hundred.to_string(), and.to_string(), twenty_ninety[p2].to_string(), zero_tp_19[p1].to_string()]
// 				},
// 				_ => &[]
// 			}
// 		}
// 		1000 => &[one.to_string(), thousand.to_string()],
// 		_ => &[],
// 	}
// };
