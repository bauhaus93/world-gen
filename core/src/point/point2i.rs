use std::cmp::Ordering;

use super::{Point2, Point2f};

pub type Point2i = Point2<i32>;

impl Point2i {
    pub fn length(&self) -> f32 {
        ((self[0] as f32).powf(2.) + (self[1] as f32).powf(2.)).sqrt()
    }
}

impl From<Point2f> for Point2i {
    fn from(p: Point2f) -> Point2i {
        Point2i::new(p[0].round() as i32, p[1].round() as i32)
    }
}

impl Ord for Point2i {
    fn cmp(&self, other: &Self) -> Ordering {
        match self[0].cmp(&other[0]) {
            Ordering::Equal => self[1].cmp(&other[1]),
            order => order,
        }
    }
}

impl PartialOrd for Point2i {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point2i {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Point2i {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2i_new_index() {
        let p = Point2i::new(1, 2);
        assert_eq!(1, p[0]);
        assert_eq!(2, p[1]);
    }

    #[test]
    fn test_point2i_order_first_index() {
        assert_eq!(true, Point2i::new(0, 0) < Point2i::new(1, 0));
    }

    #[test]
    fn test_point2i_order_second_index() {
        assert_eq!(true, Point2i::new(0, 0) < Point2i::new(0, 1));
    }

    #[test]
    fn test_point2i_order_eq() {
        assert_eq!(true, Point2i::new(2, 2) == Point2i::new(2, 2));
    }

    #[test]
    fn test_point2i_order_eq_same() {
        let a = Point2i::new(5, 6);
        assert_eq!(true, a == a);
    }

    #[test]
    fn test_point2i_add_asign() {
        let mut a = Point2i::new(1, 2);
        a += Point2i::new(2, 1);
        assert_eq!(3, a[0]);
        assert_eq!(3, a[1]);
    }
}
