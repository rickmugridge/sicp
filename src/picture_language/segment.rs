use crate::picture_language::vector::Vector;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Segment {
    pub start: Vector,
    pub end: Vector,
}

impl Segment {
    pub fn new(start: Vector, end: Vector) -> Self {
        Segment { start, end }
    }

    pub fn open_path(v: &[Vector]) -> Vec<Segment> {
        assert!(v.len() > 0);
        let mut previous = &v[0];
        v.iter().skip(1).map(|v| {
            let segment = Segment::new(*previous, *v);
            previous = v;
            segment
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector::new(1.0, 2.0);
        let v2 = Vector::new(3.0, 4.0);
        let v3 = Vector::new(4.0, 6.0);
        let path = vec![v1, v2, v3, v1];
        assert_eq!(Segment::open_path(&path), vec![
            Segment::new(v1, v2),
            Segment::new(v2, v3),
            Segment::new(v3, v1),
        ]);
    }
}