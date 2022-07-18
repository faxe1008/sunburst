use super::{
    canvas::Canvas,
    renderer::{PPMRenderer, Renderer, RendererType, RendererType::PPM},
};
use std::{thread, io::stdout};
use std::time::{Duration, Instant};


pub type StateFn<State> = fn() -> State;

pub type SetupSketchFn<State> = fn(&mut Sketch<State>);

pub type UpdateSketchFn<State> = fn(&mut State);

pub type DrawSketchFn<State> = fn(&mut Canvas, &State);


pub struct Sketch<State> {
    canvas: Canvas,
    state: State,
    frame: usize,
    frame_time: isize,
    on_setup: Option<SetupSketchFn<State>>,
    on_update: Option<UpdateSketchFn<State>>,
    on_draw: Option<DrawSketchFn<State>>,
    renderer: Option<Box<dyn Renderer>>,
}

impl<State> Sketch<State> {
    pub fn new(state_fn: StateFn<State>) -> Self {
        let s = Sketch {
            canvas: Canvas::new(800, 800),
            state: state_fn(),
            frame: 1,
            frame_time: 1000 / 30,
            on_setup: None,
            on_update: None,
            on_draw: None,
            renderer: None,
        };
        s
    }

    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.canvas = Canvas::new(width, height);
        self
    }

    pub fn renderer(mut self, renderer_type: RendererType) -> Self {
        let renderer = match renderer_type {
            PPM(writer) => PPMRenderer::new(writer),
        };
        self.renderer.insert(Box::new(renderer));
        self
    }

    pub fn fps(mut self, fps: usize) -> Self {
        self.frame_time = 1000 / fps as isize;
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

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn run(mut self) {
        if let Some(setup_sketch_fn) = self.on_setup {
            setup_sketch_fn(&mut self);
        }

        let timer = Instant::now();
        loop {
            let start = timer.elapsed().as_millis();

            if let Some(ref update_sketch_fn) = self.on_update {
                update_sketch_fn(&mut self.state);
            }

            if let Some(ref draw_sketch_fn) = self.on_draw {
                draw_sketch_fn(&mut self.canvas, &self.state);
            }

            if let Some(ref mut renderer) = self.renderer {
                renderer.show(&self.canvas);
            }
            self.frame = self.frame.wrapping_add(1);

            let current_frame_time = timer.elapsed().as_millis() - start;
            let remaining_time = self.frame_time - current_frame_time as isize;
            if remaining_time > 10 {
                thread::sleep(Duration::from_millis(remaining_time as u64));
            }
        }
    }
}
