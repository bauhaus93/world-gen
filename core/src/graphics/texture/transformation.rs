use gl::types::GLfloat;
use glm::Vector4;

use crate::Point2i;

pub fn points2d_to_glm4d(vec: &[Point2i]) -> Vec<Vector4<GLfloat>> {
    let mut output = Vec::with_capacity(vec.len());
    for p in vec {
        output.push(Vector4::new(p[0] as GLfloat, p[1] as GLfloat, 0., 0.));
    }
    output.reverse();
    output
}

pub fn glm4d_to_floats(data: &[Vector4<GLfloat>]) -> Vec<f32> {
    data.iter().map(|v| v.x as f32).collect()
}
