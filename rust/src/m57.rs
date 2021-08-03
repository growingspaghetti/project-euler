// <p>It is possible to show that the square root of two can be expressed as an infinite continued fraction.</p>
// <p class="center">$\sqrt 2 =1+ \frac 1 {2+ \frac 1 {2 +\frac 1 {2+ \dots}}}$</p>
// <p>By expanding this for the first four iterations, we get:</p>
// <p>$1 + \frac 1 2 = \frac  32 = 1.5$<br />
// $1 + \frac 1 {2 + \frac 1 2} = \frac 7 5 = 1.4$<br />
// $1 + \frac 1 {2 + \frac 1 {2+\frac 1 2}} = \frac {17}{12} = 1.41666 \dots$<br />
// $1 + \frac 1 {2 + \frac 1 {2+\frac 1 {2+\frac 1 2}}} = \frac {41}{29} = 1.41379 \dots$<br /></p>
// <p>The next three expansions are $\frac {99}{70}$, $\frac {239}{169}$, and $\frac {577}{408}$, but the eighth expansion, $\frac {1393}{985}$, is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.</p>
// <p>In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?</p>

struct Fraction {
    deno_prev: f64,
    deno: f64,
    nume: f64,
    n: u32,
}

impl Fraction {
    fn new() -> Self {
        let deno_prev = 2f64;
        let deno = 5f64;
        Self {
            deno_prev,
            deno,
            nume: deno_prev + deno,
            n: 2,
        }
    }
    fn increment(&mut self) {
        self.n += 1;
        self.deno_prev = self.deno;
        self.deno += self.nume;
        self.nume = self.deno_prev + self.deno;
        while self.deno.exp() > 1e+17 && self.nume.exp() > 1e+17 {
            self.deno /= 1e+1;
            self.nume /= 1e+1;
        }
        //dbg!(self.nume, self.deno);
    }
    fn has_numerator_more_digit(&self) -> bool {
        //dbg!(self.nume.exp(), self.deno.exp());
        self.nume.log10().floor() > self.deno.log10().floor()
    }
}

// 70 us
/// ```rust
/// use self::project_euler::m57::fractions_contain_a_numerator_with_more_digits;
/// assert_eq!(fractions_contain_a_numerator_with_more_digits(), 153);
/// ```
/// 10000 1508 100000 15052 1000000 150520 150519? 10000000 1505158
pub fn fractions_contain_a_numerator_with_more_digits() -> u32 {
    let mut count = 0u32;
    let mut frac = Fraction::new();
    while {
        if frac.has_numerator_more_digit() {
            count += 1;
        }
        frac.increment();
        //dbg!(frac.n, frac.deno, frac.nume);
        frac.n <= 10
    } {}
    count
}

struct FractionU64 {
    deno_prev: u64,
    deno: u64,
    nume: u64,
    n: u32,
}

impl FractionU64 {
    fn new() -> Self {
        let deno_prev = 2u64;
        let deno = 5u64;
        Self {
            deno_prev,
            deno,
            nume: deno_prev + deno,
            n: 2,
        }
    }
    fn increment(&mut self) {
        self.n += 1;
        self.deno_prev = self.deno;
        self.deno += self.nume;
        self.nume = self.deno_prev + self.deno;
        const THRESHOLD: u64 = {
            let mut threshold = 1u64;
            while threshold < u64::MAX / 100 {
                threshold *= 10;
            }
            threshold
        };
        if self.deno > THRESHOLD && self.nume > THRESHOLD {
            self.deno /= 10;
            self.nume /= 10;
        }
        //dbg!(self.nume, self.deno);
    }
    fn has_numerator_more_digit(&self) -> bool {
        //dbg!(self.nume.exp(), self.deno.exp());
        // self.nume.log10().floor() > self.deno.log10().floor()
        let mut n = self.nume;
        let mut d = self.deno;
        while n > 0 && d > 0 {
            n /= 10;
            d /= 10;
        }
        n > d
    }
}

