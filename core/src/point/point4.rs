use glm::{Primitive, Vector4};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Point4<T: Primitive>(pub Vector4<T>);

impl<T: Primitive> Point4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(Vector4::new(x, y, z, w))
    }

    pub fn from_scalar(scalar: T) -> Point4<T> {
        Point4::new(scalar, scalar, scalar, scalar)
    }

    pub fn as_glm(&self) -> Vector4<T> {
        self.0
    }

    pub fn as_array(&self) -> &[T; 4] {
        self.0.as_array()
    }
}

impl<T: Primitive> From<Vector4<T>> for Point4<T> {
    fn from(v: Vector4<T>) -> Point4<T> {
        Self(v)
    }
}

impl<T: Default + Primitive> Default for Point4<T> {
    fn default() -> Self {
        Self::from_scalar(T::default())
    }
}

impl<T: Primitive> Index<usize> for Point4<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T: Primitive> IndexMut<usize> for Point4<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T: Add<Output = T> + Primitive + AddAssign> Add for Point4<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..4 {
            self.0[i] += other.0[i]
        }
        self
    }
}

impl<T: Sub<Output = T> + Primitive + SubAssign> Sub for Point4<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        for i in 0..4 {
            self[i] -= other[i]
        }
        self
    }
}

impl<T: Mul<Output = T> + Primitive + MulAssign> Mul<T> for Point4<T> {
    type Output = Self;
    fn mul(mut self, scalar: T) -> Self {
        for i in 0..4 {
            self.0[i] *= scalar;
        }
        self
    }
}

impl<T: Div<Output = T> + Primitive + DivAssign> Div<T> for Point4<T> {
    type Output = Self;
    fn div(mut self, scalar: T) -> Self {
        for i in 0..4 {
            self.0[i] /= scalar;
        }
        self
    }
}

impl<T: Add<Output = T> + Primitive + AddAssign> AddAssign for Point4<T> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..4 {
            self[i] += other[i];
        }
    }
}

impl<T: Sub<Output = T> + Primitive + SubAssign> SubAssign for Point4<T> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..4 {
            self[i] -= other[i];
        }
    }
}

impl<T: fmt::Display + Primitive> fmt::Display for Point4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}/{}", self[0], self[1], self[2], self[3])
    }
}
