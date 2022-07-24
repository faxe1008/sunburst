use crate::canvas::Color;

pub struct PixelBuffer {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        PixelBuffer {
            buffer: vec![255; 3 * width * height],
            width,
            height,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn clear(&mut self, color: &Color) {
        for index in (0..self.buffer.len()).step_by(3) {
            self.buffer[index] = color.red;
            self.buffer[index + 1] = color.green;
            self.buffer[index + 2] = color.blue;
        }
    }

    fn cartesian_to_index(&self, width: usize, height: usize) -> usize {
        3 * (height * self.width + width)
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: &Color) {
        if x < 0 || y < 0 {
            return;
        }
        let index = self.cartesian_to_index(x as usize, y as usize);
        if index >= self.buffer.len() {
            return;
        }
        self.buffer[index] = color.red;
        self.buffer[index + 1] = color.green;
        self.buffer[index + 2] = color.blue;
    }

    pub fn as_raw_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}
