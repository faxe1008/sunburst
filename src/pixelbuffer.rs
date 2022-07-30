use crate::canvas::Color;

pub struct PixelBuffer {
    buffer: Vec<Color>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        PixelBuffer {
            buffer: vec![Color::rgb(255, 255, 255); width * height],
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
        for index in 0..self.buffer.len() {
            self.buffer[index] = *color;
        }
    }

    fn cartesian_to_index(&self, width: usize, height: usize) -> usize {
        height * self.width + width
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: &Color) {
        if x < 0 || y < 0 {
            return;
        }
        let index = self.cartesian_to_index(x as usize, y as usize);
        if index >= self.buffer.len() {
            return;
        }
        self.buffer[index] = *color;
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&Color> {
        let index = self.cartesian_to_index(x as usize, y as usize);
        if index >= self.buffer.len() {
            None
        } else {
            Some(&self.buffer[index])
        }
    }

    pub fn as_raw_buffer(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.buffer.as_ptr() as *const u8,
                self.buffer.len() * std::mem::size_of::<Color>(),
            )
        }
    }

    pub fn as_raw_buffer_mut(&self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.buffer.as_ptr() as *mut u8,
                self.buffer.len() * std::mem::size_of::<Color>(),
            )
        }
    }
}
