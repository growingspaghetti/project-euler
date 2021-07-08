// <p>If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.</p>
// <p>Not all numbers produce palindromes so quickly. For example,</p>
// <p class="margin_left">349 + 943 = 1292,<br />
// 1292 + 2921 = 4213<br />
// 4213 + 3124 = 7337</p>
// <p>That is, 349 took three iterations to arrive at a palindrome.</p>
// <p>Although no one has proved it yet, it is thought that some numbers, like 196, never produce a palindrome. A number that never forms a palindrome through the reverse and add process is called a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise. In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome. In fact, 10677 is the first number to be shown to require over fifty iterations before producing a palindrome: 4668731596684224866951378664 (53 iterations, 28-digits).</p>
// <p>Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is 4994.</p>
// <p>How many Lychrel numbers are there below ten-thousand?</p>
// <p class="smaller">NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.</p>

// for i in 0..bignum.len() {
//     let s = bignum[i] + rev[i] + carry;
//     bignum[i] = s % 10;
//     carry = s / 10;
// }

fn merge_each_other(num: &mut Vec<u8>, rev: &mut Vec<u8>) {
    let mut carry = 0u8;
    rev.iter().zip(num.iter_mut()).for_each(|(&r, d)| {
        let s = r + *d + carry;
        *d = s % 10;
        carry = s / 10;
    });
    if carry != 0 {
        num.push(carry);
        rev.push(0);
    }
    num.iter()
        .rev()
        .zip(rev.iter_mut())
        .for_each(|(&d, r)| *r = d);
}

fn symmetric_digit_vector_pair(mut n: u32, num: &mut Vec<u8>, rev: &mut Vec<u8>) {
    num.clear();
    rev.clear();
    while n > 0 {
        let d = (n % 10) as u8;
        rev.push(d);
        n /= 10;
    }
    rev.iter().rev().for_each(|&d| num.push(d));
    //let num = rev.iter().rev().map(|&d| d).collect::<Vec<u8>>();
}

// 1.53 ms
/// ```rust
/// use self::project_euler::m55::how_many_lychrel_numbers_are_there_below_ten_thousand;
/// assert_eq!(how_many_lychrel_numbers_are_there_below_ten_thousand(), 249);
/// ```
pub fn how_many_lychrel_numbers_are_there_below_ten_thousand() -> u32 {
    let mut count = 0u32;
    let mut num = Vec::with_capacity(56);
    let mut rev = Vec::with_capacity(56);

    'next_num: for n in 10u32..10_000 {
        symmetric_digit_vector_pair(n, &mut num, &mut rev);
        for _ in 0..50 {
            merge_each_other(&mut num, &mut rev);
            if rev == num {
                continue 'next_num;
            }
        }
        count += 1;
    }
    count
}

use std::collections::LinkedList;

fn merge_each_other_linked_list(num: &mut LinkedList<u8>, rev: &mut LinkedList<u8>) {
    let mut carry = 0u8;
    rev.iter().zip(num.iter_mut()).for_each(|(&r, d)| {
        let s = r + *d + carry;
        *d = s % 10;
        carry = s / 10;
    });
    if carry != 0 {
        num.push_back(carry);
        rev.push_back(0);
    }
    num.iter()
        .rev()
        .zip(rev.iter_mut())
        .for_each(|(&d, r)| *r = d);
}

fn symmetric_digit_vector_pair_linked_list(mut n: u32) -> (LinkedList<u8>, LinkedList<u8>) {
    let mut rev = LinkedList::new();
    while n > 0 {
        let d = (n % 10) as u8;
        rev.push_back(d);
        n /= 10;
    }
    let num = rev.iter().rev().map(|&d| d).collect::<LinkedList<u8>>();
    (num, rev)
}

// 4.2 ms
/// ```rust
/// use self::project_euler::m55::how_many_lychrel_numbers_are_there_below_ten_thousand_linked_list;
/// assert_eq!(how_many_lychrel_numbers_are_there_below_ten_thousand_linked_list(), 249);
/// ```
pub fn how_many_lychrel_numbers_are_there_below_ten_thousand_linked_list() -> u32 {
    let mut count = 0u32;
    'next_num: for n in 10u32..10_000 {
        let (mut num, mut rev) = symmetric_digit_vector_pair_linked_list(n);
        for _ in 0..50 {
            merge_each_other_linked_list(&mut num, &mut rev);
            if rev == num {
                continue 'next_num;
            }
        }
        count += 1;
    }
    count
}
