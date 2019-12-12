use std::convert::{TryFrom, TryInto};

use super::vertex_buffer::{BUFFER_NORMAL, BUFFER_POSTION, BUFFER_UV};
use super::{read_obj, triangles_to_buffers, MeshError, Triangle, VertexBuffer, VAO};
use crate::graphics::GraphicsError;
use crate::traits::{RenderInfo, Renderable};

pub struct Mesh {
    vao: Option<VAO>,
}

impl Mesh {
    pub fn from_obj(obj_path: &str) -> Result<Mesh, MeshError> {
        Mesh::try_from((read_obj(obj_path)?).as_slice())
    }

    pub fn from_obj_custom_buffers(obj_path: &str, buffer_flags: u8) -> Result<Mesh, MeshError> {
        let triangles = read_obj(obj_path)?;
        let (pos, uv, nm, index) = triangles_to_buffers(&triangles, buffer_flags);
        let mut vb = VertexBuffer::default();
        let mut attr_index = 0;
        if buffer_flags & BUFFER_POSTION != 0 {
            vb.add_float_buffer(pos, attr_index, 3);
            attr_index += 1;
        }
        if buffer_flags & BUFFER_UV != 0 {
            let uv_size = triangles[0].get_uv_dim();
            vb.add_float_buffer(uv, attr_index, uv_size.into());
            attr_index += 1;
        }
        if buffer_flags & BUFFER_NORMAL != 0 {
            vb.add_float_buffer(nm, attr_index, 3);
        }
        vb.set_index_buffer(index);
        vb.try_into()
    }

    pub fn get_vertex_count(&self) -> u32 {
        match self.vao {
            Some(ref vao) => vao.get_index_count(),
            _ => 0,
        }
    }
}

impl Renderable for Mesh {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        match self.vao {
            Some(ref vao) => vao.render(info)?,
            None => {}
        }
        Ok(())
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self { vao: None }
    }
}

impl TryFrom<&[Triangle]> for Mesh {
    type Error = MeshError;
    fn try_from(triangles: &[Triangle]) -> Result<Self, Self::Error> {
        let vb = VertexBuffer::from(triangles);
        let mesh = Self {
            vao: Some(vb.try_into()?),
        };
        Ok(mesh)
    }
}

impl TryFrom<VertexBuffer> for Mesh {
    type Error = MeshError;
    fn try_from(vb: VertexBuffer) -> Result<Self, Self::Error> {
        let mesh = Self {
            vao: Some(vb.try_into()?),
        };
        Ok(mesh)
    }
}
