use crate::picture_language::segment::Segment;
use crate::picture_language::vector::Vector;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Frame {
    origin: Vector,
    edge1: Vector,
    edge2: Vector,
}

impl Frame {
    pub fn new(origin: Vector, edge1: Vector, edge2: Vector) -> Self {
        Frame { origin, edge1, edge2 }
    }

    // v is within unit square
    pub fn map(&self, v: Vector) -> Vector {
        self.origin + self.edge1.scale(v.x) + self.edge2.scale(v.y)
    }

    // s is within unit square
    pub fn map_segment(&self, s: Segment) -> Segment {
        Segment::new(self.map(s.start), self.map(s.end))
    }

    pub fn below(&self) -> Self {
        Frame::new(self.origin + Vector::new(self.edge2.x, self.edge2.y), self.edge1, self.edge2)
    }

    pub fn right(&self) -> Self {
        Frame::new(self.origin + Vector::new(self.edge1.x, self.edge1.y), self.edge1, self.edge2)
    }

    pub fn x_max(&self) -> f32 {
        self.origin.x.max(self.origin.x + self.edge1.x).max(self.origin.x + self.edge2.x)
    }

    pub fn y_max(&self) -> f32 {
        self.origin.y.max(self.origin.y + self.edge1.y).max(self.origin.y + self.edge2.y)
    }

    pub fn transform(&self, v: Vector) -> Self {
        Frame::new(self.origin + v, self.edge1, self.edge2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_from_origin() {
        let f = Frame::new(Vector::zero(),
                           Vector::new(100.0, 0.0),
                           Vector::new(0.0, 200.0));
        assert_eq!(f.map(Vector::zero()), Vector::zero());
        assert_eq!(f.map(Vector::new(1.0, 1.0)), Vector::new(100.0, 200.0));
        assert_eq!(f.map(Vector::new(0.5, 0.5)), Vector::new(50.0, 100.0));
    }

    #[test]
    fn map_off_origin() {
        let f = Frame::new(Vector::new(10.0, 20.0),
                           Vector::new(100.0, 0.0),
                           Vector::new(0.0, 200.0));
        assert_eq!(f.map(Vector::zero()), Vector::new(10.0, 20.0));
        assert_eq!(f.map(Vector::new(1.0, 1.0)), Vector::new(110.0, 220.0));
        assert_eq!(f.map(Vector::new(0.5, 0.5)), Vector::new(60.0, 120.0));
    }
}