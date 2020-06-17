use std::cmp::Ordering;

use super::Point3;

pub type Point3i = Point3<i32>;

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
