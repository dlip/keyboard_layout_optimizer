use crate::metrics::directional_roll::DirectionalRoll;

use super::TrigramMetric;
use serde::Deserialize;

use keyboard_layout::layout::{LayerKey, Layout};

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Factor to apply to a trigram's weight if the roll is going inwards
    pub factor_inward: f64,
    /// Factor to apply to a trigram's weight if the roll is going outwards
    pub factor_outward: f64,
    /// Factor to apply to a trigram's weight if the roll is going between inwards and outwards
    pub factor_pingpong: f64,
}

#[derive(Clone, Debug)]
pub struct TrigramDirectionalRolls {
    factor_inward: f64,
    factor_outward: f64,
    factor_pingpong: f64,
}

impl TrigramDirectionalRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
            factor_pingpong: params.factor_pingpong,
        }
    }
}

impl TrigramMetric for TrigramDirectionalRolls {
    fn name(&self) -> &str {
        "Trigram Directional Rolls"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        lk1: &LayerKey,
        lk2: &LayerKey,
        lk3: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        let direction1 = DirectionalRoll::new(lk1, lk2);
        let direction2 = DirectionalRoll::new(lk2, lk3);
        if direction1 == DirectionalRoll::Inward && direction2 == DirectionalRoll::Inward {
            Some(self.factor_inward * weight)
        } else if direction1 == DirectionalRoll::Outward && direction2 == DirectionalRoll::Outward {
            Some(self.factor_outward * weight)
        } else if direction1 == DirectionalRoll::Outward && direction2 == DirectionalRoll::Inward
            || direction1 == DirectionalRoll::Inward && direction2 == DirectionalRoll::Outward
        {
            Some(self.factor_pingpong * weight)
        } else {
            Some(0.0)
        }
    }
}
