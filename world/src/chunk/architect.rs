use super::get_world_pos;
use super::height_map::HeightMap;
use crate::Terrain;
use core::{Point2f, Point2i};

pub trait Architect: Send + Sync {
    fn get_height(&self, absolute_pos: Point2f) -> f32;
    fn get_terrain(&self, absolute_pos: Point2f) -> &Terrain;

    fn create_heightmap(&self, chunk_pos: Point2i, chunk_size: i32, resolution: i32) -> HeightMap {
        let size = chunk_size + 1;
        let mut height_map = HeightMap::new(size, resolution);
        for y in 0..size {
            for x in 0..size {
                let rel_pos = Point2i::new(x, y);
                let abs_pos = get_world_pos(chunk_pos, Point2f::from(rel_pos * resolution));
                height_map.set(rel_pos, self.get_height(abs_pos));
            }
        }
        height_map
    }
}
