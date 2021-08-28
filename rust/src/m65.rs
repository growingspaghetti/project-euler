mod tests {
    use super::*;

    #[test]
    fn test_() {
        assert_eq!(e_numerator_1(), 272);
    }
}

mod bignum {
    #[derive(Clone)]
    pub struct BigNum {
        v: Vec<u64>,
    }

    impl BigNum {
        const TEN_MIL: u64 = 10_000_000_000;
        pub fn new(n: u8) -> Self {
            Self { v: vec![n as u64] }
        }
        fn merge(&mut self, page: usize, n: u64, carry: &mut u64) {
            if self.v.get(page).is_none() {
                self.v.push(0u64)
            }
            if let Some(con) = self.v.get_mut(page) {
                *con += n + *carry;
                if *con < Self::TEN_MIL {
                    *carry = 0;
                    return;
                }
                *carry = 1;
                *con -= Self::TEN_MIL;
            }
        }
        pub fn add(&mut self, b: &BigNum) {
            let mut p = 0usize;
            let mut carry = 0u64;
            let mut ite = b.v.iter();
            while let Some(n) = ite.next() {
                self.merge(p, *n, &mut carry);
                p += 1;
            }
            while carry != 0 {
                self.merge(p, 0, &mut carry);
                p += 1;
            }
        }
        pub fn multiply(&mut self, n: u8) {
            let mut carry = 0u64;
            for con in self.v.iter_mut() {
                *con = *con * n as u64 + carry;
                if *con < Self::TEN_MIL {
                    carry = 0;
                    continue;
                }
                carry = *con / Self::TEN_MIL;
                *con %= Self::TEN_MIL;
            }
            if carry != 0 {
                self.v.push(carry);
            }
        }
        pub fn digit_sum(&self) -> u32 {
            let mut sum = 0u32;
            for &con in &self.v {
                let mut t = con;
                while t > 0 {
                    sum += (t % 10) as u32;
                    t /= 10;
                }
            }
            sum
        }
    }
}

// struct EulerNumberNumerator {
//     prev: u128,
//     value: u128,
//     term: u32,
// }

struct EulerNumberNumerator {
    prev: bignum::BigNum,
    value: bignum::BigNum,
    term: u8,
}

/// ```rust
/// use self::project_euler::m65::e_numerator_1;
/// assert_eq!(e_numerator_1(), 272);
/// ```
pub fn e_numerator_1() -> u32 {
    let mut enn = EulerNumberNumerator {
        prev: bignum::BigNum::new(3),
        value: bignum::BigNum::new(8),
        term: 3,
    };
    while enn.term < 100 {
        enn.term += 1;
        let t = enn.value.clone();
        if enn.term % 3 == 0 {
            let k = enn.term / 3;
            enn.value.multiply(2 * k);
        }
        enn.value.add(&enn.prev);
        enn.prev = t;
    }
    enn.value.digit_sum()
}

struct EulerNumberSeries {
    prev: u64,
    value: u64,
    term: u8,
}

fn main() {
    let mut numerator = EulerNumberSeries {
        prev: 3,
        value: 8,
        term: 3,
    };
    let mut denominator = EulerNumberSeries {
        prev: 1,
        value: 3,
        term: 3,
    };
    for _ in 0..20 {
        for n in [&mut numerator, &mut denominator].iter_mut() {
            n.term += 1;
            let t = n.value.clone();
            if n.term % 3 == 0 {
                let k = n.term / 3;
                n.value += 2 * k as u64;
            }
            n.value += n.prev;
            n.prev = t;
        }
    }
    println!("{}", numerator.value);
}
