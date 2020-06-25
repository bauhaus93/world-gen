use glm;

use super::{Point2, Point2i};

pub type Point2f = Point2<f32>;

impl Point2f {
    pub fn as_normalized(&self) -> Self {
        Self(glm::normalize(self.0))
    }

    pub fn length(&self) -> f32 {
        glm::length(self.0)
    }
}

impl From<Point2i> for Point2f {
    fn from(p: Point2i) -> Point2f {
        Point2f::new(p[0] as f32, p[1] as f32)
    }
}
