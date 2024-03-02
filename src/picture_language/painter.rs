use crate::picture_language::frame::Frame;
use crate::picture_language::segment::Segment;
use crate::picture_language::shape::Shape;
use speedy2d::color::Color;
use crate::picture_language::composite_painter::CompositePainter;
use crate::picture_language::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct Painter {
    pub frame: Frame,
    shapes: Vec<Shape>,
}

impl Painter {
    pub fn new(frame: Frame) -> Self {
        Painter { frame, shapes: vec![] }
    }

    pub fn below(&self) -> Self {
        Painter { frame: self.frame.below(), shapes: vec![] }
    }

    pub fn right(&self) -> Self {
        Painter { frame: self.frame.right(), shapes: vec![] }
    }

    pub fn render_lines(&mut self, unit_segments: &Vec<Segment>, thickness: f32, colour: Color) {
        let segments: Vec<Segment> = unit_segments.iter()
            .map(|segment| self.frame.map_segment(*segment)).collect();
        segments.iter()
            .for_each(|s| self.shapes.push(Shape::new_line(*s, thickness, colour)));
    }

    pub fn render_circle(&mut self, centre: &Vector, radius: f32, colour: Color) {
        let centre = self.frame.map(*centre);
        self.shapes.push(Shape::new_circle(&centre, radius, colour));
    }

    pub fn paint(&mut self) -> Vec<Shape> {
        let result = self.shapes.clone();
        self.shapes = vec![];
        result
    }

    pub fn tessellate_to_right(&self, count: usize) -> CompositePainter {
        self.tessellate(Vector::new(self.frame.x_max(), 0.0), count)
    }

    pub fn tessellate_to_down(&self, count: usize) -> CompositePainter {
        self.tessellate(Vector::new(0.0, self.frame.y_max()), count)
    }

    pub fn tessellate(&self, _translate_initial: Vector, count: usize) -> CompositePainter {
        let mut translate = Vector::zero();
        let painters = (0..count)
            .map(|_| {
                translate = translate + translate;
                self.transform(translate)
            })
            .collect();
        CompositePainter::new(painters)
    }

    fn transform(&self, v: Vector) -> Self {
        Painter::new(self.frame.transform(v))
    }
}

