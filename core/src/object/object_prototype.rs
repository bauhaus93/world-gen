use super::ObjectError;
use crate::graphics::{GraphicsError, Mesh};
use crate::traits::{RenderInfo, Renderable};

pub struct ObjectPrototype {
    lod_meshes: [Mesh; 2],
}

impl ObjectPrototype {
    pub fn from_obj(lod0_path: &str, lod1_path: &str) -> Result<ObjectPrototype, ObjectError> {
        let lod0 = Mesh::from_obj(lod0_path)?;
        let lod1 = Mesh::from_obj(lod1_path)?;
        let proto = ObjectPrototype {
            lod_meshes: [lod0, lod1],
        };
        Ok(proto)
    }
}

impl Renderable for ObjectPrototype {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        debug_assert!(info.get_lod() < 2);
        self.lod_meshes[info.get_lod() as usize].render(info)
    }
}
