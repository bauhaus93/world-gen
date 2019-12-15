use rand::rngs::SmallRng;
use rand::FromEntropy;

use super::State;
use crate::HeightMap;

// http://www-ljk.imag.fr/Publications/Basilic/com.lmc.publi.PUBLI_Inproceedings@117681e94b6_fff75c/FastErosion_PG07.pdf

pub struct HydraulicErosion {
    rng: SmallRng,
	state: State
}

impl HydraulicErosion {
	pub fn get_state(&self) -> &State {
		&self.state
	}
	pub fn rain(&mut self, total_water: f64, drop_count: u32) {
		self.state = self.state.rain(total_water, drop_count, &mut self.rng);
	}

    pub fn simulate(&mut self, count: usize) {
		for _ in 0..count {
			self.tick();
		}
    }

	fn tick(&mut self) {
        let mut next_state = self.state.clone();
		next_state.age_increment();
        next_state.calculate_flow(&self.state);
		next_state.apply_flow();
		next_state.calculate_velocity(&self.state);
		next_state.calculate_normals(&self.state);
		next_state.calculate_transport_capacity(&self.state);
		next_state.apply_erosion_deposition();
		next_state.apply_sediment_transportation();
		self.state = next_state;
	}
}

impl Into<HeightMap> for HydraulicErosion {
	fn into(self) -> HeightMap {
		self.state.into()
	}
}

impl From<HeightMap> for HydraulicErosion {
	fn from(height_map: HeightMap) -> HydraulicErosion {
		Self {
			rng: SmallRng::from_entropy(),	// TODO: use seed from builder
			state: height_map.into()
		}
	}
}

	

    
