use std::sync::Arc;

use super::ObjectPrototype;
use crate::graphics::GraphicsError;
use crate::traits::{RenderInfo, Renderable, Rotatable, Scalable, Translatable};
use crate::{Model, Point3f};

pub struct Object {
    prototype: Arc<ObjectPrototype>,
    model: Model,
}

impl Object {
    pub fn new(prototype: Arc<ObjectPrototype>) -> Object {
        Object {
            prototype: prototype,
            model: Model::default(),
        }
    }
}

impl Renderable for Object {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        let mvp = info.get_camera().create_mvp_matrix(&self.model);
        let shader = info.get_active_shader();
        shader.set_resource_mat4("mvp", &mvp)?;
        shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
        self.prototype.render(info)
    }
}

impl Translatable for Object {
    fn set_translation(&mut self, new_translation: Point3f) {
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Point3f {
        self.model.get_translation()
    }
}

impl Rotatable for Object {
    fn set_rotation(&mut self, new_rotation: Point3f) {
        self.model.set_rotation(new_rotation);
    }
    fn get_rotation(&self) -> Point3f {
        self.model.get_rotation()
    }
}

impl Scalable for Object {
    fn set_scale(&mut self, new_scale: Point3f) {
        self.model.set_scale(new_scale);
    }
    fn get_scale(&self) -> Point3f {
        self.model.get_scale()
    }
}
