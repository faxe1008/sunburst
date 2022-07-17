use super::canvas::Canvas;
use std::io::Write;

pub enum RendererType {
    PPM(Box<dyn Write>),
}

pub trait Renderer {
    fn show(&mut self, canvas: &Canvas);
}

pub struct PPMRenderer {
    writer: Box<dyn Write>,
}

impl PPMRenderer {
    pub fn new(writer: Box<dyn Write>) -> Self {
        PPMRenderer { writer }
    }
}

impl Renderer for PPMRenderer {
    fn show(&mut self, canvas: &Canvas) {
        writeln!(
            self.writer,
            "P6\n{} {}\n255",
            canvas.width(),
            canvas.height()
        )
        .unwrap();
        self.writer.write(canvas.raw_buffer()).unwrap();
    }
}
