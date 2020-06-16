use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Index, Sub};

pub struct Point2<T>(pub [T; 2]);
pub struct Point3<T>(pub [T; 3]);

type Point2i = Point2<i32>;
type Point3i = Point3<i32>;
type Point3f = Point3<f32>;

const EPSILON: f32 = 1e-6;

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }
}

impl<T: Default> Default for Point2<T> {
    fn default() -> Self {
        Point2([T::default(), T::default()])
    }
}

impl<T> Index<usize> for Point2<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T: Add<Output = T> + Copy + Clone> Add for Point2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self([self[0] + other[0], self[1] + other[1]])
    }
}

impl<T: Add<Output = T> + Copy + Clone> AddAssign for Point2<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self([self[0] + other[0], self[1] + other[1]])
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

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}

impl<T> Index<usize> for Point3<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl Ord for Point3i {
    fn cmp(&self, other: &Self) -> Ordering {
        match self[0].cmp(&other[0]) {
            Ordering::Equal => match self[1].cmp(&other[1]) {
                Ordering::Equal => self[2].cmp(&other[2]),
                order => order,
            },
            order => order,
        }
    }
}

impl PartialOrd for Point3i {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point3i {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Point3i {}

impl Ord for Point3f {
    fn cmp(&self, other: &Self) -> Ordering {
        match self[0] - other[0] {
            diff if diff >= EPSILON => Ordering::Greater,
            diff if diff <= -EPSILON => Ordering::Less,
            _ => match self[1] - other[1] {
                diff if diff >= EPSILON => Ordering::Greater,
                diff if diff <= -EPSILON => Ordering::Less,
                _ => match self[2] - other[2] {
                    diff if diff >= EPSILON => Ordering::Greater,
                    diff if diff <= -EPSILON => Ordering::Less,
                    _ => Ordering::Equal,
                },
            },
        }
    }
}

impl PartialOrd for Point3f {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point3f {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Point3f {}

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

    #[test]
    fn test_point3f_eq() {
        assert_eq!(true, Point3f::new(3.9, 1.8, 2.7) == Point3f::new(3.9, 1.8, 2.7));
    }

    #[test]
    fn test_point3f_first_less() {
        assert_eq!(true, Point3f::new(3.9, 1.8, 2.6) < Point3f::new(3.9 + 2. * EPSILON, 1.8, 2.6));
    }

    #[test]
    fn test_point3f_second_less() {
        assert_eq!(true, Point3f::new(3.9, 1.8, 2.6) < Point3f::new(3.9, 1.8 + 2. * EPSILON, 2.6));
    }

    #[test]
    fn test_point3f_third_less() {
        assert_eq!(true, Point3f::new(3.9, 1.8, 2.6) < Point3f::new(3.9, 1.8, 2.6 + 2. * EPSILON));
    }
}
