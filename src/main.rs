#![feature(unchecked_math)]

mod fjor;
use std::io::stdout;

use fjor::{
    canvas::{Canvas, Color},
    ppm,
};

fn main() {
    let mut can = Canvas::new(200, 200);

    for r in (1..100).step_by(1) {
        can.draw_circle(0, 0, r, &Color::new(255, 0, 0));
    }
    can.draw_line(0, 0, 200, 200, &Color::new(255, 0, 255));
    let mut stdout = stdout();
    ppm::PPMExporter::write(&can, &mut stdout);
}
