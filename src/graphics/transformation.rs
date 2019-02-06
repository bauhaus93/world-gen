use glm;
use glm::{ Vector3, Vector4, Matrix4, normalize };
use num_traits::One;

use crate::utility::Float;

pub fn create_transformation_matrix(translation: Vector3<Float>, rotation: Vector3<Float>, scale: Vector3<Float>) -> Matrix4<Float> {
    create_translation_matrix(translation) * create_rotation_matrix(rotation) * create_scale_matrix(scale) 
}

pub fn create_translation_matrix(translation: Vector3<Float>) -> Matrix4<Float> {
    glm::ext::translate(&Matrix4::<Float>::one(), translation)
}

pub fn create_rotation_matrix(rotation: Vector3<Float>) -> Matrix4<Float> {
    let one = Matrix4::<Float>::one();
    glm::ext::rotate(&one, rotation.x as Float, glm::Vector3::<Float>::new(1., 0., 0.)) *
    glm::ext::rotate(&one, rotation.y as Float, glm::Vector3::<Float>::new(0., 1., 0.)) *
    glm::ext::rotate(&one, rotation.z as Float, glm::Vector3::<Float>::new(0., 0., 1.))
}

pub fn create_scale_matrix(scale: Vector3<Float>) -> Matrix4<Float> {
    glm::ext::scale(&Matrix4::<Float>::one(), scale)
}

pub fn create_direction(rotation: Vector3<Float>) -> Vector3<Float> {
    normalize(Vector3::<Float>::new(
        rotation.y.sin() * rotation.x.cos(),
        rotation.y.sin() * rotation.x.sin(),
        rotation.y.cos()))
}

pub fn create_orthographic_projection_matrix(left: Float, right: Float, top: Float, bottom: Float, near: Float, far: Float) -> Matrix4<Float> {
    let trans_input = Matrix4::<Float>::new(Vector4::<Float>::new(1., 0., 0., 0.), Vector4::<Float>::new(0., 1., 0., 0.),
                                              Vector4::<Float>::new(0., 0., -1., 0.), Vector4::<Float>::new(0., 0., 0., 1.));
    create_scale_matrix(Vector3::<Float>::new(2. / (right - left), 2. / (top - bottom), 2. / (far - near))) * 
    glm::ext::translate(&trans_input, Vector3::<Float>::new(-(left + right) / 2., -(top + bottom) / 2., -(far + near) / 2.))
}
