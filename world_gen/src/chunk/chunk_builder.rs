use std::convert::TryFrom;
use std::cmp::{ min, max };

use glm::{ Vector2, Vector3 };

use utility::Float;
use graphics::mesh::{ Vertex, Triangle, Mesh, VertexBuffer };
use crate::{ Object, ObjectManager };
use crate::traits::{ Translatable };
use super::{ Chunk, ChunkError, HeightMap, Architect, CHUNK_SIZE };

pub struct ChunkBuilder {
    pos: [i32; 2],
    lod: u8,
    height_map: HeightMap,
    tree_list: Vec<Object>,
    vertex_buffer: Option<VertexBuffer>
}

impl ChunkBuilder {

    pub fn new(pos: [i32; 2], lod: u8, architect: &Architect) -> Self {
        let height_map = match lod {
            0 => architect.create_height_map(pos, CHUNK_SIZE, 1),
            _ => architect.create_height_map(pos, CHUNK_SIZE / 8, 8),
        };
        Self {
            pos: pos,
            lod: lod,
            height_map: height_map,
            tree_list: Vec::new(),
            vertex_buffer: None
        }
    }

    pub fn finish(self) -> Result<Chunk, ChunkError> {
        let mesh = match self.vertex_buffer {
            Some(vb) => Mesh::try_from(vb)?,
            _ => { return Err(ChunkError::NoBufferBuilt(self.pos)); }
        };
        let mut chunk = Chunk::new(self.pos, self.height_map, self.lod, mesh);
        self.tree_list.into_iter().for_each(|t| chunk.add_tree(t));
        Ok(chunk)
    }

    pub fn load_trees(&mut self, architect: &Architect, object_manager: &ObjectManager) -> Result<(), ChunkError> {
        if self.lod < 2 {
            let resolution = self.height_map.get_resolution();
            let size = self.height_map.get_size();
            for y in 0..size {
                for x in 0..size {
                    let abs_pos = [((self.pos[0] * CHUNK_SIZE) + x * resolution) as Float,
                                   ((self.pos[1] * CHUNK_SIZE) + y * resolution) as Float];
                    if architect.has_tree(abs_pos) {
                        let mut tree = object_manager.create_object("tree")?;

                        tree.set_translation(Vector3::new(abs_pos[0], abs_pos[1], self.height_map.get(&[x, y])));
                        self.tree_list.push(tree);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn create_surface_buffer(&mut self) {
        let size = self.height_map.get_size();
        let mut triangles: Vec<Triangle> = Vec::with_capacity((size * size * 2) as usize);
        for y in 0..size - 1 {
            for x in 0..size - 1 {
                triangles.extend(&add_quad_triangles(&[x, y], &self.height_map));
            }
        }
        trace!("Created chunk vertices for {}/{}: triangle count = {}", self.pos[0], self.pos[1], triangles.len());
        self.vertex_buffer = Some(VertexBuffer::from(triangles.as_slice()));
    }
}

fn add_quad_triangles(offset: &[i32; 2], height_map: &HeightMap) -> [Triangle; 2] {
    const OFFSET: Float = 1.;
    const VERTEX_OFFSETS: [[Float; 2]; 6] = [
        [0., 0.],         [OFFSET, OFFSET], [0., OFFSET],
        [OFFSET, OFFSET], [0., 0.],         [OFFSET, 0.]
    ];
    let mut triangles = [Triangle::default(),
                         Triangle::default()];
    let resolution: Float = height_map.get_resolution() as Float;

    for i in 0..2 {
        let mut vertices: [Vertex; 3] = [Vertex::default(),
                                         Vertex::default(),
                                         Vertex::default()];
        for (vert, off) in vertices.iter_mut().zip(VERTEX_OFFSETS.iter().skip(i * 3).take(3)) {
                let map_pos = [offset[0] + max(0, min(1, off[0] as i32)),
                               offset[1] + max(0, min(1, off[1] as i32))];
                let height = height_map.get(&map_pos);
                vert.set_pos(Vector3::new((offset[0] as Float + off[0]) * resolution,
                                          (offset[1] as Float + off[1]) * resolution,
                                          height));
                debug_assert!(off[0] <= 1., off[1] <= 1.);
                vert.set_uv(Vector2::new(off[0], off[1]));
        }
        triangles[i] = Triangle::new(vertices);
    }
    triangles.iter_mut().for_each(|t| t.set_uv_layer(1));
    triangles
}