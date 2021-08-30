// Problem 66 "Diophantine equation"

// <p>Consider quadratic Diophantine equations of the form:</p>
// <p class="margin_left"><i>x</i><sup>2</sup> – D<i>y</i><sup>2</sup> = 1</p>
// <p>For example, when D=13, the minimal solution in <i>x</i> is 649<sup>2</sup> – 13×180<sup>2</sup> = 1.</p>
// <p>It can be assumed that there are no solutions in positive integers when D is square.</p>
// <p>By finding minimal solutions in <i>x</i> for D = {2, 3, 5, 6, 7}, we obtain the following:</p>
// <p class="margin_left">3<sup>2</sup> – 2×2<sup>2</sup> = 1<br />
// 2<sup>2</sup> – 3×1<sup>2</sup> = 1<br /><span class="red strong">9</span><sup>2</sup> – 5×4<sup>2</sup> = 1<br />
// 5<sup>2</sup> – 6×2<sup>2</sup> = 1<br />
// 8<sup>2</sup> – 7×3<sup>2</sup> = 1</p>
// <p>Hence, by considering minimal solutions in <i>x</i> for D ≤ 7, the largest <i>x</i> is obtained when D=5.</p>
// <p>Find the value of D ≤ 1000 in minimal solutions of <i>x</i> for which the largest value of <i>x</i> is obtained.</p>

mod tests {
    use super::*;

    #[test]
    fn test_diophantine_equation_1() {
        assert_eq!(diophantine_equation_1(), 661);
    }
}

mod fraction {
    trait MixedFraction {
        fn integer_part(&self) -> u32;
    }

    pub struct QuadraticIrrational {
        rational_part: u32,
        irrational_part: u32,
        downscaling_ratio: u32,
        pub integer_part: u32,
        pub iteration: u32,
        integers: Vec<u32>,
    }

    impl MixedFraction for QuadraticIrrational {
        fn integer_part(&self) -> u32 {
            let mixed_fraction = self.rational_part as f64 + (self.irrational_part as f64).sqrt();
            (mixed_fraction / self.downscaling_ratio as f64) as u32
        }
    }

    impl QuadraticIrrational {
        pub fn new(square_free_integer: u32) -> Self {
            assert!(((square_free_integer as f64).sqrt() as u32).pow(2) != square_free_integer);
            let mut q = Self {
                rational_part: 0,
                irrational_part: square_free_integer,
                downscaling_ratio: 1,
                integer_part: u32::MAX,
                iteration: 0,
                integers: vec![],
            };
            q.integer_part = q.integer_part();
            q.integers.push(q.integer_part);
            q
        }
        pub fn next_iteration(&mut self) {
            self.iteration += 1;
            self.rational_part = self.downscaling_ratio * self.integer_part - self.rational_part;
            self.downscaling_ratio =
                (self.irrational_part - self.rational_part.pow(2)) / self.downscaling_ratio;
            self.integer_part = self.integer_part();
            self.integers.push(self.integer_part);
        }
        pub fn last_numerator_approx(&self) -> f64 {
            let mut result = self.integers[0] as f64;
            let mut term_before_last = 1f64;
            for a in (&self.integers[1..self.integers.len() - 1])
                .iter()
                .map(|&a| a as f64)
            {
                let t = result.clone();
                result *= a;
                result += term_before_last;
                term_before_last = t;
            }
            result
        }
        pub fn last_denominator_approx(&self) -> f64 {
            let mut result = 1f64;
            let mut term_before_last = 0f64;
            for a in (&self.integers[1..self.integers.len() - 1])
                .iter()
                .map(|&a| a as f64)
            {
                let t = result.clone();
                result *= a;
                result += term_before_last;
                term_before_last = t;
            }
            result
        }
    }
}

/// ```rust
/// use self::project_euler::m66::diophantine_equation_1;
/// assert_eq!(diophantine_equation_1(), 661);
/// ```
pub fn diophantine_equation_1() -> u32 {
    let mut max_numerator_aprox = 0f64;
    let mut max_d = 0u32;

    for n in 2..=1000 {
        if ((n as f64).sqrt() as u32).pow(2) == n {
            continue;
        }
        let mut surd = fraction::QuadraticIrrational::new(n);
        let integer_part_orig = surd.integer_part;
        while {
            surd.next_iteration();
            surd.integer_part != integer_part_orig * 2 || surd.iteration % 2 != 0
        } {}

        let numerator_approx = surd.last_numerator_approx();
        println!("{} {} {} {} {}", n, surd.iteration, numerator_approx / surd.last_denominator_approx(),numerator_approx, surd.last_denominator_approx());
        if numerator_approx > max_numerator_aprox {
            max_numerator_aprox = numerator_approx;
            max_d = n;
        }
    }
    max_d
}
