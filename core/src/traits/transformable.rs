use std::ops::Add;
use glm::{ Vector3, Matrix4 };

use utility::Float;

pub trait Transformable {
    fn transform(&mut self, transformation_matrix: Matrix4<Float>);
}