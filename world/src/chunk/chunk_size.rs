use core::{Point2f, Point2i, Point3f};

pub const CHUNK_SIZE: i32 = 64;

pub fn get_chunk_pos(world_pos: Point3f) -> Point2i {
    let mut chunk_pos = Point2i::from_scalar(0);
    for i in 0..2 {
        chunk_pos[i] = world_pos[i].round() as i32 / CHUNK_SIZE;
        if world_pos[i] < 0. {
            chunk_pos[i] -= 1;
        }
    }
    chunk_pos
}

#[allow(unused)]
pub fn get_chunk_relative_pos(chunk_pos: Point3f, world_pos: Point3f, resolution: i32) -> Point2i {
    Point2i::from(world_pos.as_xy()) / resolution
}

pub fn get_world_pos(chunk_pos: Point2i, offset: Point2f) -> Point2f {
    Point2f::from(chunk_pos * CHUNK_SIZE) + offset
}
