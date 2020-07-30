use std::convert::TryFrom;

use super::{get_world_pos, Chunk, ChunkError, CHUNK_SIZE};
use crate::architect::Architect;
use crate::HeightMap;
use core::graphics::mesh::{Mesh, Triangle, Vertex, VertexBuffer};
use core::{Point2f, Point2i, Point3f};

pub struct ChunkBuilder {
    pos: Point2i,
    lod: u8,
    height_map: HeightMap,
    surface_vertices: VertexBuffer,
}

impl ChunkBuilder {
    pub fn new(pos: Point2i, lod: u8, architect: &Architect) -> Result<Self, ChunkError> {
        let resolution = match lod {
            0 => 1.,
            _ => 8.,
        };
        let height_map =
            architect.create_heightmap(pos, (CHUNK_SIZE as f32 / resolution) as i32, resolution);
        let surface_buffer = create_surface_buffer(pos, &height_map, architect, resolution);
        let builder = Self {
            pos: pos,
            lod: lod,
            height_map: height_map,
            surface_vertices: surface_buffer,
        };
        Ok(builder)
    }

    pub fn finish(self) -> Result<Chunk, ChunkError> {
        let mesh = Mesh::try_from(self.surface_vertices)?;
        let chunk = Chunk::new(self.pos, self.height_map, self.lod, mesh);
        Ok(chunk)
    }
}

fn create_surface_buffer(
    origin: Point2i,
    height_map: &HeightMap,
    architect: &Architect,
    resolution: f32,
) -> VertexBuffer {
    let size = height_map.get_size();
    let resolution = height_map.get_resolution();
    let mut triangles: Vec<Triangle> = Vec::with_capacity((size * size * 2) as usize);
    for y in 0..size - 1 {
        for x in 0..size - 1 {
            let rel_pos = Point2i::new(x, y);
            let abs_pos = get_world_pos(origin, Point2f::from(rel_pos) * resolution);
            let terrain = architect.get_terrain(abs_pos);
            triangles.extend(&add_quad_triangles(
                rel_pos,
                origin,
                height_map,
                architect,
                resolution,
                terrain.get_layer(),
            ));
        }
    }
    VertexBuffer::from(triangles.as_slice())
}

fn add_quad_triangles(
    offset: Point2i,
    origin: Point2i,
    height_map: &HeightMap,
    architect: &Architect,
    resolution: f32,
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
    let resolution = height_map.get_resolution();

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
            vert.set_pos(Point3f::new(
                (offset[0] as f32 + off[0]) * resolution,
                (offset[1] as f32 + off[1]) * resolution,
                height,
            ));
            debug_assert!(off[0] <= 1., off[1] <= 1.);
            vert.set_uv(Point2f::new(off[0], off[1]));

            let r = height_map.get(offset + Point2i::new(1, 0));
            let (l, t) = match map_pos {
                mp if mp[0] == 0 && mp[1] == 0 => (
                    architect.get_height(get_world_pos(origin, Point2f::new(-resolution, 0.))),
                    architect.get_height(get_world_pos(origin, Point2f::new(0., -resolution))),
                ),
                mp if mp[0] == 0 => (
                    architect.get_height(get_world_pos(
                        origin,
                        Point2f::new(-1., mp[1] as f32) * resolution,
                    )),
                    height_map.get(mp + Point2i::new(0, -1)),
                ),
                mp if mp[1] == 0 => (
                    height_map.get(mp + Point2i::new(-1, 0)),
                    architect.get_height(get_world_pos(
                        origin,
                        Point2f::new(mp[0] as f32, -1.) * resolution,
                    )),
                ),
                mp => (
                    height_map.get(mp + Point2i::new(-1, 0)),
                    height_map.get(mp + Point2i::new(0, -1)),
                ),
            };
            let b = height_map.get(offset + Point2i::new(0, 1));

            let normal = Point3f::new((r - l) / 2., (b - t) / 2., 1.).as_normalized();
            vert.set_normal(normal);
        }
        triangles[i] = Triangle::new(vertices);
    }
    triangles
        .iter_mut()
        .for_each(|t| { t.set_uv_layer(texture_layer); t.update_normals();});
    triangles
}
