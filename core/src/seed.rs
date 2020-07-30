use rand::{
    rngs::{SmallRng, StdRng},
    FromEntropy, Rng, SeedableRng,
};
use std::fmt;
use crate::Point2i;

#[derive(Clone, Copy)]
pub struct Seed([u8; 16]);

impl Seed {
    pub fn from_entropy() -> Self {
        Self::from_rng(&mut StdRng::from_entropy())
    }

    pub fn from_rng(rng: &mut impl Rng) -> Self {
        let mut seed = [0; 16];
        rng.fill_bytes(&mut seed);
        Self(seed)
    }

    pub fn mix_with_point(&self, point: Point2i) -> Self {
        let mut seed = self.0.clone();
        for (point_byte, seed_byte) in point[0].to_le_bytes().iter().zip(seed.iter_mut()) {
            *seed_byte = *seed_byte ^ *point_byte;
        }
        for (point_byte, seed_byte) in point[1].to_le_bytes().iter().zip(seed.iter_mut().skip(4)) {
            *seed_byte = *seed_byte ^ *point_byte;
        }
        Self(seed)
    }
}

impl Into<SmallRng> for Seed {
    fn into(self) -> SmallRng {
        SmallRng::from_seed(self.0)
    }
}

impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .fold(String::new(), |acc, b| acc + &format!("{:02X}", b))
        )
    }
}
