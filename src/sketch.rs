use crate::renderer::SDLRenderer;
use crate::sketch::RendererType::PPM;

use super::{
    canvas::Canvas,
    renderer::{PPMRenderer, Renderer, RendererType, RendererType::SDL2},
};
use std::time::{Duration, Instant};

pub type StateFn<State> = fn() -> State;

pub type SetupSketchFn<State> = fn(&mut Sketch<State>);

pub type UpdateSketchFn<State> = fn(&mut State, &SketchMetrics);

pub type DrawSketchFn<State> = fn(&mut Canvas, &State, &SketchMetrics);

#[derive(Default)]
pub struct SketchMetrics {
    pub frame_count: usize,
    pub delta_time: Duration,
    pub frames_per_second: usize,
}

pub struct Sketch<State> {
    canvas: Canvas,
    state: State,
    metrics: SketchMetrics,
    on_setup: Option<SetupSketchFn<State>>,
    on_update: Option<UpdateSketchFn<State>>,
    on_draw: Option<DrawSketchFn<State>>,
    renderer: Option<Box<dyn Renderer>>,
}

impl<State> Sketch<State> {
    pub fn new(state_fn: StateFn<State>, width: usize, height: usize) -> Self {
        let s = Sketch {
            canvas: Canvas::new(width, height),
            state: state_fn(),
            metrics: SketchMetrics::default(),
            on_setup: None,
            on_update: None,
            on_draw: None,
            renderer: None,
        };
        s
    }

    pub fn renderer(mut self, renderer_type: RendererType) -> Self {
        let renderer: Box<dyn Renderer> = match renderer_type {
            PPM(file) => Box::new(PPMRenderer::new(file)),
            SDL2(title) => Box::new(SDLRenderer::new(
                &title,
                self.canvas.width(),
                self.canvas.height(),
            )),
        };
        self.renderer = Some(renderer);
        self
    }

    pub fn setup(mut self, setup_fn: SetupSketchFn<State>) -> Self {
        self.on_setup = Some(setup_fn);
        self
    }

    pub fn update(mut self, update_fn: UpdateSketchFn<State>) -> Self {
        self.on_update = Some(update_fn);
        self
    }

    pub fn draw(mut self, draw_fn: DrawSketchFn<State>) -> Self {
        self.on_draw = Some(draw_fn);
        self
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn frame_count(&self) -> usize {
        self.metrics.frame_count
    }

    pub fn delta_time(&self) -> Duration {
        self.metrics.delta_time
    }
    pub fn run(mut self) {
        if let Some(setup_sketch_fn) = self.on_setup {
            setup_sketch_fn(&mut self);
        }

        let timer = Instant::now();
        loop {
            let start = timer.elapsed();

            if let Some(ref update_sketch_fn) = self.on_update {
                update_sketch_fn(&mut self.state, &self.metrics);
            }

            if let Some(ref draw_sketch_fn) = self.on_draw {
                draw_sketch_fn(&mut self.canvas, &self.state, &self.metrics);
            }

            if let Some(ref mut renderer) = self.renderer {
                if !renderer.update(&self.canvas) {
                    break;
                }
            }
            self.metrics.frame_count = self.metrics.frame_count.wrapping_add(1);
            self.metrics.delta_time = timer.elapsed() - start;
            self.metrics.frames_per_second =
                (1000 as u128 / self.metrics.delta_time.as_millis()) as usize;
        }
    }
}
