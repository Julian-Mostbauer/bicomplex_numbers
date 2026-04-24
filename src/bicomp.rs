use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::Complex;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BiCompNum(pub f64, pub f64, pub f64, pub f64);

impl BiCompNum {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        BiCompNum(a, b, c, d)
    }

    pub fn new_i(a: i32, b: i32, c: i32, d: i32) -> Self {
        BiCompNum(a as f64, b as f64, c as f64, d as f64)
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
        self.0 == 0f64 && self.1 == 0f64 && self.2 == 0f64 && self.3 == 0f64
    }

    pub fn is_one(&self) -> bool {
        self.0 == 1f64 && self.1 == 0f64 && self.2 == 0f64 && self.3 == 0f64
    }

    fn z(&self) -> Complex {
        Complex::new(self.0, self.1)
    }

    fn w(&self) -> Complex {
        Complex::new(self.2, self.3)
    }

    fn from_complex(z: Complex, w: Complex) -> Self {
        Self(z.re, z.im, w.re, w.im)
    }

    pub fn first_half(&self) -> BiCompNum {
        BiCompNum(self.0, self.1, 0., 0.)
    }

    pub fn second_half(&self) -> BiCompNum {
        BiCompNum(0., 0., self.2, self.3)
    }

    pub fn exp_tailor(&self, precision: u32) -> BiCompNum {
        assert!(precision >= 1, "Precision must be at least 1");

        let mut term = BiCompNum::one();
        let mut res = term;

        for n in 1..=precision {
            term = term * *self / n;
            res += term;
        }
        res
    }

    pub fn exp(&self) -> BiCompNum {
        let z = BiCompNum(self.1.cos(), self.1.sin(), 0., 0.) * self.0.exp();
        let w = self.second_half() + Self::one();

        z * w
    }

    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.0 + rhs.1 == 0. {
            return None;
        }

        Some(self / rhs)
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

impl SubAssign for BiCompNum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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

impl std::ops::Div for BiCompNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let z1 = self.z();
        let w1 = self.w();

        let z2 = rhs.z();
        let w2 = rhs.w();

        let z = z1.div(z2);

        let numerator = w1.mul(z2).mul(Complex::new(1.0, 0.0)).re - z1.mul(w2).re;

        let numerator_i = w1.mul(z2).im - z1.mul(w2).im;

        let z2_sq = z2.mul(z2);

        let w = Complex::new(numerator, numerator_i).div(z2_sq);

        BiCompNum::from_complex(z, w)
    }
}

impl<T> std::ops::Mul<T> for BiCompNum
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        BiCompNum(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl<T> std::ops::Div<T> for BiCompNum
where
    T: Into<f64>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        BiCompNum(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod helper {
        use super::*;

        pub fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
            (a - b).abs() < eps
        }

        pub fn approx_eq_bi(a: BiCompNum, b: BiCompNum, eps: f64) -> bool {
            approx_eq(a.0, b.0, eps)
                && approx_eq(a.1, b.1, eps)
                && approx_eq(a.2, b.2, eps)
                && approx_eq(a.3, b.3, eps)
        }
    }

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
    fn test_div_f64() {
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

    #[test]
    fn div_by_one() {
        let x = BiCompNum(2.0, -3.0, 4.0, 5.0);
        let one = BiCompNum(1.0, 0.0, 0.0, 0.0);

        let result = x.checked_div(one).unwrap();
        assert!(helper::approx_eq_bi(result, x, 1e-5));
    }

    #[test]
    fn div_complex_only() {
        let x = BiCompNum(1.0, 2.0, 0.0, 0.0);
        let y = BiCompNum(3.0, -1.0, 0.0, 0.0);

        let result = x.checked_div(y).unwrap();

        // Expected: (1+2i)/(3-i) = (1+2i)*(3+i)/(10) = (1 + 7i)/10
        let expected = BiCompNum(0.1, 0.7, 0.0, 0.0);

        assert!(helper::approx_eq_bi(result, expected, 1e-5));
    }
    #[test]

    fn div_epsilon_numerator() {
        let x = BiCompNum(0.0, 0.0, 1.0, 2.0); // ε(1+2i)
        let y = BiCompNum(1.0, 0.0, 0.0, 0.0); // 1

        let result = x.checked_div(y).unwrap();

        assert!(helper::approx_eq_bi(result, x, 1e-5));
    }
    #[test]
    fn div_epsilon_in_denominator() {
        let x = BiCompNum(1.0, 0.0, 0.0, 0.0); // 1
        let y = BiCompNum(1.0, 0.0, 1.0, 0.0); // 1 + ε

        let result = x.checked_div(y).unwrap();

        // (1) / (1 + ε) = 1 - ε
        let expected = BiCompNum(1.0, 0.0, -1.0, 0.0);

        assert!(helper::approx_eq_bi(result, expected, 1e-5));
    }
    #[test]
    fn div_general_case() {
        let x = BiCompNum(1.0, 2.0, 3.0, 4.0);
        let y = BiCompNum(2.0, -1.0, 1.0, 1.0);

        let result = x.checked_div(y).unwrap();

        // Validate via inverse: result * y ≈ x
        let recomposed = result * y;
        dbg!(recomposed);

        assert!(helper::approx_eq_bi(recomposed, x, 1e-4));
    }

    #[test]
    fn div_inverse_property() {
        let x = BiCompNum(2.0, 1.0, 3.0, -1.0);
        let y = BiCompNum(1.5, -0.5, 2.0, 0.5);

        let r1 = x.checked_div(y).unwrap();
        let r2 = y.checked_div(x).unwrap();

        let one = r1 * r2;

        let expected = BiCompNum(1.0, 0.0, 0.0, 0.0);
        dbg!(one);

        assert!(helper::approx_eq_bi(one, expected, 1e-4));
    }
    #[test]
    fn div_non_invertible_pure_epsilon() {
        let x = BiCompNum(1.0, 0.0, 0.0, 0.0);
        let y = BiCompNum(0.0, 0.0, 1.0, 0.0); // ε

        assert!(x.checked_div(y).is_none());
    }

    #[test]
    fn div_division_by_zero() {
        let x = BiCompNum(1.0, 2.0, 3.0, 4.0);
        let y = BiCompNum(0.0, 0.0, 0.0, 0.0);

        assert!(x.checked_div(y).is_none());
    }

    #[test]
    fn div_non_invertible_no_complex_part() {
        let x = BiCompNum(1.0, 0.0, 0.0, 0.0);
        let y = BiCompNum(0.0, 0.0, 2.0, 3.0); // only ε part

        assert!(x.checked_div(y).is_none());
    }
}
