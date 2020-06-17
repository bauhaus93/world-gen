use glm::ext::{look_at, perspective};
use glm::{Matrix4, Vector3};
use num_traits::One;

use crate::graphics::{
    create_direction, projection::create_default_perspective, Model, Projection,
};
use crate::traits::{Rotatable, Translatable};
use crate::Point3f;

pub struct Camera {
    model: Model,
    projection: Projection,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn set_far(&mut self, new_far: f32) {
        match &mut self.projection {
            Projection::Perspective { far, .. } => {
                *far = new_far;
            }
            Projection::Orthographic { width, .. } => {
                *width = new_far / 2.;
            }
        }
        self.update_projection();
    }

    pub fn create_mvp_matrix(&self, model: &Model) -> Matrix4<f32> {
        self.projection_matrix * self.view_matrix * model.get_matrix()
    }

    pub fn set_projection(&mut self, new_projection: Projection) {
        self.projection = new_projection;
        self.update_projection();
    }

    pub fn get_projection(&self) -> Projection {
        self.projection
    }

    pub fn get_direction(&self) -> Point3f {
        create_direction(self.model.get_rotation())
    }

    fn update_view(&mut self) {
        let direction = create_direction(self.model.get_rotation());
        self.view_matrix = look_at(
            self.model.get_translation().as_glm(),
            (self.model.get_translation() + direction).as_glm(),
            Vector3::<f32>::new(0., 0., 1.),
        );
    }

    fn update_projection(&mut self) {
        self.projection_matrix = match self.projection {
            Projection::Perspective {
                fov,
                aspect_ratio,
                near,
                far,
            } => {
                trace!("projection update: perspective, fov = {}, aspect ratio = {}, near = {}, far = {}", fov.to_degrees(), aspect_ratio, near, far);
                perspective(fov, aspect_ratio, near, far)
            }
            _ => unreachable!(),
        }
    }
}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            model: Model::default(),
            projection: create_default_perspective(),
            view_matrix: Matrix4::<f32>::one(),
            projection_matrix: Matrix4::<f32>::one(),
        };
        camera.set_rotation(Point3f::new(45f32.to_radians(), 125f32.to_radians(), 0.));
        camera.update_projection();
        camera
    }
}

impl Translatable for Camera {
    fn set_translation(&mut self, new_translation: Point3f) {
        self.model.set_translation(new_translation);
        self.update_view();
    }
    fn get_translation(&self) -> Point3f {
        self.model.get_translation()
    }
}

impl Rotatable for Camera {
    fn set_rotation(&mut self, new_rotation: Point3f) {
        const THRESHOLD: f32 = 0.01;
        const MIN_Y: f32 = THRESHOLD;
        const MAX_Y: f32 = std::f32::consts::PI - THRESHOLD;
        const DOUBLE_PI: f32 = 2. * std::f32::consts::PI;
        let mut fixed_rotation = new_rotation;
        if fixed_rotation[0] >= DOUBLE_PI {
            fixed_rotation[0] -= DOUBLE_PI;
        } else if fixed_rotation[0] < 0. {
            fixed_rotation[0] += DOUBLE_PI;
        }
        if fixed_rotation[1] < MIN_Y {
            fixed_rotation[1] = MIN_Y;
        } else if fixed_rotation[1] > MAX_Y {
            fixed_rotation[1] = MAX_Y;
        }

        self.model.set_rotation(fixed_rotation);
        self.update_view();
    }
    fn get_rotation(&self) -> Point3f {
        self.model.get_rotation()
    }
}
