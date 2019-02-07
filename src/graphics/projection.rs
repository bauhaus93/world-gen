use crate::utility::Float;

#[derive(Copy, Clone)]
pub enum Projection {
    Perspective { fov: Float, aspect_ratio: Float, near: Float, far: Float },
    Orthographic { width: Float, aspect_ratio: Float }
}

pub fn create_orthographic_projection(width: Float, aspect_ratio: Float) -> Projection {
    Projection::Orthographic {
        width: width,
        aspect_ratio: aspect_ratio,
    }
}

pub fn create_default_perspective() -> Projection {
    Projection::Perspective { fov: 90f32.to_radians(), aspect_ratio: 4./3., near: 0.5, far: 500. }
}

pub fn create_default_orthographic() -> Projection {
    Projection::Orthographic { width: 20., aspect_ratio: 4./3. }
}



