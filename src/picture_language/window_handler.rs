use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use crate::picture_language::shape::Shape;
use crate::picture_language::vector::Vector;

pub fn run_picture_window(shapes: Vec<Shape>) {
    let window = Window::new_centered("Speedy2D: Animation", (800, 800)).unwrap();
    window.run_loop(PictureWindowHandler {
        shapes,
    })
}

pub struct PictureWindowHandler {
    shapes: Vec<Shape>,
}

fn to_vec2(vec: &Vector) -> Vec2 {
    Vec2 { x: vec.x, y: vec.y }
}


impl WindowHandler for PictureWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        for shape in &self.shapes {
            match shape {
                Shape::Line(ref segment, thickness, colour) =>
                    graphics.draw_line(to_vec2(&segment.start), to_vec2(&segment.end),
                                       *thickness, *colour),
                Shape::PolyLine(lines, thickness, colour) => {
                    lines.iter().for_each(|(from, to)|
                        graphics.draw_line(to_vec2(from), to_vec2(to), *thickness, *colour));
                }
                Shape::Circle(centre, radius, colour) =>
                    graphics.draw_circle(to_vec2(centre), *radius, *colour),
            }
        }
        helper.request_redraw();
    }
}

impl PictureWindowHandler {
    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
}

