use std::convert::TryFrom;

use crate::mesh::{ VAO, Triangle, MeshError, Buffer, read_obj::read_obj };

pub struct Mesh {
    vao: Option<VAO>
}

impl Mesh {
    pub fn from_obj(obj_path: &str) -> Result<Mesh, MeshError> {
        Self::try_from((read_obj(obj_path)?).as_slice())
    }

    pub fn get_vertex_count(&self) -> u32 {
        match self.vao {
            Some(ref vao) => vao.get_index_count(),
            _ => 0
        }
    }

    pub fn render(&self) -> Result<(), MeshError> {
        match self.vao {
            Some(ref vao) => vao.render(),
            None => { Ok(()) }
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vao: None
        }
    }
}

impl TryFrom<Buffer> for Mesh {
    type Error = MeshError;
    fn try_from(buffer: Buffer) -> Result<Self, Self::Error> {
        let mesh = Self {
            vao: Some(VAO::try_from(buffer)?)
        };
        Ok(mesh)
    }
}

impl TryFrom<&[Triangle]> for Mesh {
    type Error = MeshError;
    fn try_from(triangles: &[Triangle]) -> Result<Self, Self::Error> {
        let buffer = Buffer::from(triangles);
        let mesh = Self {
            vao: Some(VAO::try_from(buffer)?)
        };
        Ok(mesh)
    }
}
