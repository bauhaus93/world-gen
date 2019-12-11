use crate::graphics::{ Mesh, GraphicsError };
use super::ObjectError;

pub struct ObjectPrototype {
    lod_meshes: [Mesh; 2]
}

impl ObjectPrototype {

    pub fn from_obj(lod0_path: &str, lod1_path: &str) -> Result<ObjectPrototype, ObjectError> {
        let lod0 = Mesh::from_obj(lod0_path)?;
        let lod1 = Mesh::from_obj(lod1_path)?;
        let proto = ObjectPrototype {
            lod_meshes: [lod0, lod1]
        };
        Ok(proto)
    }

    pub fn render(&self, lod: u8) -> Result<(), GraphicsError> {
        debug_assert!(lod < 2);
        self.lod_meshes[lod as usize].render()?;
        Ok(())
    }
}

