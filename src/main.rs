#![feature(unchecked_math)]

mod fjor;
use std::io::stdout;

use fjor::{
    canvas::{Canvas, Color, IntPoint, IntRect},
    ppm,
};

fn main() {
    let mut can = Canvas::new(500, 500);

    can.fill_rect(
        &IntRect::new(IntPoint::new(0, 0), 500, 500),
        &Color::new(255, 0, 9),
    );
    let mut stdout = stdout();
    ppm::PPMExporter::write(&can, &mut stdout);
}
