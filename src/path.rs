use crate::primitives::IntPoint;

pub enum PathSegment {
    MoveTo(IntPoint),
    LineTo(IntPoint),
}

pub struct Path {
    segments: Vec<PathSegment>,
}

impl Path {
    pub fn new() -> Self {
        Path {
            segments: Vec::new(),
        }
    }

    pub fn move_to(&mut self, pt: &IntPoint) {
        self.segments.push(PathSegment::MoveTo(pt.clone()));
    }

    pub fn line_to(&mut self, pt: &IntPoint) {
        self.segments.push(PathSegment::LineTo(pt.clone()));
    }

    pub fn clear(&mut self) {
        self.segments.clear();
    }

    pub fn segments(&self) -> &Vec<PathSegment> {
        &self.segments
    }
}
