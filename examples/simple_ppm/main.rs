#![feature(unchecked_math)]

extern crate fjor;

use rand::{rngs::ThreadRng, Rng};
use std::io::stdout;

use fjor::{
    canvas::{Canvas, Color, IntPoint, IntRect},
    renderer::RendererType::PPM,
    sketch::Sketch,
};

struct SketchState {
    rnd: ThreadRng,
    endpoint: IntPoint,
}

fn state_create() -> SketchState {
    SketchState {
        rnd: rand::thread_rng(),
        endpoint: IntPoint::new(155, 1000),
    }
}

fn setup(sketch: &mut Sketch<SketchState>) {
    sketch.canvas_mut().fill(Color::rgb(255, 0, 0));
    sketch.canvas_mut().clear();
}

fn clamp<T>(v: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

fn update(state: &mut SketchState) {
    state.endpoint.y = (state.endpoint.y + 10) % 1000;
}

fn draw(canvas: &mut Canvas, state: &SketchState) {
    canvas.clear();

    canvas.draw_square(&state.endpoint, 10);
}

fn main() {
    let file = Box::new(stdout());

    let mut sketch = Sketch::new(state_create)
        .size(1000, 1000)
        .fps(30)
        .renderer(PPM(file))
        .setup(setup)
        .update(update)
        .draw(draw);

    sketch.run();
}
