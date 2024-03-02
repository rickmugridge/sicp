use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    pub fn zero() -> Self {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Self {
        Vector { x: 1.0, y: 1.0 }
    }

    pub fn scale(&self, s: f32) -> Self {
        Vector { x: self.x * s, y: self.y * s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Vector::new(1.0, 2.0) + Vector::new(3.0, 4.0),
                   Vector::new(4.0, 6.0));
    }

    #[test]
    fn subtract() {
        assert_eq!(Vector::new(1.0, 2.0) - Vector::new(3.0, 4.0),
                   Vector::new(-2.0, -2.0));
    }

    #[test]
    fn scale() {
        assert_eq!(Vector::new(1.0, 2.0).scale(10.0),
                   Vector::new(10.0, 20.0));
    }
}