#![feature(unchecked_math)]

extern crate fjor;

use std::f32::consts::PI;

use fjor::{
    canvas::{Canvas, Color, IntPoint},
    path::Path,
    renderer::RendererType::SDL2,
    sketch::{Sketch, SketchMetrics},
};

const SPINNER_RADIUS: usize = 25;
const SPINNER_MARGIN: usize = 5;
const SPINNER_COUNT: usize = 15;
const INDICATOR_SIZE: isize = 2;

enum SpinnerType {
    Column,
    Row,
}

struct LissaJousSpinner {
    center: IntPoint,
    radius: usize,
    angle: f32,
    speed_multiplier: f32,
    indicator_pos: IntPoint,
    ty: SpinnerType,
}

#[derive(Clone)]
struct LissaJousFigure {
    points: Path,
}

impl LissaJousFigure {
    fn new() -> Self {
        LissaJousFigure {
            points: Path::new(),
        }
    }

    fn show(&self, canvas: &mut Canvas) {
        canvas.draw_path(&self.points);
    }

    fn add_point(&mut self, col_spinner: &LissaJousSpinner, row_spinner: &LissaJousSpinner) {
        let next_point = IntPoint::new(col_spinner.indicator_pos.x, row_spinner.indicator_pos.y);
        if self.points.segment_count() == 0 {
            self.points.move_to(&next_point);
        } else {
            self.points.line_to(&next_point);
        }
    }

    fn clear(&mut self) {
        self.points.clear();
    }
}

impl LissaJousSpinner {
    fn new(center: IntPoint, speed_multiplier: f32, ty: SpinnerType) -> Self {
        LissaJousSpinner {
            indicator_pos: LissaJousSpinner::indicator_pos(&center, SPINNER_RADIUS, 0.0),
            center,
            radius: SPINNER_RADIUS,
            angle: 0.0,
            speed_multiplier,
            ty,
        }
    }

    fn indicator_pos(center: &IntPoint, radius: usize, angle: f32) -> IntPoint {
        IntPoint::new(
            center.x + (angle.cos() * radius as f32) as isize,
            center.y + (angle.sin() * radius as f32) as isize,
        )
    }

    fn show(&self, canvas: &mut Canvas) {
        canvas.stroke(Color::rgb(0, 0, 0));
        canvas.no_fill();
        canvas.draw_ellipse(&self.center, self.radius, self.radius);

        canvas.stroke(Color::rgb(200, 200, 200));
        match self.ty {
            SpinnerType::Column => {
                canvas.draw_line(
                    &IntPoint::new(self.indicator_pos.x, 0),
                    &IntPoint::new(self.indicator_pos.x, canvas.height() as isize),
                );
            }
            SpinnerType::Row => {
                canvas.draw_line(
                    &IntPoint::new(0, self.indicator_pos.y),
                    &IntPoint::new(canvas.width() as isize, self.indicator_pos.y),
                );
            }
        }

        canvas.stroke(Color::rgb(0, 0, 255));
        canvas.fill(Color::rgb(0, 0, 255));
        let indicator_pos = IntPoint::new(
            self.indicator_pos.x - INDICATOR_SIZE / 2,
            self.indicator_pos.y - INDICATOR_SIZE / 2,
        );
        canvas.draw_square(&indicator_pos, INDICATOR_SIZE);
    }

    fn update(&mut self) {
        self.angle += 0.005 * self.speed_multiplier;
        self.indicator_pos =
            LissaJousSpinner::indicator_pos(&self.center, SPINNER_RADIUS, self.angle);
    }

    fn has_finished(&self) -> bool {
        self.angle >= 2.0 * PI
    }

    fn reset(&mut self) {
        self.angle = 0.0;
    }
}

struct SketchState {
    column_spinners: Vec<LissaJousSpinner>,
    row_spinners: Vec<LissaJousSpinner>,
    figures: Vec<LissaJousFigure>,
}

fn state_create() -> SketchState {
    let starting_x = SPINNER_MARGIN + SPINNER_RADIUS;
    let starting_y = SPINNER_MARGIN + SPINNER_RADIUS;

    let mut column_spinners = Vec::new();
    let mut row_spinners = Vec::new();

    for i in 1..=SPINNER_COUNT {
        let spinner_x = starting_x + (SPINNER_RADIUS * 2 + SPINNER_MARGIN) * i;
        column_spinners.push(LissaJousSpinner::new(
            IntPoint::new(spinner_x as isize, starting_y as isize),
            i as f32,
            SpinnerType::Column,
        ));
    }

    for i in 1..=SPINNER_COUNT {
        let spinner_y = starting_y + (SPINNER_RADIUS * 2 + SPINNER_MARGIN) * i;
        row_spinners.push(LissaJousSpinner::new(
            IntPoint::new(starting_x as isize, spinner_y as isize),
            i as f32,
            SpinnerType::Row,
        ));
    }

    SketchState {
        column_spinners,
        row_spinners,
        figures: vec![LissaJousFigure::new(); SPINNER_COUNT * SPINNER_COUNT],
    }
}

fn setup(_: &mut Sketch<SketchState>) {}

fn update(state: &mut SketchState, _: &SketchMetrics) {
    for column_spinner in state.column_spinners.iter_mut() {
        column_spinner.update();
    }

    for row_spinner in state.row_spinners.iter_mut() {
        row_spinner.update();
    }

    let mut all_finished = true;
    for fig_index in 0..state.figures.len() {
        let row_spinner = &state.row_spinners[fig_index / SPINNER_COUNT];
        let col_spinner = &state.column_spinners[fig_index % SPINNER_COUNT];

        all_finished = all_finished & row_spinner.has_finished() && col_spinner.has_finished();
        state.figures[fig_index].add_point(col_spinner, row_spinner);
    }

    if all_finished {
        for column_spinner in state.column_spinners.iter_mut() {
            column_spinner.reset();
        }
        for row_spinner in state.row_spinners.iter_mut() {
            row_spinner.reset();
        }
        for figure in state.figures.iter_mut() {
            figure.clear();
        }
    }
}

fn draw(canvas: &mut Canvas, state: &SketchState, _: &SketchMetrics) {
    canvas.clear();

    for column_spinner in &state.column_spinners {
        column_spinner.show(canvas);
    }

    for row_spinner in &state.row_spinners {
        row_spinner.show(canvas);
    }

    for figure in &state.figures {
        figure.show(canvas);
    }
}

fn main() {
    let sketch = Sketch::new(state_create, 900, 900)
        .renderer(SDL2("Lissajous"))
        .setup(setup)
        .update(update)
        .draw(draw);

    sketch.run();
}
