#![feature(unchecked_math)]

mod fjor;

use rand::{rngs::ThreadRng, Rng};
use std::io::stdout;

use fjor::{
    canvas::{Canvas, Color, IntPoint, IntRect},
    renderer::RendererType::PPM,
    sketch::Sketch,
};

struct SketchState {
    rnd: ThreadRng,
    rects: Vec<IntRect>,
}

fn state_create() -> SketchState {
    SketchState {
        rnd: rand::thread_rng(),
        rects: Vec::new(),
    }
}

fn setup(sketch: &mut Sketch<SketchState>) {
    let state = sketch.state_mut();

    state.rects.push(IntRect::new(
        IntPoint::new(state.rnd.gen_range(0..100), state.rnd.gen_range(0..100)),
        100,
        100,
    ));
}

fn update(state: &mut SketchState) {}

fn draw(canvas: &mut Canvas, state: &SketchState) {
    canvas.clear();
    canvas.no_fill();
    for rect in &state.rects {
        canvas.draw_rect(rect);
    }
}

fn main() {
    let file = Box::new(stdout());

    let mut sketch = Sketch::new(state_create)
        .size(1000, 1000)
        .fps(10)
        .renderer(PPM(file))
        .setup(setup)
        .update(update)
        .draw(draw);

    sketch.run();
}
