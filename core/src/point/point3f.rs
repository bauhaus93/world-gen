use glm;
use std::cmp::Ordering;

use super::Point3;

pub type Point3f = Point3<f32>;

const EPSILON: f32 = 1e-6;

impl Point3f {
    pub fn as_normalized(&self) -> Self {
        Self(glm::normalize(self.0))
    }

    pub fn get_length(&self) -> f32 {
        glm::length(self.0)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(glm::cross(self.0, rhs.0))
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        glm::dot(self.0, rhs.0)
    }
}

impl Ord for Point3f {
    fn cmp(&self, other: &Self) -> Ordering {
        match self[0] - other[0] {
            diff if diff >= EPSILON => Ordering::Greater,
            diff if diff <= -EPSILON => Ordering::Less,
            _ => match self[1] - other[1] {
                diff if diff >= EPSILON => Ordering::Greater,
                diff if diff <= -EPSILON => Ordering::Less,
                _ => match self[2] - other[2] {
                    diff if diff >= EPSILON => Ordering::Greater,
                    diff if diff <= -EPSILON => Ordering::Less,
                    _ => Ordering::Equal,
                },
            },
        }
    }
}

impl PartialOrd for Point3f {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point3f {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Point3f {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3f_eq() {
        assert_eq!(
            true,
            Point3f::new(3.9, 1.8, 2.7) == Point3f::new(3.9, 1.8, 2.7)
        );
    }

    #[test]
    fn test_point3f_first_less() {
        assert_eq!(
            true,
            Point3f::new(3.9, 1.8, 2.6) < Point3f::new(3.9 + 2. * EPSILON, 1.8, 2.6)
        );
    }

    #[test]
    fn test_point3f_second_less() {
        assert_eq!(
            true,
            Point3f::new(3.9, 1.8, 2.6) < Point3f::new(3.9, 1.8 + 2. * EPSILON, 2.6)
        );
    }

    #[test]
    fn test_point3f_third_less() {
        assert_eq!(
            true,
            Point3f::new(3.9, 1.8, 2.6) < Point3f::new(3.9, 1.8, 2.6 + 2. * EPSILON)
        );
    }
}
