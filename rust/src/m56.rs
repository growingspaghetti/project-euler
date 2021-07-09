// <p>A googol (10<sup>100</sup>) is a massive number: one followed by one-hundred zeros; 100<sup>100</sup> is almost unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the digits in each number is only 1.</p>
// <p>Considering natural numbers of the form, <i>a<sup>b</sup></i>, where <i>a, b</i> &lt; 100, what is the maximum digital sum?</p>

struct BigNum {
    _base: u64,
    _exp: u8,
    _v: Vec<u64>,
}

impl BigNum {
    const TEN_MIL: u64 = 10_000_000_000;
    fn new(base: u8, exp: u8) -> Self {
        BigNum {
            _base: base as u64,
            _exp: exp,
            _v: vec![1],
        }
    }
    fn _multiply_by_base(&mut self) {
        let mut carry = 0u64;
        for con in self._v.iter_mut() {
            *con = *con * self._base + carry;
            //*con *= self._base;
            //*con += carry;
            if *con < Self::TEN_MIL {
                carry = 0;
                continue;
            }
            carry = *con / Self::TEN_MIL;
            *con %= Self::TEN_MIL;
        }
        if carry != 0 {
            self._v.push(carry);
        }
    }
    fn compute(&mut self) {
        for _ in 0..self._exp {
            self._multiply_by_base();
        }
    }
    fn digit_sum(&self) -> u32 {
        let mut sum = 0u32;
        for &con in &self._v {
            let mut t = con;
            while t > 0 {
                sum += (t % 10) as u32;
                t /= 10;
            }
        }
        sum
    }
}

// 8.4 ms
/// ```rust
/// use self::project_euler::m56::a_exp_b_100_what_is_the_maximum_digital_sum;
/// assert_eq!(a_exp_b_100_what_is_the_maximum_digital_sum(), 972);
/// ```
pub fn a_exp_b_100_what_is_the_maximum_digital_sum() -> u32 {
    let mut max = 0u32;
    for a in 2u8..100 {
        if a % 10 == 0 {
            continue;
        }
        for b in 2u8..100 {
            let mut bignum = BigNum::new(a, b);
            bignum.compute();
            max = std::cmp::max(max, bignum.digit_sum());
        }
    }
    max
}



struct BigNumReuse {
    _v: Vec<u64>,
}

impl BigNumReuse {
    const TEN_MIL: u64 = 10_000_000_000;
    fn new() -> Self {
        BigNumReuse {
            _v: Vec::with_capacity(30),
        }
    }
    fn compute(&mut self, base: u8, exp: u8) {
        self._v.clear();
        self._v.push(1);
        for _ in 0..exp {
            self._multiply(base as u64);
        }
    }
    fn _multiply(&mut self, n: u64) {
        let mut carry = 0u64;
        for con in self._v.iter_mut() {
            *con = *con * n + carry;
            if *con < Self::TEN_MIL {
                carry = 0;
                continue;
            }
            carry = *con / Self::TEN_MIL;
            *con %= Self::TEN_MIL;
        }
        if carry != 0 {
            self._v.push(carry);
        }
    }
    fn digit_sum(&self) -> u32 {
        let mut sum = 0u32;
        for &con in &self._v {
            let mut t = con;
            while t > 0 {
                sum += (t % 10) as u32;
                t /= 10;
            }
        }
        sum
    }
}

// 7.25 ms
/// ```rust
/// use self::project_euler::m56::a_exp_b_100_what_is_the_maximum_digital_sum_reuse;
/// assert_eq!(a_exp_b_100_what_is_the_maximum_digital_sum_reuse(), 972);
/// ```
pub fn a_exp_b_100_what_is_the_maximum_digital_sum_reuse() -> u32 {
    let mut bignum = BigNumReuse::new();
    let mut max = 0u32;
    for a in 2u8..100 {
        if a % 10 == 0 {
            continue;
        }
        for b in 2u8..100 {
            bignum.compute(a, b);
            max = std::cmp::max(max, bignum.digit_sum());
        }
    }
    max
}


struct BigNumGradual {
    _v: Vec<u64>,
}

impl BigNumGradual {
    const TEN_MIL: u64 = 10_000_000_000;
    fn new() -> Self {
        BigNumGradual {
            _v: vec![],//Vec::with_capacity(50),
        }
    }
    fn init(&mut self, base: u8) {
        self._v.clear();
        self._v.push(base as u64);
    }
    fn multiply(&mut self, n: u8) {
        let mut carry = 0u64;
        for con in self._v.iter_mut() {
            *con = *con * n as u64 + carry;
            if *con < Self::TEN_MIL {
                carry = 0;
                continue;
            }
            carry = *con / Self::TEN_MIL;
            *con %= Self::TEN_MIL;
        }
        if carry != 0 {
            self._v.push(carry);
        }
    }
    fn digit_sum(&self) -> u32 {
        let mut sum = 0u32;
        for &con in &self._v {
            let mut t = con;
            while t > 0 {
                sum += (t % 10) as u32;
                t /= 10;
            }
        }
        sum
    }
}


// 1.6 ms
/// ```rust
/// use self::project_euler::m56::a_exp_b_100_what_is_the_maximum_digital_sum_gradual;
/// assert_eq!(a_exp_b_100_what_is_the_maximum_digital_sum_gradual(), 972);
/// ```
pub fn a_exp_b_100_what_is_the_maximum_digital_sum_gradual() -> u32 {
    let mut bignum = BigNumGradual::new();
    let mut max = 0u32;
    for a in 2u8..100 {
        if a % 10 == 0 {
            continue;
        }
        bignum.init(a);
        for _ in 2u8..100 {
            bignum.multiply(a);
            max = std::cmp::max(max, bignum.digit_sum());
        }
    }
    max
}
