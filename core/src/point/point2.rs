use glm::{Primitive, Vector2};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use super::Point3;

#[derive(Copy, Clone)]
pub struct Point2<T: Primitive>(pub Vector2<T>);

impl<T: Primitive> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(Vector2::new(x, y))
    }

    pub fn from_scalar(scalar: T) -> Point2<T> {
        Point2::new(scalar, scalar)
    }

    pub fn extend(&self, z: T) -> Point3<T> {
        Point3::new(self[0], self[1], z)
    }

    pub fn apply<T2: Primitive + Default, F: (Fn(T) -> T2)>(&self, f: F) -> Point2<T2> {
        let mut res = Point2::<T2>::default();
        for i in 0..2 {
            res[i] = f(self[i])
        }
        res
    }
}

impl<T: Primitive> From<Vector2<T>> for Point2<T> {
    fn from(v: Vector2<T>) -> Point2<T> {
        Self(v)
    }
}

impl<T: Default + Primitive> Default for Point2<T> {
    fn default() -> Self {
        Self::from_scalar(T::default())
    }
}

impl<T: Primitive> Index<usize> for Point2<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T: Primitive> IndexMut<usize> for Point2<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T: Add<Output = T> + Primitive + AddAssign> Add for Point2<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..2 {
            self[i] += other[i];
        }
        self
    }
}

impl<T: Sub<Output = T> + Primitive + SubAssign> Sub for Point2<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        for i in 0..2 {
            self[i] -= other[i];
        }
        self
    }
}

impl<T: Mul<Output = T> + Primitive + MulAssign> Mul for Point2<T> {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self::Output {
        for i in 0..2 {
            self[i] *= other[i];
        }
        self
    }
}

impl<T: Div<Output = T> + Primitive + DivAssign> Div for Point2<T> {
    type Output = Self;

    fn div(mut self, other: Self) -> Self::Output {
        for i in 0..2 {
            self[i] /= other[i];
        }
        self
    }
}

impl<T: Div<Output = T> + Primitive + DivAssign> Div<T> for Point2<T> {
    type Output = Self;

    fn div(mut self, scalar: T) -> Self::Output {
        for i in 0..2 {
            self[i] /= scalar;
        }
        self
    }
}

impl<T: Mul<Output = T> + Primitive + MulAssign> Mul<T> for Point2<T> {
    type Output = Self;
    fn mul(mut self, scalar: T) -> Self {
        for i in 0..2 {
            self[i] *= scalar;
        }
        self
    }
}

impl<T: Add<Output = T> + Primitive + AddAssign> AddAssign for Point2<T> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..2 {
            self[i] += other[i];
        }
    }
}

impl<T: fmt::Display + Primitive> fmt::Display for Point2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self[0], self[1])
    }
}
