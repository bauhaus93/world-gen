use crate::noise::Noise;
use core::Point2f;

pub struct FactoredNoise {
    base_noise: Box<dyn Noise>,
    factor_noises: Vec<Box<dyn Noise>>,
    merge_type: MergeType,
}

pub enum MergeType {
    SUM,
    PRODUCT,
    AVG,
}

impl FactoredNoise {
    pub fn new(base_noise: Box<dyn Noise>, merge_type: MergeType) -> Self {
        Self {
            base_noise: base_noise,
            factor_noises: Vec::new(),
            merge_type: merge_type,
        }
    }

    pub fn add_factor(&mut self, factor_noise: Box<dyn Noise>) {
        self.factor_noises.push(factor_noise);
    }

    fn calculate_factor(&self, point: Point2f) -> f32 {
        1. + match self.merge_type {
            MergeType::SUM => self
                .factor_noises
                .iter()
                .fold(0., |acc, n| acc + n.get_noise(point)),
            MergeType::PRODUCT => self
                .factor_noises
                .iter()
                .fold(0., |acc, n| acc * n.get_noise(point)),
            MergeType::AVG => {
                self.factor_noises
                    .iter()
                    .fold(0., |acc, n| acc + n.get_noise(point))
                    / self.factor_noises.len() as f32
            }
        }
    }

    fn get_min_factor(&self) -> f32 {
        1. + match self.merge_type {
            MergeType::SUM => self
                .factor_noises
                .iter()
                .fold(0., |acc, n| acc + n.get_range()[0]),
            MergeType::PRODUCT => self
                .factor_noises
                .iter()
                .fold(0., |acc, n| acc * n.get_range()[0]),
            MergeType::AVG => {
                self.factor_noises
                    .iter()
                    .fold(0., |acc, n| acc * n.get_range()[0])
                    / self.factor_noises.len() as f32
            }
        }
    }
    fn get_max_factor(&self) -> f32 {
        1. + match self.merge_type {
            MergeType::SUM => self
                .factor_noises
                .iter()
                .fold(0., |acc, n| acc + n.get_range()[1]),
            MergeType::PRODUCT => self
                .factor_noises
                .iter()
                .fold(0., |acc, n| acc * n.get_range()[1]),
            MergeType::AVG => {
                self.factor_noises
                    .iter()
                    .fold(0., |acc, n| acc * n.get_range()[1])
                    / self.factor_noises.len() as f32
            }
        }
    }
}

impl Noise for FactoredNoise {
    fn get_noise(&self, point: Point2f) -> f32 {
        let bn = self.base_noise.get_noise(point);
        bn * self.calculate_factor(point)
    }

    fn get_range(&self) -> [f32; 2] {
        [
            self.base_noise.get_range()[0] * self.get_min_factor(),
            self.base_noise.get_range()[1] * self.get_max_factor(),
        ]
    }

    fn get_cycle(&self) -> Point2f {
        self.base_noise.get_cycle()
    }
}
