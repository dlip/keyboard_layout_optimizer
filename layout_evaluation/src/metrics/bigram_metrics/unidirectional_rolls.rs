use super::BigramMetric;
use serde::Deserialize;

use keyboard_layout::{
    key::Hand,
    layout::{LayerKey, Layout},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum DirectionalOnehand {
    None,    // 0
    Inward,  // 1
    Outward, // 2
}

pub fn get_unidirectional_roll(lk1: &LayerKey, lk2: &LayerKey) -> DirectionalOnehand {
    let k1 = lk1.key.clone();
    let k2 = lk2.key.clone();
    // should also check same row
    if k1.hand != k2.hand || k1.finger == k2.finger || k1.direction != k2.direction {
        return DirectionalOnehand::None;
    }

    let inwards: bool = if k1.hand == Hand::Left {
        k1.matrix_position.0 < k2.matrix_position.0
    } else {
        k1.matrix_position.0 > k2.matrix_position.0
    };

    if inwards {
        DirectionalOnehand::Inward
    } else {
        DirectionalOnehand::Outward
    }
}

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

impl BigramMetric for UnidirectionalRolls {
    fn name(&self) -> &str {
        "Unidirectional Rolls"
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
        let direction = get_unidirectional_roll(lk1, lk2);
        match direction {
            DirectionalOnehand::Inward => Some(self.factor_inward * weight),
            DirectionalOnehand::Outward => Some(self.factor_outward * weight),
            DirectionalOnehand::None => Some(0.0),
        }
    }
}
