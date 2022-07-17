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
    pub size: usize,
}

impl Brush {
    pub fn new(color: Color, size: usize) -> Self {
        Brush { color, size }
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
    brush: Brush,
    background: Color,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            buffer: vec![0; width * height * 3],
            brush: Brush::new(Color::new(0, 0, 0), 1),
            background: Color::new(255, 255, 255),
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

    pub fn set_brush(&mut self, brush: Brush) {
        self.brush = brush;
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn set_color(&mut self, color: Color) {
        self.brush.color = color;
    }

    fn cartesian_to_index(&self, width: usize, height: usize) -> usize {
        3 * (height * self.width + width)
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = match i % 3 {
                0 => self.background.red,
                1 => self.background.green,
                2 => self.background.blue,
                _ => panic!("Invalid modulus result"),
            };
        }
    }

    fn set_pixel_internal(&mut self, width: isize, height: isize) {
        if width < 0 || height < 0 {
            return;
        }
        let index = self.cartesian_to_index(width as usize, height as usize);
        if index >= self.buffer.len() {
            return;
        }
        self.buffer[index] = self.brush.color.red;
        self.buffer[index + 1] = self.brush.color.green;
        self.buffer[index + 2] = self.brush.color.blue;
    }

    pub fn set_pixel(&mut self, point: &IntPoint) {
        self.set_pixel_internal(point.x, point.y);
    }

    /// https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
    pub fn draw_circle(&mut self, center: &IntPoint, r: usize) {
        let mut draw_subsequence_points = |x: isize, y: isize| {
            self.set_pixel_internal(center.x + x, center.y + y);
            self.set_pixel_internal(center.x - x, center.y + y);
            self.set_pixel_internal(center.x + x, center.y - y);
            self.set_pixel_internal(center.x - x, center.y - y);
            self.set_pixel_internal(center.x + y, center.y + x);
            self.set_pixel_internal(center.x - y, center.y + x);
            self.set_pixel_internal(center.x + y, center.y - x);
            self.set_pixel_internal(center.x - y, center.y - x);
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
    pub fn draw_line(&mut self, start: &IntPoint, end: &IntPoint) {
        let m_new: isize = 2 * (end.y - start.y);
        let mut slope_error_new: isize = m_new - (end.x - start.x);
        let mut x = start.x;
        let mut y = start.y;
        for _ in x..=end.x {
            self.set_pixel_internal(x, y);
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

    pub fn fill_rect(&mut self, rect: &IntRect) {
        for x in rect.x()..rect.x() + rect.width {
            for y in rect.y()..rect.y() + rect.height {
                self.set_pixel_internal(x, y);
            }
        }
    }
}