// 41 us
/// ```rust
/// use self::project_euler::m57::fractions_contain_a_numerator_with_more_digits_u64;
/// assert_eq!(fractions_contain_a_numerator_with_more_digits_u64(), 153);
/// ```
pub fn fractions_contain_a_numerator_with_more_digits_u64() -> u32 {
    let mut count = 0u32;
    let mut frac = FractionU64::new();
    while {
        if frac.has_numerator_more_digit() {
            count += 1;
        }
        frac.increment();
        //dbg!(frac.n, frac.deno, frac.nume);
        frac.n <= 1000
    } {}
    count
}

// 22 us 3.8 us
/// ```rust
/// use self::project_euler::m57::fractions_contain_a_numerator_with_more_digits_linear;
/// assert_eq!(fractions_contain_a_numerator_with_more_digits_linear(), 153);
/// ```
pub fn fractions_contain_a_numerator_with_more_digits_linear() -> u32 {
    let r1 = 1f64 + 2f64.sqrt();
    let r2 = 1f64 - 2f64.sqrt();
    let deno_c = 1.5f64 * 2f64.log10();
    let nume_c = 2f64.log10();
    let log10_r1 = r1.log10();
    //let r2_over_r1 = r2 / r1;

    let mut count = 0u32;
    //let mut pow = r2_over_r1.clone();
    for n in 2u32..=1001 {
        //pow *= r2_over_r1;
        //let deno_digit = (n as f64 * log10_r1 + (1f64 - pow).log10() - deno_c) as u32;
        //let nume_digit = (n as f64 * log10_r1 + (1f64 + pow).log10() - nume_c) as u32;
        let deno_digit = (n as f64 * log10_r1 - deno_c) as u32;
        let nume_digit = (n as f64 * log10_r1 - nume_c) as u32;
        if nume_digit > deno_digit {
            count += 1;
        }
    }
    count
}

#[derive(Clone)]
struct BigNum {
    v: Vec<u64>,
}

impl BigNum {
    const TEN_MIL: u64 = 10_000_000_000;
    fn new(n: u8) -> Self {
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
    fn double(&mut self) -> &Self {
        let mut carry = 0u64;
        for con in self.v.iter_mut() {
            *con *= 2;
            *con += carry;
            if *con < Self::TEN_MIL {
                carry = 0;
                continue;
            }
            carry = 1;
            *con -= Self::TEN_MIL;
        }
        if carry != 0 {
            self.v.push(1u64);
        }
        self
    }
    fn add(&mut self, b: &BigNum) {
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
}

struct FractionBigNum {
    deno: BigNum,
    nume: BigNum,
    n: u32,
}

impl FractionBigNum {
    fn new() -> Self {
        Self {
            deno: BigNum::new(5),
            nume: BigNum::new(7),
            n: 2,
        }
    }
    fn increment(&mut self) {
        self.n += 1;
        let t = self.nume.clone();
        self.nume.add(self.deno.clone().double());
        self.deno.add(&t);
    }
    fn has_numerator_more_digit(&self) -> bool {
        //dbg!(self.nume.exp(), self.deno.exp());
        // self.nume.log10().floor() > self.deno.log10().floor()
        if self.nume.v.len() != self.deno.v.len() {
            return self.nume.v.len() > self.deno.v.len();
        }
        let mut n = *self.nume.v.last().expect("BigNum must have a container");
        let mut d = *self.deno.v.last().expect("BigNum must have a container");
        while n > 0 && d > 0 {
            n /= 10;
            d /= 10;
        }
        n > d
    }
}

// 527 us
/// ```rust
/// use self::project_euler::m57::fractions_contain_a_numerator_with_more_digits_bignum;
/// assert_eq!(fractions_contain_a_numerator_with_more_digits_bignum(), 153);
/// ```
pub fn fractions_contain_a_numerator_with_more_digits_bignum() -> u32 {
    let mut count = 0u32;
    let mut frac = FractionBigNum::new();
    while {
        if frac.has_numerator_more_digit() {
            count += 1;
        }
        frac.increment();
        //dbg!(frac.n, frac.deno, frac.nume);
        frac.n <= 1000
    } {}
    count
}

mod tests {
    use super::*;

    #[test]
    fn test_bignum() {
        fractions_contain_a_numerator_with_more_digits_bignum();
    }
}
