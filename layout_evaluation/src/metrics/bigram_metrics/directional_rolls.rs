use crate::metrics::directional_roll::DirectionalRoll;

use super::BigramMetric;
use serde::Deserialize;

use keyboard_layout::layout::{LayerKey, Layout};

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Factor to apply to a trigram's weight if the roll is going inwards
    pub factor_inward: f64,
    /// Factor to apply to a trigram's weight if the roll is going outwards
    pub factor_outward: f64,
}

#[derive(Clone, Debug)]
pub struct DirectionalRolls {
    factor_inward: f64,
    factor_outward: f64,
}

impl DirectionalRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
        }
    }
}

impl BigramMetric for DirectionalRolls {
    fn name(&self) -> &str {
        "Directional Rolls"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        lk1: &LayerKey,
        lk2: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        let direction = DirectionalRoll::new(lk1, lk2);
        match direction {
            DirectionalRoll::Inward => Some(self.factor_inward * weight),
            DirectionalRoll::Outward => Some(self.factor_outward * weight),
            DirectionalRoll::None => Some(0.0),
        }
    }
}
