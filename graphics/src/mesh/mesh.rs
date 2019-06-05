use std::convert::TryFrom;

use crate::mesh::{ VAO, Triangle, MeshError, read_obj::read_obj };
use crate::mesh::vao_creation::create_vao_from_triangles;

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

impl TryFrom<&[Triangle]> for Mesh {
    type Error = MeshError;
    fn try_from(triangles: &[Triangle]) -> Result<Self, Self::Error> {
        let vao = create_vao_from_triangles(triangles)?;
        let mesh = Self {
            vao: Some(vao)
        };
        Ok(mesh)
    }
}
