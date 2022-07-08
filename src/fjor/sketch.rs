use std::io::stdout;

use super::{canvas::Canvas, ppm};

pub struct Sketch<'a> {
    canvas: Canvas,
    frame: usize,
    pub on_setup: &'a dyn Fn(&mut Canvas),
    pub on_update: &'a dyn Fn(&mut Canvas, usize),
}

impl<'a> Sketch<'a> {
    pub fn new(width: usize, height: usize) -> Self {
        Sketch {
            canvas: Canvas::new(width, height),
            frame: 1,
            on_setup: &|_: &mut Canvas| {},
            on_update: &|_: &mut Canvas, _: usize| {},
        }
    }

    pub fn run(&mut self) {
        (self.on_setup)(&mut self.canvas);
        let mut stdout = stdout();
        loop {
            (self.on_update)(&mut self.canvas, self.frame);
            ppm::PPMExporter::write(&self.canvas, &mut stdout);
            self.frame = self.frame.wrapping_add(1);
            self.canvas.clear();
        }
    }
}
