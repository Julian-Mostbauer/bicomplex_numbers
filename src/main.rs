use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

fn main() {
    let n = BiCompNum::new_i(1, 1, 1, 1);
    println!("{}", n.square())
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct BiCompNum(f32, f32, f32, f32);

impl BiCompNum {
    pub fn new_i(a: i32, b: i32, c: i32, d: i32) -> Self {
        BiCompNum(a as f32, b as f32, c as f32, d as f32)
    }

    pub fn square(self) -> Self {
        BiCompNum(
            self.0.powi(2) - self.1.powi(2),
            2.0 * self.0 * self.1,
            2.0 * self.0 * self.2 - 2.0 * self.1 * self.3,
            2.0 * self.0 * self.3 + 2.0 * self.1 * self.2,
        )
    }

    pub fn zero() -> Self {
        BiCompNum(0f32, 0f32, 0f32, 0f32)
    }

    fn one() -> Self {
        BiCompNum(1f32, 0f32, 0f32, 0f32)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0f32 && self.1 == 0f32 && self.2 == 0f32 && self.3 == 0f32
    }

    pub fn exp(&self) -> BiCompNum {
        const PRECISION: u32 = 10;

        let mut res = BiCompNum::one();
        let mut numerator: BiCompNum = self.clone();
        let mut denominator: u32 = 2;

        for i in 3..(3 + PRECISION) {
            res += numerator / denominator;
            numerator *= *self;
            denominator *= i;
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
    fn test_add() {
        assert_eq!(
            BiCompNum::new_i(1, 0, 0, 0) + BiCompNum::new_i(1, 0, 0, 0),
            BiCompNum::new_i(2, 0, 0, 0)
        );
        assert_eq!(
            BiCompNum::new_i(1, 1, 1, 1) + BiCompNum::new_i(1, 1, 1, 1),
            BiCompNum::new_i(2, 2, 2, 2)
        );

        assert_eq!(
            BiCompNum(-1f32, 0f32, 0.5f32, 0f32) + BiCompNum::new_i(1, 1, 1, 1),
            BiCompNum(0f32, 1f32, 1.5f32, 1f32)
        );
    }
    #[test]
    fn test_mul() {
        assert_eq!(BiCompNum::zero() * BiCompNum::new_i(1, 0, 0, 0), BiCompNum::zero());
        assert_eq!(
            BiCompNum::new_i(0, 2, 0, 4) * BiCompNum::new_i(0, 2, 0, 4),
            BiCompNum::new_i(-4, 0, -16, 0)
        );
        assert_eq!(
            BiCompNum::new_i(1, 1, 1, 1) * BiCompNum::new_i(1, 1, 1, 1),
            BiCompNum::new_i(0, 2, 0, 4)
        );
        assert_eq!(
            BiCompNum::new_i(0, 2, 0, 4) * BiCompNum::new_i(1, 1, 1, 1),
            BiCompNum::new_i(-2, 2, -6, 6)
        );

        assert_eq!(BiCompNum::zero() * BiCompNum::new_i(1, 1, 1, 1), BiCompNum::zero());
    }

    #[test]
    fn test_square() {
        let num = BiCompNum::new_i(12, -2, 3, 1);
        assert_eq!(num * num, num.square())
    }
}
