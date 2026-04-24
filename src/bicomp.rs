use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BiCompNum(pub f32, pub f32, pub f32, pub f32);

impl BiCompNum {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        BiCompNum(a, b, c, d)
    }

    pub fn new_i(a: i32, b: i32, c: i32, d: i32) -> Self {
        BiCompNum(a as f32, b as f32, c as f32, d as f32)
    }

    pub fn zero() -> Self {
        BiCompNum(0., 0., 0., 0.)
    }

    pub fn one() -> Self {
        BiCompNum(1., 0., 0., 0.)
    }

    pub fn i() -> Self {
        BiCompNum(0., 1., 0., 0.)
    }

    pub fn epsilon() -> Self {
        BiCompNum(0., 0., 1., 0.)
    }

    pub fn square(self) -> Self {
        BiCompNum(
            self.0.powi(2) - self.1.powi(2),
            2.0 * self.0 * self.1,
            2.0 * self.0 * self.2 - 2.0 * self.1 * self.3,
            2.0 * self.0 * self.3 + 2.0 * self.1 * self.2,
        )
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0f32 && self.1 == 0f32 && self.2 == 0f32 && self.3 == 0f32
    }

    pub fn exp(&self) -> BiCompNum {
        const PRECISION: u32 = 10;

        let mut term = BiCompNum::one();
        let mut res = term;

        for n in 1..=PRECISION {
            term = term * *self / n;
            res += term;
        }
        res
    }
}

impl Display for BiCompNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} + {}i + {}ε + {}iε",
            self.0, self.1, self.2, self.3
        ))
    }
}

impl Add for BiCompNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        BiCompNum(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl AddAssign for BiCompNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Div<f32> for BiCompNum {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        BiCompNum(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl Div<i32> for BiCompNum {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self / (rhs as f32)
    }
}

impl Div<u32> for BiCompNum {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        self / (rhs as f32)
    }
}

impl Sub for BiCompNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        BiCompNum(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Mul for BiCompNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            BiCompNum::zero()
        } else {
            BiCompNum(
                self.0 * rhs.0 - self.1 * rhs.1,
                self.0 * rhs.1 + self.1 * rhs.0,
                self.0 * rhs.2 - self.1 * rhs.3 + rhs.0 * self.2 - self.3 * rhs.1,
                self.0 * rhs.3 + self.1 * rhs.2 + rhs.1 * self.2 + rhs.0 * self.3,
            )
        }
    }
}

impl MulAssign for BiCompNum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let n = BiCompNum::new(1.5, 2.5, 3.5, 4.5);
        assert_eq!(n.0, 1.5);
        assert_eq!(n.1, 2.5);
        assert_eq!(n.2, 3.5);
        assert_eq!(n.3, 4.5);
    }

    #[test]
    fn test_new_i() {
        let n = BiCompNum::new_i(1, 2, 3, 4);
        assert_eq!(n.0, 1.0);
        assert_eq!(n.1, 2.0);
        assert_eq!(n.2, 3.0);
        assert_eq!(n.3, 4.0);
    }

    #[test]
    fn test_zero() {
        let n = BiCompNum::zero();
        assert_eq!(n, BiCompNum(0.0, 0.0, 0.0, 0.0));
        assert!(n.is_zero());
    }

    #[test]
    fn test_one() {
        let n = BiCompNum::one();
        assert_eq!(n, BiCompNum(1.0, 0.0, 0.0, 0.0));
        assert!(!n.is_zero());
    }

    #[test]
    fn test_display() {
        let n = BiCompNum::new_i(1, 2, 3, 4);
        assert_eq!(format!("{}", n), "1 + 2i + 3ε + 4iε");
    }

    #[test]
    fn test_add() {
        let n1 = BiCompNum::new_i(1, 2, 3, 4);
        let n2 = BiCompNum::new_i(5, 6, 7, 8);
        assert_eq!(n1 + n2, BiCompNum::new_i(6, 8, 10, 12));
    }

    #[test]
    fn test_add_assign() {
        let mut n1 = BiCompNum::new_i(1, 2, 3, 4);
        let n2 = BiCompNum::new_i(5, 6, 7, 8);
        n1 += n2;
        assert_eq!(n1, BiCompNum::new_i(6, 8, 10, 12));
    }

    #[test]
    fn test_sub() {
        let n1 = BiCompNum::new_i(10, 10, 10, 10);
        let n2 = BiCompNum::new_i(1, 2, 3, 4);
        assert_eq!(n1 - n2, BiCompNum::new_i(9, 8, 7, 6));
    }

    #[test]
    fn test_div_f32() {
        let n = BiCompNum::new_i(2, 4, 6, 8);
        assert_eq!(n / 2.0, BiCompNum::new_i(1, 2, 3, 4));
    }

    #[test]
    fn test_div_i32() {
        let n = BiCompNum::new_i(2, 4, 6, 8);
        assert_eq!(n / 2_i32, BiCompNum::new_i(1, 2, 3, 4));
    }

    #[test]
    fn test_div_u32() {
        let n = BiCompNum::new_i(2, 4, 6, 8);
        assert_eq!(n / 2_u32, BiCompNum::new_i(1, 2, 3, 4));
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            BiCompNum::zero() * BiCompNum::new_i(1, 0, 0, 0),
            BiCompNum::zero()
        );
        assert_eq!(
            BiCompNum::new_i(0, 2, 0, 4) * BiCompNum::new_i(0, 2, 0, 4),
            BiCompNum::new_i(-4, 0, -16, 0)
        );
        assert_eq!(
            BiCompNum::new_i(1, 1, 1, 1) * BiCompNum::new_i(1, 1, 1, 1),
            BiCompNum::new_i(0, 2, 0, 4)
        );

        let n1 = BiCompNum::new_i(1, 2, 3, 4);
        let n2 = BiCompNum::new_i(5, 6, 7, 8);
        // (1+2i+3e+4ie) * (5+6i+7e+8ie)
        // Real: 1*5 - 2*6 = 5 - 12 = -7
        // i: 1*6 + 2*5 = 6 + 10 = 16
        // e: 1*7 - 2*8 + 5*3 - 6*4 = 7 - 16 + 15 - 24 = -18
        // ie: 1*8 + 2*7 + 6*3 + 5*4 = 8 + 14 + 18 + 20 = 60
        assert_eq!(n1 * n2, BiCompNum::new_i(-7, 16, -18, 60));
    }

    #[test]
    fn test_mul_assign() {
        let mut n1 = BiCompNum::new_i(1, 2, 3, 4);
        let n2 = BiCompNum::new_i(5, 6, 7, 8);
        n1 *= n2;
        assert_eq!(n1, BiCompNum::new_i(-7, 16, -18, 60));
    }

    #[test]
    fn test_square() {
        let num = BiCompNum::new_i(12, -2, 3, 1);
        assert_eq!(num * num, num.square());

        let n = BiCompNum::new_i(1, 1, 1, 1);
        assert_eq!(n.square(), BiCompNum::new_i(0, 2, 0, 4));
    }

    #[test]
    fn test_exp() {
        assert_eq!(BiCompNum::zero().exp(), BiCompNum::one());

        let r = BiCompNum::new_i(0, 0, 1, 0).exp();
        assert_eq!(r, BiCompNum::new_i(1, 0, 1, 0));

        let e = BiCompNum::new_i(1, 1, 1, 1).exp();

        assert!(1.46 < e.0 && e.0 < 1.47);
        assert!(2.28 < e.1 && e.1 < 2.29);
        assert!(-0.82 < e.2 && e.2 < -0.8);
        assert!(3.75 < e.3 && e.3 < 3.76);
    }
}
