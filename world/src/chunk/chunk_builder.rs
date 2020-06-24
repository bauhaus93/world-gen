use std::collections::BTreeSet;
use std::convert::TryFrom;

use glm::{Vector2, Vector3};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use super::{get_world_pos, Chunk, ChunkError, CHUNK_SIZE};
use crate::HeightMap;
use crate::architect::Architect;
use core::graphics::mesh::{Mesh, Triangle, Vertex, VertexBuffer};
use core::traits::{Rotatable, Scalable, Translatable};
use core::{Object, ObjectManager, Point2f, Point2i, Point3f};

pub struct ChunkBuilder {
    pos: Point2i,
    lod: u8,
    height_map: HeightMap,
    surface_vertices: VertexBuffer,
    tree_list: Vec<Object>,
}

impl ChunkBuilder {
    pub fn new(
        pos: Point2i,
        lod: u8,
        architect: &Architect,
        object_manager: &ObjectManager,
        random_state: &[u8; 16],
    ) -> Result<Self, ChunkError> {
        let height_map = match lod {
            0 => architect.create_heightmap(pos, CHUNK_SIZE, 1),
            _ => architect.create_heightmap(pos, CHUNK_SIZE / 8, 8),
        };
        let surface_buffer = create_surface_buffer(pos, architect, &height_map);
        let mut builder = Self {
            pos: pos,
            lod: lod,
            height_map: height_map,
            surface_vertices: surface_buffer,
            tree_list: Vec::new(),
        };

        let mut seed: [u8; 16] = [0; 16];
        seed.copy_from_slice(random_state);
        for i in 0..8 {
            seed[i] += (pos[i / 4] >> (8 * (i % 4))) as u8;
        }
        let mut rng = SmallRng::from_seed(seed);

        builder.load_trees(object_manager, &mut rng)?;
        Ok(builder)
    }

    pub fn finish(self) -> Result<Chunk, ChunkError> {
        let mesh = Mesh::try_from(self.surface_vertices)?;
        let mut chunk = Chunk::new(self.pos, self.height_map, self.lod, mesh);
        self.tree_list.into_iter().for_each(|t| chunk.add_tree(t));
        Ok(chunk)
    }

    fn load_trees(
        &mut self,
        object_manager: &ObjectManager,
        rng: &mut impl Rng,
    ) -> Result<(), ChunkError> {
        if self.lod < 2 {
            let resolution = self.height_map.get_resolution();
            let size = self.height_map.get_size();
            let tree_count = rng.gen_range(2, 20);
            let mut positions: BTreeSet<Point2i> = BTreeSet::default();
            for _ in 0..tree_count {
                positions.insert(Point2i::new(rng.gen_range(0, size), rng.gen_range(0, size)));
            }
            for rel_pos in positions.into_iter() {
                let mut tree = object_manager.create_object("tree")?;
                let abs_pos = (Point2f::from(self.pos * CHUNK_SIZE)
                    + Point2f::from(rel_pos) * resolution as f32)
                    .extend(self.height_map.get(rel_pos));
                tree.set_translation(abs_pos);
                let scale_xy = rng.gen_range(0.8, 1.2);
                let scale_z = rng.gen_range(0.8, 1.4);
                tree.set_scale(Point3f::new(scale_xy, scale_xy, scale_z));
                let orientation = Point3f::new(
                    rng.gen_range(-0.2, 0.2),
                    rng.gen_range(-0.2, 0.2),
                    rng.gen_range(-0.2, 0.2),
                );
                tree.set_rotation(orientation);
                self.tree_list.push(tree);
            }
        }
        Ok(())
    }
}

fn create_surface_buffer(
    origin: Point2i,
    architect: &Architect,
    height_map: &HeightMap,
) -> VertexBuffer {
    let size = height_map.get_size();
    let resolution = height_map.get_resolution();
    let mut triangles: Vec<Triangle> = Vec::with_capacity((size * size * 2) as usize);
    for y in 0..size - 1 {
        for x in 0..size - 1 {
            let rel_pos = Point2i::new(x, y);
            let abs_pos = get_world_pos(origin, Point2f::from(rel_pos * resolution));
            let terrain = architect.get_terrain(abs_pos);
            triangles.extend(&add_quad_triangles(
                &[x, y],
                height_map,
                terrain.get_layer(),
            ));
        }
    }
    VertexBuffer::from(triangles.as_slice())
}

fn add_quad_triangles(
    offset: &[i32; 2],
    height_map: &HeightMap,
    texture_layer: u32,
) -> [Triangle; 2] {
    const OFFSET: f32 = 1.;
    const VERTEX_OFFSETS: [[f32; 2]; 6] = [
        [0., 0.],
        [OFFSET, OFFSET],
        [0., OFFSET],
        [OFFSET, OFFSET],
        [0., 0.],
        [OFFSET, 0.],
    ];
    let mut triangles = [Triangle::default(), Triangle::default()];
    let resolution = height_map.get_resolution() as f32;

    for i in 0..2 {
        let mut vertices: [Vertex; 3] = [Vertex::default(), Vertex::default(), Vertex::default()];
        for (vert, off) in vertices
            .iter_mut()
            .zip(VERTEX_OFFSETS.iter().skip(i * 3).take(3))
        {
            let map_pos = Point2i::new(
                offset[0] + i32::max(0, i32::min(1, off[0] as i32)),
                offset[1] + i32::max(0, i32::min(1, off[1] as i32)),
            );

            let height = height_map.get(map_pos);
            vert.set_pos(Vector3::new(
                (offset[0] as f32 + off[0]) * resolution,
                (offset[1] as f32 + off[1]) * resolution,
                height,
            ));
            debug_assert!(off[0] <= 1., off[1] <= 1.);
            vert.set_uv(Vector2::new(off[0], off[1]));
        }
        triangles[i] = Triangle::new(vertices);
    }
    triangles
        .iter_mut()
        .for_each(|t| t.set_uv_layer(texture_layer));
    triangles
}
