use std::cmp::Ordering;

use glm;

use super::{Point2, Point2i};

const EPSILON: f32 = 1e-6;

pub type Point2f = Point2<f32>;

impl Point2f {
    pub fn as_normalized(&self) -> Self {
        Self(glm::normalize(self.0))
    }

    pub fn rotate_ccw_90(&self) -> Self {
        Point2f::new(self[1], -self[0])
    }

    pub fn length(&self) -> f32 {
        glm::length(self.0)
    }
}

impl Ord for Point2f {
    fn cmp(&self, other: &Self) -> Ordering {
        match self[0] - other[0] {
            diff if diff >= EPSILON => Ordering::Greater,
            diff if diff <= -EPSILON => Ordering::Less,
            _ => match self[1] - other[1] {
                diff if diff >= EPSILON => Ordering::Greater,
                diff if diff <= -EPSILON => Ordering::Less,
                _ => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Point2f {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point2f {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Point2f {}

impl From<Point2i> for Point2f {
    fn from(p: Point2i) -> Point2f {
        Point2f::new(p[0] as f32, p[1] as f32)
    }
}
