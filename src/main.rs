#![feature(unchecked_math)]

mod fjor;

use std::io::stdout;

use fjor::{
    canvas::{Canvas, Color, IntPoint, IntRect},
    renderer::RendererType::PPM,
    sketch::Sketch,
};

fn main() {
    let file = Box::new(stdout());

    let mut sketch = Sketch::new(1920, 1080, PPM(file));


    sketch.on_setup = &|canvas: &mut Canvas| {
        canvas.set_color(Color::hex("#f0f").unwrap());
    };

    sketch.on_update = &|canvas: &mut Canvas, frame: usize| {
        canvas.fill_rect(&IntRect::new(
            IntPoint::new(0, 0),
            frame as isize % 50,
            frame as isize % 50,
        ));
    };

    sketch.run();
}
