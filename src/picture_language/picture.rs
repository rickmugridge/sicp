use crate::picture_language::frame::Frame;
use crate::picture_language::segment::Segment;
use crate::picture_language::vector::Vector;
use crate::picture_language::window_handler;
use speedy2d::color::Color;
use crate::picture_language::painter::{Painter};
use crate::picture_language::composite_painter::{CompositePainter};

pub fn run_picture() {
    let frame = Frame::new(Vector::zero(),
                           Vector::new(100.0, 20.0),
                           Vector::new(20.0, 200.0));
    let painter1 = Painter::new(frame);
    let painter2 = painter1.right();
    let painter3 = painter1.below();
    let mut painter = CompositePainter::new(vec![painter1, painter2, painter3]);
    painter.render_lines(&outer_bounds(), 1.0, Color::RED);
    painter.render_lines(&cross(), 1.0, Color::GREEN);
    painter.render_lines(&diamond(), 1.0, Color::BLACK);
    painter.render_circle(&Vector::new(0.5, 0.5), 0.3, Color::RED);
    window_handler::run_picture_window(painter.paint());
}

fn outer_bounds() -> Vec<Segment> {
    let top_right = Vector::new(1.0, 0.0);
    let bottom_left = Vector::new(0.0, 1.0);
    Segment::open_path(&vec![Vector::zero(), top_right, Vector::one(), bottom_left, Vector::zero()])
}

fn diamond() -> Vec<Segment> {
    Segment::open_path(&vec![
        Vector::new(0.0, 0.5),
        Vector::new(0.5, 0.0),
        Vector::new(1.0, 0.5),
        Vector::new(0.5, 1.0),
        Vector::new(0.0, 0.5),
    ])
}

fn cross() -> Vec<Segment> {
    let top_right = Vector::new(1.0, 0.0);
    let bottom_left = Vector::new(0.0, 1.0);
    vec![
        Segment::new(top_right, bottom_left),
        Segment::new(Vector::zero(), Vector::new(1.0, 1.0)),
    ]
}