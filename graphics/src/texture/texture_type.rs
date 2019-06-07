use std::collections::BTreeMap;
use gl::types::GLenum;

pub enum TextureType {
    Single2D,
    Array2D { index_list: Vec<[u32; 3]>, size: [u32; 2] },
    CubeMap { origin_map: BTreeMap<GLenum, [u32; 2]>, size: u32 }
}
