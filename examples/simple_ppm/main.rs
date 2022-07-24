#![feature(unchecked_math)]

extern crate fjor;

use std::{f32::consts::PI, io::stdout};

use fjor::{
    canvas::{Canvas, Color, FontWeight, IntPoint},
    path::Path,
    renderer::RendererType::PPM,
    sketch::{Sketch, SketchMetrics},
};

struct SketchState {
    lerp_amount: f32,
}

fn state_create() -> SketchState {
    SketchState { lerp_amount: 0.0 }
}

fn setup(sketch: &mut Sketch<SketchState>) {
    sketch.canvas_mut().fill(Color::rgb(0, 1, 111));
    sketch.canvas_mut().font_weight(FontWeight::Bold);
    sketch.canvas_mut().font_size(33);
}

fn update(state: &mut SketchState, metrics: &SketchMetrics) {
    state.lerp_amount += 0.05;
    if state.lerp_amount >= 1.0 {
        state.lerp_amount = 0.0;
    }
}

fn draw(canvas: &mut Canvas, state: &SketchState, metrics: &SketchMetrics) {
    canvas.clear();

    let c = Color::lerp_to(
        &Color::rgb(255, 0, 0),
        &Color::rgb(0, 0, 255),
        state.lerp_amount,
    );
    canvas.fill(c);
    canvas.draw_square(&IntPoint::new(50, 50), 100);

    canvas.draw_text(
        &IntPoint::new(400, 400),
        &format!(
            "dt: {}\nfps: {}",
            metrics.delta_time.as_millis(),
            metrics.frames_per_second
        ),
    );
}

fn main() {
    let file = Box::new(stdout());

    let sketch = Sketch::new(state_create)
        .size(1000, 1000)
        .renderer(PPM(file))
        .setup(setup)
        .update(update)
        .draw(draw);

    sketch.run();
}
