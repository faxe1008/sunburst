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

pub struct Brush {
    pub color: Color,
    pub thickness: usize,
}

impl Brush {
    pub fn new(color: Color, thickness: usize) -> Self {
        Brush { color, thickness }
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

    fn set_pixel(&mut self, width: isize, height: isize, color: &Color) {
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

    /// https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
    pub fn draw_circle(&mut self, xc: usize, yc: usize, r: usize, color: &Color) {
        let mut draw_subsequence_points = |x: isize, y: isize| unsafe {
            self.set_pixel(xc as isize + x, yc as isize + y, color);
            self.set_pixel(xc as isize - x, yc as isize + y, color);
            self.set_pixel(xc as isize + x, yc as isize - y, color);
            self.set_pixel(xc as isize - x, yc as isize - y, color);
            self.set_pixel(xc as isize + y, yc as isize + x, color);
            self.set_pixel(xc as isize - y, yc as isize + x, color);
            self.set_pixel(xc as isize + y, yc as isize - x, color);
            self.set_pixel(xc as isize - y, yc as isize - x, color);
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
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: &Color) {
        let m_new: isize = 2 * (y2 as isize - y1 as isize);
        let mut slope_error_new: isize = m_new - (x2 as isize - x1 as isize);
        let mut x = x1 as isize;
        let mut y = y1 as isize;
        for _ in x..=x2 as isize {
            self.set_pixel(x, y, color);
            // Add slope to increment angle formed
            slope_error_new += m_new;

            // Slope error reached limit, time to
            // increment y and update slope error.
            if slope_error_new >= 0 {
                y += 1;
                slope_error_new -= 2 * (x2 as isize - x1 as isize);
            }
            x += 1;
        }
    }
}
