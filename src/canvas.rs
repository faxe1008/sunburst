use crate::path::PathSegment;
use crate::pixelbuffer::PixelBuffer;

pub use super::color::Color;
pub use super::path::Path;
pub use super::primitives::{IntPoint, IntRect};

use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapHeight};
use std::cmp::max;
use std::cmp::min;
use std::mem::swap;

pub enum FontWeight {
    Light,
    Regular,
    Bold,
}

pub struct TextStyle {
    weight: FontWeight,
    size: usize,
}

impl TextStyle {
    pub fn new(size: usize, weight: FontWeight) -> Self {
        TextStyle { size, weight }
    }
}

pub struct Canvas {
    pixelbuffer: PixelBuffer,
    fill: Option<Color>,
    stroke: Option<Color>,
    background: Color,
    text_style: TextStyle,
}

enum ColorSource {
    Fill,
    Stroke,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            pixelbuffer: PixelBuffer::new(width, height),
            stroke: Some(Color::rgb(0, 0, 0)),
            fill: None,
            background: Color::rgb(255, 255, 255),
            text_style: TextStyle::new(16, FontWeight::Regular),
        }
    }

    pub fn height(&self) -> usize {
        self.pixelbuffer.height()
    }

    pub fn width(&self) -> usize {
        self.pixelbuffer.width()
    }

    pub fn as_raw_buffer(&self) -> &[u8] {
        self.pixelbuffer.as_raw_buffer()
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn stroke(&mut self, color: Color) {
        self.stroke = Some(color);
    }

    pub fn fill(&mut self, color: Color) {
        self.fill = Some(color);
    }

    pub fn no_fill(&mut self) {
        self.fill = None;
    }

    pub fn no_stroke(&mut self) {
        self.stroke = None;
    }

    pub fn font_size(&mut self, size: usize) {
        self.text_style.size = size;
    }

    pub fn font_weight(&mut self, weight: FontWeight) {
        self.text_style.weight = weight;
    }

    pub fn clear(&mut self) {
        self.pixelbuffer.clear(&self.background);
    }

    fn set_pixel_from_color_source(&mut self, x: isize, y: isize, color_source: ColorSource) {
        let color = match color_source {
            ColorSource::Fill if self.fill.is_some() => self.fill.as_ref().unwrap(),
            ColorSource::Stroke if self.stroke.is_some() => self.stroke.as_ref().unwrap(),
            _ => {
                return;
            }
        };
        self.pixelbuffer.set_pixel(x, y, color);
    }

    pub fn draw_point(&mut self, point: &IntPoint) {
        self.set_pixel_from_color_source(point.x, point.y, ColorSource::Stroke);
    }

    /// https://www.geeksforgeeks.org/bresenhams-line-generation-algorithm/
    pub fn draw_line(&mut self, start: &IntPoint, end: &IntPoint) {
        // vertical line
        if start.x == end.x {
            let min_y = min(start.y, end.y);
            let max_y = max(start.y, end.y);
            //TODO: step by stroke weight
            for y in (min_y..max_y).step_by(1) {
                self.set_pixel_from_color_source(start.x, y, ColorSource::Stroke);
            }
        }

        // horizontal line
        if start.y == end.y {
            let min_x = min(start.x, end.x);
            let max_x = max(start.x, end.x);
            //TODO: step by stroke weight
            for x in (min_x..max_x).step_by(1) {
                self.set_pixel_from_color_source(x, start.y, ColorSource::Stroke);
            }
        }

        // TODO: avoid this clone
        let mut point1 = start.clone();
        let mut point2 = end.clone();

        let adx = (point2.x - point1.x).abs();
        let ady = (point2.y - point1.y).abs();
        if adx > ady {
            if point1.x > point2.x {
                swap(&mut point1, &mut point2);
            }
        } else {
            if point1.y > point2.y {
                swap(&mut point1, &mut point2);
            }
        }

        let dx = point2.x - point1.x;
        let dy = point2.y - point1.y;
        let mut error = 0;

        if dx > dy {
            let y_step = dy.signum();
            let delta_error = 2 * dy.abs();
            let mut y = point1.y;
            for x in point1.x..=point2.x {
                self.set_pixel_from_color_source(x, y, ColorSource::Stroke);
                error += delta_error;
                if error >= dx {
                    y += y_step;
                    error -= 2 * dx;
                }
            }
        } else {
            let x_step = dx.signum();
            let delta_error = 2 * dx.abs();
            let mut x = point1.x;
            for y in point1.y..=point2.y {
                self.set_pixel_from_color_source(x, y, ColorSource::Stroke);
                error += delta_error;
                if error >= dy {
                    x += x_step;
                    error -= 2 * dy;
                }
            }
        }
    }

    pub fn draw_rect(&mut self, rect: &IntRect) {
        if self.fill.is_some() {
            for x in rect.x()..=rect.x() + rect.width {
                for y in rect.y()..=rect.y() + rect.height {
                    self.set_pixel_from_color_source(x, y, ColorSource::Fill);
                }
            }
        }

        if self.stroke.is_some() {
            let top_right = IntPoint::new(rect.x() + rect.width, rect.y());
            let lower_left = IntPoint::new(rect.x(), rect.y() + rect.height);
            let lower_right = IntPoint::new(rect.x() + rect.width, rect.y() + rect.height);

            self.draw_line(&rect.location, &top_right);
            self.draw_line(&top_right, &lower_right);
            self.draw_line(&lower_right, &lower_left);
            self.draw_line(&lower_left, &rect.location);
        }
    }

    pub fn draw_square(&mut self, origin: &IntPoint, size: isize) {
        let rect = IntRect::new(origin.clone(), size, size);
        self.draw_rect(&rect);
    }

    pub fn draw_text(&mut self, origin: &IntPoint, msg: &str) {
        if self.fill.is_none() {
            return;
        }

        let font_weight = match self.text_style.weight {
            FontWeight::Light => noto_sans_mono_bitmap::FontWeight::Light,
            FontWeight::Regular => noto_sans_mono_bitmap::FontWeight::Regular,
            FontWeight::Bold => noto_sans_mono_bitmap::FontWeight::Bold,
        };

        let bitmap_height = match self.text_style.size {
            0..=14 => BitmapHeight::Size14,
            15..=16 => BitmapHeight::Size16,
            17..=18 => BitmapHeight::Size18,
            19..=20 => BitmapHeight::Size20,
            21..=22 => BitmapHeight::Size22,
            23..=24 => BitmapHeight::Size24,
            25..=32 => BitmapHeight::Size32,
            33..=64 => BitmapHeight::Size64,
            65..=usize::MAX => BitmapHeight::Size64,
            _ => BitmapHeight::Size64,
        };
        let char_width = get_bitmap_width(font_weight, bitmap_height);
        let mut y_origin = origin.y;

        let lines = msg.split('\n');

        for line in lines {
            for (char_i, char) in line.chars().enumerate() {
                let bitmap_char = get_bitmap(char, font_weight, bitmap_height).unwrap_or(
                    //Fall back to whitespace for unknown char
                    get_bitmap(' ', font_weight, bitmap_height).unwrap(),
                );

                for (row_i, row) in bitmap_char.bitmap().iter().enumerate() {
                    for (col_i, intensity) in row.iter().enumerate() {
                        let x = origin.x + char_i as isize * char_width as isize + col_i as isize;
                        let y = y_origin + row_i as isize;

                        // TODO: blitting with opacity
                        if *intensity > 80 {
                            self.set_pixel_from_color_source(x, y, ColorSource::Fill);
                        }
                    }
                }
            }
            y_origin += bitmap_height as isize;
        }
    }

    pub fn draw_path(&mut self, path: &Path) {
        if self.stroke.is_some() {
            let mut cursor = IntPoint::new(0, 0);

            for segment in path.segments() {
                match segment {
                    PathSegment::MoveTo(pt) => {
                        cursor = pt.clone();
                    }
                    PathSegment::LineTo(pt) => {
                        self.draw_line(&cursor, pt);
                        cursor = pt.clone();
                    }
                }
            }
        }
    }

    pub fn draw_ellipse(&mut self, center: &IntPoint, width: usize, height: usize) {
        let h = height as isize;
        let w = width as isize;
        let hh = h * h;
        let ww = w * w;
        let hhww = hh * ww;
        let mut x0 = w;
        let mut dx = 0;

        for x in -w..w {
            self.set_pixel_from_color_source(center.x + x, center.y, ColorSource::Fill);
        }

        for y in 1..=h {
            let mut x1 = x0 - (dx - 1);
            while x1 > 0 {
                if x1 * x1 * hh + y * y * ww <= hhww {
                    break;
                }
                x1 -= 1;
            }
            dx = x0 - x1;
            x0 = x1;
            for x in -(x0 - 1)..x0 {
                self.set_pixel_from_color_source(center.x + x, center.y - y, ColorSource::Fill);
                self.set_pixel_from_color_source(center.x + x, center.y + y, ColorSource::Fill);
            }
            self.set_pixel_from_color_source(center.x + x0, center.y - y, ColorSource::Stroke);
            self.set_pixel_from_color_source(center.x + x0, center.y + y, ColorSource::Stroke);
            self.set_pixel_from_color_source(center.x - x0, center.y + y, ColorSource::Stroke);
            self.set_pixel_from_color_source(center.x - x0, center.y - y, ColorSource::Stroke);
        }
    }
}
