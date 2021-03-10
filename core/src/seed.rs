use crate::Point2i;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Seed([u8; 32]);

impl Seed {
    pub fn from_entropy() -> Self {
        Self::from_rng(&mut StdRng::from_entropy())
    }

    pub fn from_rng(rng: &mut impl Rng) -> Self {
        let mut seed = [0; 32];
        rng.fill_bytes(&mut seed);
        Self(seed)
    }

    pub fn from_string(string: &str) -> Self {
        let mut seed = [0; 32];
        for (s, c) in seed.iter_mut().zip(string.bytes()) {
            *s = c;
        }
        Self(seed)
    }

    pub fn from_byte_string(byte_string: &str) -> Option<Self> {
        let mut seed = [0; 32];
        if byte_string.len() != 32 || !byte_string.chars().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        let bytes: Vec<u8> = byte_string
            .chars()
            .step_by(2)
            .zip(byte_string.chars().skip(1).step_by(2))
            .map(|(a, b)| (a.to_digit(16).unwrap() * 16 + b.to_digit(16).unwrap()) as u8)
            .collect();
        for (s, c) in seed.iter_mut().zip(bytes.into_iter()) {
            *s = c;
        }
        Some(Self(seed))
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

impl Into<StdRng> for Seed {
    fn into(self) -> StdRng {
        StdRng::from_seed(self.0)
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
