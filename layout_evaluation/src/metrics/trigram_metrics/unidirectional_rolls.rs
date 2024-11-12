use super::TrigramMetric;
use serde::Deserialize;

use keyboard_layout::{
    key::Hand,
    layout::{LayerKey, Layout},
};

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    /// Factor to apply to a trigram's weight if the roll is going inwards
    pub factor_inward: f64,
    /// Factor to apply to a trigram's weight if the roll is going outwards
    pub factor_outward: f64,
}

#[derive(Clone, Debug)]
pub struct UnidirectionalRolls {
    factor_inward: f64,
    factor_outward: f64,
}

impl UnidirectionalRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
        }
    }
}

impl TrigramMetric for UnidirectionalRolls {
    fn name(&self) -> &str {
        "Unidirectional Rolls"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        let h1 = k1.key.hand;
        let h2 = k2.key.hand;
        let h3 = k3.key.hand;

        let first_roll = h1 == h2 && h2 != h3;
        let second_roll = h1 != h2 && h2 == h3;

        if !(first_roll || second_roll) {
            return Some(0.0);
        }

        let (kr1, kr2) = if first_roll { (k1, k2) } else { (k2, k3) };

        // same-finger is not a roll
        if kr1.key.finger == kr2.key.finger {
            return Some(0.0);
        }

        // Check if unidirectional
        if kr1.key.direction != kr2.key.direction {
            return Some(0.0);
        }

        let inwards: bool = if kr1.key.hand == Hand::Left {
            kr1.key.matrix_position.0 < kr2.key.matrix_position.0
        } else {
            kr1.key.matrix_position.0 > kr2.key.matrix_position.0
        };

        if inwards {
            Some(self.factor_inward * weight)
        } else {
            Some(self.factor_outward * weight)
        }
    }
}
