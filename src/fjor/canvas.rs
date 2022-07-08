use std::mem;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

pub struct IntPoint {
    pub x: isize,
    pub y: isize,
}

impl IntPoint {
    pub fn new(x: isize, y: isize) -> Self {
        IntPoint { x, y }
    }
}

pub struct IntRect {
    pub location: IntPoint,
    pub width: isize,
    pub height: isize,
}

impl IntRect {
    pub fn new(mut location: IntPoint, mut width: isize, mut height: isize) -> Self {
        if width < 0 {
            location.x += width;
            width *= -1;
        }
        if height < 0 {
            location.y += height;
            height *= -1;
        }
        IntRect {
            location,
            width,
            height,
        }
    }

    pub fn x(&self) -> isize {
        self.location.x
    }

    pub fn y(&self) -> isize {
        self.location.y
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            buffer: vec![0; width * height * 3],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn raw_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    fn cartesian_to_index(&self, width: usize, height: usize) -> usize {
        3 * (height * self.width + width)
    }

    pub fn clear(&mut self) {
        unsafe {
            libc::memset(
                self.buffer.as_mut_ptr() as _,
                0,
                self.buffer.len() * mem::size_of::<u8>(),
            );
        }
    }

    fn set_pixel_internal(&mut self, width: isize, height: isize, color: &Color) {
        if width < 0 || height < 0 {
            return;
        }
        let index = self.cartesian_to_index(width as usize, height as usize);
        if index >= self.buffer.len() {
            return;
        }
        self.buffer[index] = color.red;
        self.buffer[index + 1] = color.green;
        self.buffer[index + 2] = color.blue;
    }

    pub fn set_pixel(&mut self, point: &IntPoint, color: &Color) {
        self.set_pixel_internal(point.x, point.y, color);
    }

    /// https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
    pub fn draw_circle(&mut self, center: &IntPoint, r: usize, color: &Color) {
        let mut draw_subsequence_points = |x: isize, y: isize| {
            self.set_pixel_internal(center.x + x, center.y + y, color);
            self.set_pixel_internal(center.x - x, center.y + y, color);
            self.set_pixel_internal(center.x + x, center.y - y, color);
            self.set_pixel_internal(center.x - x, center.y - y, color);
            self.set_pixel_internal(center.x + y, center.y + x, color);
            self.set_pixel_internal(center.x - y, center.y + x, color);
            self.set_pixel_internal(center.x + y, center.y - x, color);
            self.set_pixel_internal(center.x - y, center.y - x, color);
        };

        let mut x: isize = 0;
        let mut y: isize = r as isize;
        let mut d: isize = 3 - 2 * r as isize;
        draw_subsequence_points(x, y);
        while y >= x {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
            draw_subsequence_points(x, y);
        }
    }

    /// https://www.geeksforgeeks.org/bresenhams-line-generation-algorithm/
    pub fn draw_line(&mut self, start: &IntPoint, end: &IntPoint, color: &Color) {
        let m_new: isize = 2 * (end.y - start.y);
        let mut slope_error_new: isize = m_new - (end.x - start.x);
        let mut x = start.x;
        let mut y = start.y;
        for _ in x..=end.x {
            self.set_pixel_internal(x, y, color);
            // Add slope to increment angle formed
            slope_error_new += m_new;

            // Slope error reached limit, time to
            // increment y and update slope error.
            if slope_error_new >= 0 {
                y += 1;
                slope_error_new -= 2 * (end.x - start.x);
            }
            x += 1;
        }
    }

    pub fn fill_rect(&mut self, rect: &IntRect, color: &Color) {
        for x in rect.x()..rect.x() + rect.width {
            for y in rect.y()..rect.y() + rect.height {
                self.set_pixel_internal(x, y, color);
            }
        }
    }
}
