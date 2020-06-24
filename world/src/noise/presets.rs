use rand::rngs::SmallRng;

use super::{ModifierType, Noise, NoiseBuilder};
use core::Seed;

pub fn get_default_noise(seed: Seed) -> Box<dyn Noise> {
    let mut local_rng: SmallRng = seed.into();

    let mountain_factor = NoiseBuilder::new()
        .seed(Seed::from_rng(&mut local_rng))
        .octaves(4)
        .scale(1e-4)
        .roughness(2.)
        .range([-1., 1.])
        .modifier(ModifierType::FactoredExponent(10., 2.))
        .above(0.)
        .finish();

    let _lake_factor = NoiseBuilder::new()
        .seed(Seed::from_rng(&mut local_rng))
        .octaves(4)
        .scale(1e-5)
        .roughness(2.)
        .range([-1., 1.])
        .modifier(ModifierType::FactoredExponent(-100., 0.75))
        .below(0.)
        .finish();

    let height_noise = NoiseBuilder::new()
        .seed(Seed::from_rng(&mut local_rng))
        .octaves(6)
        .scale(1e-3)
        .roughness(0.5)
        .range([0., 100.])
        .add_factor(mountain_factor)
        //     .add_factor(lake_factor)
        .finish();
    height_noise
}
