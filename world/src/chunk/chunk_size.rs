use core::{Point2f, Point2i, Point3f};

pub const CHUNK_SIZE: i32 = 32;

pub fn get_chunk_pos(world_pos: Point3f) -> Point2i {
    let mut chunk_pos = Point2i::from_scalar(0);
    for i in 0..2 {
        chunk_pos[i] = world_pos[i].round() as i32 / (CHUNK_SIZE - 1);
    }
    chunk_pos
}

pub fn get_world_pos(chunk_pos: Point2i, offset: Point2f) -> Point2f {
    Point2f::from(chunk_pos * (CHUNK_SIZE - 1)) + offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_zero_zero() {
        let chunk_pos = Point2i::new(0, 0);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, Point2f::from_scalar(0.)).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_pos_pos() {
        let chunk_pos = Point2i::new(1, 1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, Point2f::from_scalar(0.)).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_neg_neg() {
        let chunk_pos = Point2i::new(-1, -1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, Point2f::from_scalar(0.)).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_pos_neg() {
        let chunk_pos = Point2i::new(1, -1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, Point2f::from_scalar(0.)).extend(0.))
        );
    }

    #[test]
    fn test_roundtrip_neg_pos() {
        let chunk_pos = Point2i::new(-1, 1);
        assert_eq!(
            chunk_pos,
            get_chunk_pos(get_world_pos(chunk_pos, Point2f::from_scalar(0.)).extend(0.))
        );
    }
}
