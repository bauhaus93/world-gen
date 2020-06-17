use glm;
use glm::Matrix4;
use num_traits::One;

use crate::Point3f;

pub fn create_transformation_matrix(
    translation: Point3f,
    rotation: Point3f,
    scale: Point3f,
) -> Matrix4<f32> {
    create_translation_matrix(translation)
        * create_rotation_matrix(rotation)
        * create_scale_matrix(scale)
}

pub fn create_translation_matrix(translation: Point3f) -> Matrix4<f32> {
    glm::ext::translate(&Matrix4::<f32>::one(), translation.as_glm())
}

pub fn create_rotation_matrix(rotation: Point3f) -> Matrix4<f32> {
    let one = Matrix4::<f32>::one();
    glm::ext::rotate(&one, rotation[0], glm::Vector3::<f32>::new(1., 0., 0.))
        * glm::ext::rotate(&one, rotation[1], glm::Vector3::<f32>::new(0., 1., 0.))
        * glm::ext::rotate(&one, rotation[2], glm::Vector3::<f32>::new(0., 0., 1.))
}

pub fn create_scale_matrix(scale: Point3f) -> Matrix4<f32> {
    glm::ext::scale(&Matrix4::<f32>::one(), scale.as_glm())
}

pub fn create_direction(rotation: Point3f) -> Point3f {
    Point3f::new(
        rotation[1].sin() * rotation[0].cos(),
        rotation[1].sin() * rotation[0].sin(),
        rotation[1].cos(),
    )
    .as_normalized()
}
