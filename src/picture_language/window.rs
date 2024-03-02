use std::time::Instant;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{WindowHandler, WindowHelper, MouseButton};
use speedy2d::{Graphics2D, Window};
use crate::picture_language::segment::Segment;
use crate::picture_language::shape::Shape;
use crate::picture_language::vector::Vector;

pub fn run_window() {
    let window = Window::new_centered("Speedy2D: Animation", (800, 800)).unwrap();
    window.run_loop(MyWindowHandler {
        start_time: Instant::now(),
        shapes: vec![],
        mouse_position: Vector::new(0.0, 0.0),
        last_click: Vector::new(0.0, 0.0),
    })
}

struct MyWindowHandler {
    start_time: Instant,
    shapes: Vec<Shape>,
    mouse_position: Vector,
    last_click: Vector,
}

fn to_vec2(vec: &Vector) -> Vec2 {
    Vec2 { x: vec.x, y: vec.y }
}


impl WindowHandler for MyWindowHandler {
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

        let elapsed_secs = self.start_time.elapsed().as_secs_f32();
        let center = Vec2::new(400.0, 400.0);
        let offset = 200.0;
        let position =
            center + Vec2::new(elapsed_secs.cos() * offset, elapsed_secs.sin() * offset);

        graphics.draw_circle(position, 75.0, Color::from_rgb(0.8, 0.9, 1.0));
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vec2) {
        self.mouse_position = Vector::new(position.x, position.y)
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, _button: MouseButton) {
        self.shapes.push(
            Shape::Line(Segment::new(self.last_click, self.mouse_position),
                        2.0, Color::RED));
        self.last_click = self.mouse_position;
        helper.request_redraw();
    }
}

