use rand::{
    rngs::{SmallRng, StdRng},
    FromEntropy, Rng, SeedableRng,
};
use std::fmt;

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
