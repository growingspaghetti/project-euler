mod tests {
    use super::*;

    #[test]
    fn test_odd_period_quadratic_irrationals_1() {
        assert_eq!(odd_period_quadratic_irrationals_1(), 1322);
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
            };
            q.integer_part = q.integer_part();
            q
        }
        pub fn next_iteration(&mut self) {
            self.iteration += 1;
            self.rational_part = self.downscaling_ratio * self.integer_part - self.rational_part;
            self.downscaling_ratio =
                (self.irrational_part - self.rational_part.pow(2)) / self.downscaling_ratio;
            self.integer_part = self.integer_part();
        }
    }
}

/// ```rust
/// use self::project_euler::m64::odd_period_quadratic_irrationals_1;
/// assert_eq!(odd_period_quadratic_irrationals_1(), 1322);
/// ```
pub fn odd_period_quadratic_irrationals_1() -> u32 {
    let mut count = 0u32;
    for n in 2..=10000 {
        if ((n as f64).sqrt() as u32).pow(2) == n {
            continue;
        }
        let mut surd = fraction::QuadraticIrrational::new(n);
        let integer_part_orig = surd.integer_part;
        while {
            surd.next_iteration();
            surd.integer_part != integer_part_orig * 2
        } {}
        if surd.iteration % 2 != 0 {
            count += 1;
        }
    }
    count
}
