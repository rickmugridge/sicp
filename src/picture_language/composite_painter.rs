use crate::picture_language::painter::Painter;
use crate::picture_language::segment::Segment;
use crate::picture_language::shape::Shape;
use crate::picture_language::vector::Vector;
use speedy2d::color::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct CompositePainter {
    painters: Vec<Painter>,
}

impl CompositePainter {
    pub fn new(painters: Vec<Painter>) -> Self {
        Self { painters }
    }

    pub fn render_lines(&mut self, unit_segments: &Vec<Segment>, thickness: f32, colour: Color) {
        self.painters.iter_mut()
            .for_each(|p| p.render_lines(unit_segments, thickness, colour));
    }

    pub fn render_circle(&mut self, centre: &Vector, radius: f32, colour: Color) {
        self.painters.iter_mut()
            .for_each(|p| p.render_circle(centre, radius, colour));
    }

    pub fn paint(&mut self) -> Vec<Shape> {
        let mut result: Vec<Shape> = vec![];
        self.painters.iter_mut()
            .for_each(|p| result.extend_from_slice(&p.paint()));
        result
    }

    pub fn to_right(&self) -> Self {
        let x_max: f32 = self.painters.iter()
            .map(|p| p.frame.x_max())
            .fold(0.0, |x1, x2| x1.max(x2));
        self.transform(Vector::new(x_max, 0.0))
    }

    pub fn to_below(&self) -> Self {
        let y_max: f32 = self.painters.iter()
            .map(|p| p.frame.y_max())
            .fold(0.0, |y1, y2| y1.max(y2));
        self.transform(Vector::new(0.0, y_max))
    }

    fn transform(&self, v: Vector) -> Self {
        CompositePainter::new(self.painters
            .iter()
            .map(|p|
                Painter::new(p.frame.transform(v)))
            .collect())
    }
}
