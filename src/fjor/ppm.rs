use super::canvas::Canvas;
use std::io::Write;

pub struct PPMExporter {}

impl PPMExporter {
    pub fn write(canvas: &Canvas, file: &mut dyn Write) {
        writeln!(file, "P6\n{} {}\n255", canvas.width(), canvas.height()).unwrap();
        file.write(canvas.raw_buffer()).unwrap();
    }
}
