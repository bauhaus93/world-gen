use std::ops::{ Mul, Add };
use num_traits::CheckedSub;

pub fn get_distance_2d_from_zero<T>(point: [T; 2]) -> f64
where T: Into<f64> + Add<Output = T> + Mul<Output = T>  + Copy {
    f64::sqrt((point[0] * point[0] + point[1] * point[1]).into())
}

pub fn get_distance_2d<T>(point_a: [T; 2], point_b: [T; 2]) -> f64
where T: Into<f64> + Add<Output = T> + CheckedSub<Output = T> + Mul<Output = T> + Copy {
    let diff_x = subtract_or_panic(point_a[0], point_b[0]);
    let diff_y = subtract_or_panic(point_a[1], point_b[1]);
    f64::sqrt((diff_x * diff_x + diff_y * diff_y).into())
}

pub fn get_distance_3d_from_zero<T>(point: [T; 3]) -> f64
where T: Into<f64> + Add<Output = T> + Mul<Output = T>  + Copy {
    f64::sqrt((point[0] * point[0] + point[1] * point[1] + point[2] * point[2]).into())
}

pub fn get_distance_3d<T>(point_a: [T; 3], point_b: [T; 3]) -> f64
where T: Into<f64> + Add<Output = T> + CheckedSub<Output = T> + Mul<Output = T> + Copy {
    let diff_x = subtract_or_panic(point_a[0], point_b[0]);
    let diff_y = subtract_or_panic(point_a[1], point_b[1]);
    let diff_z = subtract_or_panic(point_a[2], point_b[2]);
    f64::sqrt((diff_x * diff_x + diff_y * diff_y + diff_z * diff_z).into())
}

pub fn subtract_or_panic<T: CheckedSub<Output = T>>(a: T, b: T) -> T {
    match a.checked_sub(&b) {
        Some(value) => value,
        None => {
            error!("Distance subtraction overflows");
            panic!();
        }
    }
}
