use super::{
    canvas::Canvas,
    renderer::{PPMRenderer, Renderer, RendererType, RendererType::PPM},
};
use std::thread;
use std::time::{Duration, Instant};

pub struct Sketch<'a> {
    canvas: Canvas,
    frame: usize,
    frame_time: isize,
    pub on_setup: &'a dyn Fn(&mut Canvas),
    pub on_update: &'a dyn Fn(&mut Canvas, usize),
    renderer: Box<dyn Renderer>,
}

impl<'a> Sketch<'a> {
    pub fn new(width: usize, height: usize, renderer_type: RendererType) -> Self {
        let renderer = match renderer_type {
            PPM(writer) => PPMRenderer::new(writer),
        };

        Sketch {
            canvas: Canvas::new(width, height),
            frame: 1,
            frame_time: 1000 / 30,
            on_setup: &|_: &mut Canvas| {},
            on_update: &|_: &mut Canvas, _: usize| {},
            renderer: Box::new(renderer),
        }
    }

    pub fn set_fps(&mut self, fps: usize) {
        self.frame_time = 1000 / fps as isize;
    }

    pub fn run(&mut self) {
        (self.on_setup)(&mut self.canvas);
        let timer = Instant::now();
        loop {
            let start = timer.elapsed().as_millis();

            (self.on_update)(&mut self.canvas, self.frame);
            self.renderer.show(&self.canvas);
            self.frame = self.frame.wrapping_add(1);
            self.canvas.clear();

            let current_frame_time = timer.elapsed().as_millis() - start;
            let remaining_time = self.frame_time - current_frame_time as isize;
            if remaining_time > 10 {
                thread::sleep(Duration::from_millis(remaining_time as u64));
            }
        }
    }
}
