use crate::Point3f;

#[derive(Clone, Copy)]
pub struct Light {
    color: Point3f,
    world_pos: Point3f,
    absolute_intensity: f32,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

impl Light {
    pub fn get_color(&self) -> Point3f {
        self.color
    }
    pub fn get_world_pos(&self) -> Point3f {
        self.world_pos
    }
    pub fn get_absolute_intensity(&self) -> f32 {
        self.absolute_intensity
    }
    pub fn get_ambient_intensity(&self) -> f32 {
        self.ambient_intensity
    }
    pub fn get_diffuse_intensity(&self) -> f32 {
        self.diffuse_intensity
    }
    pub fn get_specular_intensity(&self) -> f32 {
        self.specular_intensity
    }
    pub fn get_specular_shininess(&self) -> f32 {
        self.specular_shininess
    }

    pub fn set_color(&mut self, new_color: Point3f) {
        self.color = new_color;
    }
    pub fn set_world_pos(&mut self, new_pos: Point3f) {
        self.world_pos = new_pos;
    }
    pub fn set_absolute_intensity(&mut self, new_intensity: f32) {
        self.absolute_intensity = new_intensity;
    }
    pub fn set_ambient_intensity(&mut self, new_intensity: f32) {
        self.ambient_intensity = new_intensity;
    }
    pub fn set_diffuse_intensity(&mut self, new_intensity: f32) {
        self.diffuse_intensity = new_intensity;
    }

    pub fn set_specular_intensity(&mut self, new_intensity: f32) {
        self.specular_intensity = new_intensity;
    }
    pub fn set_specular_shininess(&mut self, new_shininess: f32) {
        self.specular_shininess = new_shininess;
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            color: Point3f::from_scalar(1.),
            world_pos: Point3f::from_scalar(0.),
            absolute_intensity: 100.,
            ambient_intensity: 0.1,
            diffuse_intensity: 0.5,
            specular_intensity: 0.2,
            specular_shininess: 2.,
        }
    }
}
