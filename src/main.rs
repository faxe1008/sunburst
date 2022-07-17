#![feature(unchecked_math)]

mod fjor;

use std::sync::Mutex;

use fjor::{
    canvas::{Canvas, Color, IntPoint, IntRect},
    sketch::Sketch,
};

fn main() {
    let mut sketch = Sketch::new(1920, 1080);

    sketch.on_setup = &|canvas: &mut Canvas| {
        canvas.set_color(Color::new(255, 0, 0));
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
