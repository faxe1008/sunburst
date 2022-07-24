use crate::primitives::IntPoint;

#[derive(Clone)]
pub enum PathSegment {
    MoveTo(IntPoint),
    LineTo(IntPoint),
}

#[derive(Clone)]
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

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}
