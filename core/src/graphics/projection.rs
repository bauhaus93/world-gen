use glm;
use glm::{ Vector3, Vector4, Matrix4 };

use crate::graphics::transformation::create_scale_matrix;
use crate::Point3f;

#[derive(Copy, Clone)]
pub enum Projection {
    Perspective { fov: f32, aspect_ratio: f32, near: f32, far: f32 },
    Orthographic { width: f32, aspect_ratio: f32 }
}

#[allow(dead_code)]
pub fn create_orthographic_projection(width: f32, aspect_ratio: f32) -> Projection {
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

pub fn create_orthographic_projection_matrix(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Matrix4<f32> {
    let trans_input = Matrix4::<f32>::new(Vector4::<f32>::new(1., 0., 0., 0.), Vector4::<f32>::new(0., 1., 0., 0.),
                                              Vector4::<f32>::new(0., 0., -1., 0.), Vector4::<f32>::new(0., 0., 0., 1.));
    create_scale_matrix(Point3f::new(2. / (right - left), 2. / (top - bottom), 2. / (far - near))) *
    glm::ext::translate(&trans_input, Vector3::<f32>::new(-(left + right) / 2., -(top + bottom) / 2., -(far + near) / 2.))
}

