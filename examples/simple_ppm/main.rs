#![feature(unchecked_math)]

extern crate fjor;

use std::io::stdout;

use fjor::{
    canvas::{Canvas, Color, FontWeight, IntPoint},
    renderer::RendererType::PPM,
    sketch::Sketch,
};

struct SketchState {
    endpoint: IntPoint,
    lerp_amount: f32,
}

fn state_create() -> SketchState {
    SketchState {
        endpoint: IntPoint::new(155, 1000),
        lerp_amount: 0.0,
    }
}

fn setup(sketch: &mut Sketch<SketchState>) {
    sketch.canvas_mut().fill(Color::rgb(255, 0, 0));
    sketch.canvas_mut().clear();

    sketch.canvas_mut().font_size(55);
    sketch.canvas_mut().font_weight(FontWeight::Bold);
}

fn update(state: &mut SketchState) {
    state.endpoint.y = (state.endpoint.y + 10) % 1000;

    state.lerp_amount += 0.05;
    if state.lerp_amount > 1.0 {
        state.lerp_amount = 0.0;
    }
}

fn draw(canvas: &mut Canvas, state: &SketchState) {
    canvas.clear();

    let c = Color::rgb(255, 0, 0).lerp_to(&Color::rgb(0, 0, 200), state.lerp_amount);
    canvas.fill(c);

    //canvas.draw_square(&state.endpoint, 100);

    canvas.draw_text(
        &IntPoint::new(100, 100),
        &format!("Hello at {}", state.endpoint.y),
    );
}

fn main() {
    let file = Box::new(stdout());

    let sketch = Sketch::new(state_create)
        .size(1000, 1000)
        .fps(30)
        .renderer(PPM(file))
        .setup(setup)
        .update(update)
        .draw(draw);

    sketch.run();
}
