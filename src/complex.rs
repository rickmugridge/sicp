#[derive(Debug, PartialEq, Clone)]
enum Complex {
    Rectangular(f32, f32),
    Polar(f32, f32),
}

impl Complex {
    fn new_rectangular(real: f32, imaginary: f32) -> Self {
        Self::Rectangular(real, imaginary)
    }

    fn new_polar(magnitude: f32, angle: f32) -> Self {
        Self::Polar(magnitude, angle)
    }

    fn real(&self) -> f32 {
        match self {
            Self::Rectangular(real, _) => *real,
            Self::Polar(magnitude, angle) => *magnitude * angle.cos(),
        }
    }

    fn imaginary(&self) -> f32 {
        match self {
            Self::Rectangular(_, imaginary) => *imaginary,
            Self::Polar(magnitude, angle) => *magnitude * angle.sin(),
        }
    }

    fn magnitude(&self) -> f32 {
        match self {
            Self::Rectangular(real, imaginary) => (real * real + imaginary * imaginary).sqrt(),
            Self::Polar(magnitude, _) => *magnitude,
        }
    }

    fn angle(&self) -> f32 {
        match self {
            Self::Rectangular(real, imaginary) => imaginary.atan2(*real),
            Self::Polar(_, angle) => *angle,
        }
    }

    fn plus(&self, other: &Self) -> Self {
        Self::new_rectangular(self.real() + other.real(),
                              self.imaginary() + other.imaginary())
    }

    fn minus(&self, other: &Self) -> Self {
        Self::new_rectangular(self.real() - other.real(),
                              self.imaginary() - other.imaginary())
    }

    fn times(&self, other: &Self) -> Self {
        Self::new_polar(self.magnitude() * other.magnitude(),
                        self.angle() + other.angle())
    }

    fn divided_by(&self, other: &Self) -> Self {
        Self::new_polar(self.magnitude() / other.magnitude(),
                        self.angle() - other.angle())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangular() {
        let r = Complex::new_rectangular(12.2, 13.3);
        assert_eq!(r.real(), 12.2);
        assert_eq!(r.imaginary(), 13.3);
        assert_eq!(r.magnitude(), 18.04799);
        assert_eq!(r.angle(), 0.8285087);
    }

    #[test]
    fn polar() {
        let r = Complex::new_polar(18.04799, 0.8285087);
        assert_eq!(r.real(), 12.2);
        assert_eq!(r.imaginary(), 13.299999);
        assert_eq!(r.magnitude(), 18.04799);
        assert_eq!(r.angle(), 0.8285087);
    }

    #[test]
    fn plus() {
        let r = Complex::new_rectangular(12.2, 13.3);
        assert_eq!(r.plus(&r), Complex::new_rectangular(24.4, 26.6));
        let r = Complex::new_polar(18.04799, 0.8285087);
        assert_eq!(r.plus(&r), Complex::new_rectangular(24.4, 26.599998));
    }
}