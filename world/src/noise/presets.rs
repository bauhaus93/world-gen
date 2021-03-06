use rand::rngs::StdRng;

use super::{ModifierType, Noise, NoiseBuilder};
use core::Seed;

pub fn get_default_noise(seed: Seed) -> Box<dyn Noise> {
    let mut local_rng: StdRng = seed.into();

    let mountain_factor = NoiseBuilder::new()
        .seed(Seed::from_rng(&mut local_rng))
        .octaves(6)
        .scale(1e-3)
        .roughness(0.3)
        .range([-1., 1.])
        .modifier(ModifierType::FactoredExponent(10., 2.))
        .above(0.)
        .finish();

    /*let mountain_factor = NoiseBuilder::new()
    .seed(Seed::from_rng(&mut local_rng))
    .base_worley()
    .octaves(1)
    .scale(6e-2)
    .roughness(0.5)
    .range([-2., 4.])
    .modifier(ModifierType::FactoredExponent(2., 2.))
    .above(0.)
    .finish();*/

    let _lake_factor = NoiseBuilder::new()
        .seed(Seed::from_rng(&mut local_rng))
        .octaves(3)
        .scale(5e-4)
        .roughness(0.5)
        .range([-1., 1.0])
        .modifier(ModifierType::FactoredExponent(10., 3.))
        .below(0.)
        .finish();

    let height_noise = NoiseBuilder::new()
        .seed(Seed::from_rng(&mut local_rng))
        .octaves(6)
        .scale(1e-3)
        .roughness(0.5)
        .range([-20., 100.])
        .add_factor(mountain_factor)
        .add_factor(_lake_factor)
        .finish();
    height_noise
}

pub fn get_default_tree_noise(seed: Seed) -> Box<dyn Noise> {
    NoiseBuilder::new()
        .seed(seed)
        .octaves(3)
        .scale(5e-2)
        .roughness(0.5)
        .range([-1.5, 1.])
        .finish()
}
