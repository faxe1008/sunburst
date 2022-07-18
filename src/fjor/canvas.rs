pub use super::color::Color;

#[derive(Debug, Clone)]
pub struct IntPoint {
    pub x: isize,
    pub y: isize,
}

impl IntPoint {
    pub fn new(x: isize, y: isize) -> Self {
        IntPoint { x, y }
    }
}

#[derive(Debug, Clone)]
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
    fill: Option<Color>,
    stroke: Option<Color>,
    background: Color,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            buffer: vec![0; width * height * 3],
            stroke: Some(Color::rgb(0, 0, 0)),
            fill: None,
            background: Color::rgb(255, 255, 255),
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

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn stroke(&mut self, color: Color) {
        self.stroke.insert(color);
    }

    pub fn fill(&mut self, color: Color) {
        self.fill.insert(color);
    }

    pub fn no_fill(&mut self) {
        self.fill = None;
    }

    pub fn no_stroke(&mut self) {
        self.stroke = None;
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

    fn stroke_pixel_internal(&mut self, width: isize, height: isize) {
        if let Some(color) = self.stroke.as_ref() {
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
    }

    fn fill_pixel_internal(&mut self, width: isize, height: isize) {
        if let Some(color) = self.fill.as_ref() {
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
    }

    pub fn draw_point(&mut self, point: &IntPoint) {
        self.stroke_pixel_internal(point.x, point.y);
    }

    pub fn draw_circle(&mut self, center: &IntPoint, r: usize) {
        let r_sq = r as isize * r as isize;
        for x in -1 * r as isize..r as isize {
            let f = (r_sq - x * x) as f32;
            let height = f.sqrt() as isize;
            let x_coor = x + center.x;
            for y in -height..height {
                self.fill_pixel_internal(x_coor, y + center.y);
            }
            self.stroke_pixel_internal(x_coor, height + center.y);
            self.stroke_pixel_internal(x_coor, -height + center.y);
        }
    }

    /// https://www.geeksforgeeks.org/bresenhams-line-generation-algorithm/
    pub fn draw_line(&mut self, start: &IntPoint, end: &IntPoint) {
        let m_new: isize = 2 * (end.y - start.y);
        let mut slope_error_new: isize = m_new - (end.x - start.x);
        let mut x = start.x;
        let mut y = start.y;
        for _ in x..=end.x {
            self.stroke_pixel_internal(x, y);
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

    pub fn draw_rect(&mut self, rect: &IntRect) {
        let rect_bottom_y = rect.y() + rect.height;
        let rect_bottom_x = rect.x() + rect.width;

        for x in rect.x()..=rect_bottom_y {
            for y in rect.y()..=rect_bottom_x {
                if x == rect.x() || x == rect_bottom_x || y == rect.y() || y == rect_bottom_y {
                    self.stroke_pixel_internal(x, y);
                } else {
                    self.fill_pixel_internal(x, y);
                }
            }
        }
    }
}
