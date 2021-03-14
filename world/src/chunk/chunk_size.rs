use core::{Point2f, Point2i, Point3f};

pub const CHUNK_SIZE: i32 = 64;

// TODO: CHECK PROPER CONVERSION ON NEGATIVE POINTS (for world_pos)

pub fn get_chunk_pos(world_pos: Point3f) -> Point2i {
    let mut chunk_pos = Point2i::from(world_pos.as_xy()) / (CHUNK_SIZE - 1);
    for i in 0..2 {
        if world_pos[i] < 0. {
            chunk_pos[i] -= 1;
        }
    }
    chunk_pos
}

pub fn get_relative_pos(world_pos: Point3f) -> Point2f {
    let chunk_pos = get_chunk_pos(world_pos);
    world_pos.as_xy() - get_world_pos(chunk_pos, None)
}

pub fn get_world_pos(chunk_pos: Point2i, offset: Option<Point2f>) -> Point2f {
    Point2f::from(chunk_pos * (CHUNK_SIZE - 1))
        + match offset {
            Some(off) => off,
            None => Point2f::from_scalar(0.),
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_world_pos_to_chunk_pos() {
        assert_eq!(
            Point2i::new(-1, -1),
            get_chunk_pos(Point3f::new(-1., -1., 0.))
        );
    }

    #[test]
    fn test_negative_chunk_pos_to_world_pos() {
        assert_eq!(
            Point2f::from_scalar(-(CHUNK_SIZE - 1) as f32),
            get_world_pos(Point2i::new(-1, -1), None)
        );
    }

    #[test]
    fn test_roundtrip_zero_zero() {
        let chunk_pos = Point2i::new(0, 0);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, None).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_pos_pos() {
        let chunk_pos = Point2i::new(1, 1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, None).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_neg_neg() {
        let chunk_pos = Point2i::new(-1, -1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, None).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_pos_neg() {
        let chunk_pos = Point2i::new(1, -1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, None).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_neg_pos() {
        let chunk_pos = Point2i::new(-1, 1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, None).extend(0.))
        );
    }
}
