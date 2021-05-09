//! See [m16](./m16.rs)

/// n! means n × (n − 1) × ... × 3 × 2 × 1
///
/// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
/// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
///
/// Find the sum of the digits in the number 100!
///
/// ```rust
/// use self::project_euler::m20::sum_of_the_digits_in_the_number_factorial_100;
/// assert_eq!(sum_of_the_digits_in_the_number_factorial_100(), 648);
/// ```
#[allow(clippy::needless_range_loop)]
pub fn sum_of_the_digits_in_the_number_factorial_100() -> u64 {
    fn multiply_by_num(containers: &mut std::vec::Vec<u64>, num: u64) {
        let mut carry = 0u64;
        for i in 0..containers.len() {
            containers[i] = containers[i] * num + carry;
            if containers[i] > 1_000_000_000_000_000 {
                carry = containers[i] / 1_000_000_000_000_000;
                containers[i] -= 1_000_000_000_000_000 * carry;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            containers.push(carry);
        }
    }
    let mut containers: Vec<u64> = vec![];
    containers.push(1);

    let mut sum = 0u64;
    for i in 0..100u64 {
        // println!("{} {:?}", 100 - i, containers);
        multiply_by_num(&mut containers, 100 - i);
    }
    for &container in &containers {
        let mut tmp = container;
        while tmp > 0 {
            let d = tmp % 10;
            sum += d;
            tmp /= 10;
        }
    }
    // [0, 916864000000000, 223758251185210, 286253697920827, 941463976156518, 599993229915608, 468592963895217, 715968264381621, 238856266700490, 443944152681699, 93326215]
    // println!("{:?}", containers);
    sum
}


struct BigNum {
    v: Vec<u64>,
}

impl BigNum {
    const TEN_MIL: u64 = 10_000_000_000;
    fn factorial(&mut self) {
        assert!(self.v.len() == 1);
        let up = self.v[0];
        assert!(((u64::MAX as f32 - Self::TEN_MIL as f32) / Self::TEN_MIL as f32) > (up as f32));
        for b in 2..up {
            self._multiply(b);
        }
    }
    fn _multiply(&mut self, b: u64) {
        let mut carry = 0u64;
        for con in self.v.iter_mut() {
            *con *= b;
            *con += carry;
            if *con < Self::TEN_MIL {
                carry = 0;
                continue;
            }
            carry = *con / Self::TEN_MIL;
            *con -= carry * Self::TEN_MIL;
        }
        if carry != 0 {
            self.v.push(carry);
        }
    }
    fn sum_of_digits(&self) -> u32 {
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

/// ```rust
/// use self::project_euler::m20::sum_of_the_digits_in_the_number_factorial_100_2;
/// assert_eq!(sum_of_the_digits_in_the_number_factorial_100_2(), 648);
/// ```
pub fn sum_of_the_digits_in_the_number_factorial_100_2() -> u32 {
    let mut a = BigNum { v: vec![100u64] };
    a.factorial();
    a.sum_of_digits()
}
