use glm;
use glm::{ Vector3, Vector4, Matrix4 };

use crate::Float;
use crate::graphics::transformation::create_scale_matrix;

#[derive(Copy, Clone)]
pub enum Projection {
    Perspective { fov: Float, aspect_ratio: Float, near: Float, far: Float },
    Orthographic { width: Float, aspect_ratio: Float }
}

#[allow(dead_code)]
pub fn create_orthographic_projection(width: Float, aspect_ratio: Float) -> Projection {
    Projection::Orthographic {
        width: width,
        aspect_ratio: aspect_ratio,
    }
}

pub fn create_default_perspective() -> Projection {
    Projection::Perspective { fov: 90f32.to_radians(), aspect_ratio: 4./3., near: 0.5, far: 10000. }
}

pub fn create_default_orthographic() -> Projection {
    Projection::Orthographic { width: 2500., aspect_ratio: 4./3. }
}

pub fn create_orthographic_projection_matrix(left: Float, right: Float, top: Float, bottom: Float, near: Float, far: Float) -> Matrix4<Float> {
    let trans_input = Matrix4::<Float>::new(Vector4::<Float>::new(1., 0., 0., 0.), Vector4::<Float>::new(0., 1., 0., 0.),
                                              Vector4::<Float>::new(0., 0., -1., 0.), Vector4::<Float>::new(0., 0., 0., 1.));
    create_scale_matrix(Vector3::<Float>::new(2. / (right - left), 2. / (top - bottom), 2. / (far - near))) * 
    glm::ext::translate(&trans_input, Vector3::<Float>::new(-(left + right) / 2., -(top + bottom) / 2., -(far + near) / 2.))
}

