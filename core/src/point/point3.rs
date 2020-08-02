use glm::{Primitive, Vector3};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use super::Point2;

#[derive(Copy, Clone, Debug)]
pub struct Point3<T: Primitive>(pub Vector3<T>);

impl<T: Primitive> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Vector3::new(x, y, z))
    }

    pub fn from_scalar(scalar: T) -> Point3<T> {
        Point3::new(scalar, scalar, scalar)
    }

    pub fn as_xy(&self) -> Point2<T> {
        Point2::new(self[0], self[1])
    }

    pub fn as_glm(&self) -> Vector3<T> {
        self.0
    }

    pub fn clamp_min(&mut self, min: T) {
        for i in 0..3 {
            if self[i] < min {
                self[i] = min;
            }
        }
    }

    pub fn clamp_max(&mut self, max: T) {
        for i in 0..3 {
            if self[i] > max {
                self[i] = max;
            }
        }
    }
    pub fn clamp_range(&mut self, min: T, max: T) {
        self.clamp_min(min);
        self.clamp_max(max);
    }

    pub fn as_array(&self) -> &[T; 3] {
        self.0.as_array()
    }

    pub fn apply<T2: Primitive + Default, F: (Fn(T) -> T2)>(&self, f: F) -> Point3<T2> {
        let mut res = Point3::<T2>::default();
        for i in 0..3 {
            res[i] = f(self[i])
        }
        res
    }
}

impl<T: Primitive> From<Vector3<T>> for Point3<T> {
    fn from(v: Vector3<T>) -> Point3<T> {
        Self(v)
    }
}

impl<T: Default + Primitive> Default for Point3<T> {
    fn default() -> Self {
        Self::from_scalar(T::default())
    }
}

impl<T: Primitive> Index<usize> for Point3<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T: Primitive> IndexMut<usize> for Point3<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T: Add<Output = T> + Primitive + AddAssign> Add for Point3<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..3 {
            self.0[i] += other.0[i]
        }
        self
    }
}

impl<T: Sub<Output = T> + Primitive + SubAssign> Sub for Point3<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        for i in 0..3 {
            self[i] -= other[i]
        }
        self
    }
}

impl<T: Mul<Output = T> + Primitive + MulAssign> Mul<T> for Point3<T> {
    type Output = Self;
    fn mul(mut self, scalar: T) -> Self {
        for i in 0..3 {
            self.0[i] *= scalar;
        }
        self
    }
}

impl<T: Div<Output = T> + Primitive + DivAssign> Div<T> for Point3<T> {
    type Output = Self;
    fn div(mut self, scalar: T) -> Self {
        for i in 0..3 {
            self.0[i] /= scalar;
        }
        self
    }
}

impl<T: Add<Output = T> + Primitive + AddAssign> AddAssign for Point3<T> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..3 {
            self[i] += other[i];
        }
    }
}

impl<T: Sub<Output = T> + Primitive + SubAssign> SubAssign for Point3<T> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..3 {
            self[i] -= other[i];
        }
    }
}

impl<T: fmt::Display + Primitive> fmt::Display for Point3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self[0], self[1], self[2])
    }
}
