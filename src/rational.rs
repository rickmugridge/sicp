use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use crate::gcd::gcd;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rational {
    numerator: isize,
    denominator: isize,
}

impl Rational {
    fn new(numerator: isize, denominator: isize) -> Self {
        let g = gcd(numerator, denominator);
        Self { numerator: numerator / g, denominator: (denominator / g).abs() }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}/{})", self.numerator, self.denominator)
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make() {
        let r1 = Rational::new(-1, -2);
        assert_eq!(r1, Rational::new(-1, 2));
    }

    #[test]
    fn make_gcd() {
        let r1 = Rational::new(4, 16);
        assert_eq!(r1, Rational::new(1, 4));
    }

    #[test]
    fn add() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 4);
        assert_eq!(r1 + r2, Rational::new(3, 4));
        // println!("r1 is {r1}");
    }

    #[test]
    fn subtract() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 4);
        assert_eq!(r1 - r2, Rational::new(1, 4));
    }
}