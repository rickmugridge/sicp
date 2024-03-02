use crate::picture_language::vector::Vector;
use speedy2d::color::Color;
use crate::picture_language::segment::{Segment};

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Line(Segment, f32, Color),
    PolyLine(Vec<(Vector, Vector)>, f32, Color),
    Circle(Vector, f32, Color), // radius
}

impl Shape {
    pub fn new_line(segment: Segment, thickness: f32, colour: Color) -> Shape {
        Shape::Line(segment, thickness, colour)
    }

    pub fn new_poly_line(s: Vec<Segment>, thickness: f32, colour: Color) -> Shape {
        Shape::PolyLine(s.iter().map(|s| (s.start, s.end)).collect(),
                        thickness, colour)
    }

    pub fn new_circle(centre: &Vector, radius: f32, colour: Color) -> Shape {
        Shape::Circle(*centre, radius, colour)
    }
}