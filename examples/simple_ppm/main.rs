#![feature(unchecked_math)]

extern crate fjor;

use std::{f32::consts::PI, io::stdout};

use fjor::{
    canvas::{Canvas, Color, FontWeight, IntPoint},
    path::Path,
    renderer::RendererType::PPM,
    sketch::Sketch,
};

struct SketchState {
    distance: f32,
    path: Path,
    origin: IntPoint,
}

fn state_create() -> SketchState {
    SketchState {
        distance: 0.0,
        path: Path::new(),
        origin: IntPoint::new(200, 200),
    }
}

fn setup(sketch: &mut Sketch<SketchState>) {
    sketch.canvas_mut().fill(Color::rgb(0, 1, 111));
    sketch.canvas_mut().font_weight(FontWeight::Bold);
    sketch.canvas_mut().font_size(33);
}

fn update(state: &mut SketchState) {
    state.distance += 0.3;

    if state.distance > PI * 2.0 {
        state.path.clear();
        state.distance = 0.0;
    }

    let y = state.origin.y + (state.distance.sin() * RAD) as isize;
    let x = state.origin.x + (state.distance.cos() * RAD) as isize;
    state.path.line_to(&IntPoint::new(x, y));
}

const RAD: f32 = 100.0;

fn draw(canvas: &mut Canvas, state: &SketchState) {
    canvas.clear();

    canvas.draw_path(&state.path);
}

fn main() {
    let file = Box::new(stdout());

    let sketch = Sketch::new(state_create)
        .size(1000, 1000)
        .fps(60)
        .renderer(PPM(file))
        .setup(setup)
        .update(update)
        .draw(draw);

    sketch.run();
}
