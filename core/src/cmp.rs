use std::cmp::Ordering;

use glm::{ Vector2, Vector3 };

use super::Float;

pub fn cmp_vec2(lhs: &Vector2<Float>, rhs: &Vector2<Float>) -> Ordering {
    const THRESHOLD: Float = 1e-6;
    for i in 0..2 {
        let diff = lhs[i] - rhs[i];
        if diff < -THRESHOLD {
            return Ordering::Less;
        } else if diff > THRESHOLD {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}


pub fn cmp_vec3(lhs: &Vector3<Float>, rhs: &Vector3<Float>) -> Ordering {
    const THRESHOLD: Float = 1e-6;
    for i in 0..3 {
        let diff = lhs[i] - rhs[i];
        if diff < -THRESHOLD {
            return Ordering::Less;
        } else if diff > THRESHOLD {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}
