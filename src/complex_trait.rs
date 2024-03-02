trait Complex {
    fn real(&self) -> f32;
    fn imaginary(&self) -> f32;
    fn magnitude(&self) -> f32;
    fn angle(&self) -> f32;
}

#[derive(Debug, PartialEq, Clone)]
struct RectangularComplex {
    real: f32,
    imaginary: f32,
}

impl Complex for RectangularComplex {
    fn real(&self) -> f32 {
        self.real
    }

    fn imaginary(&self) -> f32 {
        self.imaginary
    }

    fn magnitude(&self) -> f32 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    fn angle(&self) -> f32 {
        self.imaginary.atan2(self.real)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PolarComplex {
    magnitude: f32,
    angle: f32,
}

impl Complex for PolarComplex {
    fn real(&self) -> f32 {
        self.magnitude * self.angle.cos()
    }

    fn imaginary(&self) -> f32 {
        self.magnitude * self.angle.sin()
    }

    fn magnitude(&self) -> f32 {
        self.magnitude
    }

    fn angle(&self) -> f32 {
        self.angle
    }
}

impl dyn Complex {
    pub fn add(&self, other: &Self) -> Box<dyn Complex> {
        Box::new(RectangularComplex {
            real: self.real() + other.real(),
            imaginary: self.imaginary() + other.imaginary(),
        })
    }
}

impl PartialEq for dyn Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real() == other.real() && self.imaginary() == other.imaginary()
    }
}

fn add(a: &impl Complex, b: &impl Complex) -> Box<dyn Complex> {
    Box::new(RectangularComplex {
        real: a.real() + b.real(),
        imaginary: a.imaginary() + b.imaginary(),
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangular() {
        let r = RectangularComplex { real: 12.2, imaginary: 13.3 };
        assert_eq!(r.real(), 12.2);
        assert_eq!(r.imaginary(), 13.3);
        assert_eq!(r.magnitude(), 18.04799);
        assert_eq!(r.angle(), 0.8285087);
    }

    #[test]
    fn polar() {
        let r = PolarComplex { magnitude: 18.04799, angle: 0.8285087 };
        assert_eq!(r.real(), 12.2);
        assert_eq!(r.imaginary(), 13.299999);
        assert_eq!(r.magnitude(), 18.04799);
        assert_eq!(r.angle(), 0.8285087);
    }

    #[test]
    fn plus() {
        let complex = RectangularComplex { real: 12.2, imaginary: 13.3 };
        let p = add(&complex, &complex);
        assert_eq!(p.real(), 24.4);
        assert_eq!(p.imaginary(), 26.6);
    }

    #[test]
    fn plus2() {
        let rect = RectangularComplex { real: 12.2, imaginary: 13.3 };
        let polar = PolarComplex { magnitude: 18.04799, angle: 0.8285087 };
        let p = add(&rect, &polar);
        assert_eq!(p.real(), 24.4);
        assert_eq!(p.imaginary(), 26.599998);
    }

    // #[test]
    // fn plus_dyn_error() {
    //     let complex = RectangularComplex { real: 12.2, imaginary: 13.3 };
    //     let p = add(&complex, &complex);
    //     // todo FOLLOWING FAILS TO COMPILE: expected trait object `dyn Complex`, found struct `RectangularComplex
    //     assert_eq!(*p, &RectangularComplex { real: 24.4, imaginary: 26.6 });
    // }

    // #[test]
    // fn plus_missing_trait_fn() {
    //     let complex = RectangularComplex { real: 12.2, imaginary: 13.3 };
    //     // todo FOLLOWING FAILS TO COMPILE: no method named `add` found for struct `RectangularComplex` in the current scope
    //     let p = complex.add(&complex);
    //     assert_eq!(p.real(), 24.4);
    //     assert_eq!(p.imaginary(), 26.6);
    // }
}