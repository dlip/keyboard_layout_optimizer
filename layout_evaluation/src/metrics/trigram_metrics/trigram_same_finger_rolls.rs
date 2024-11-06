use crate::metrics::bigram_metrics::same_finger_rolls::{get_same_finger_roll, SameFingerRoll};

use super::TrigramMetric;
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
pub struct TrigramSameFingerRolls {
    factor_inward: f64,
    factor_outward: f64,
}

impl TrigramSameFingerRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
        }
    }
}

impl TrigramMetric for TrigramSameFingerRolls {
    fn name(&self) -> &str {
        "Trigram Same Finger Rolls"
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
        let direction1 = get_same_finger_roll(lk1, lk2);
        let direction2 = get_same_finger_roll(lk2, lk3);
        if direction1 == SameFingerRoll::Inward && direction2 == SameFingerRoll::Inward {
            Some(self.factor_inward * weight)
        } else if direction1 == SameFingerRoll::Outward && direction2 == SameFingerRoll::Outward {
            Some(self.factor_outward * weight)
        } else {
            Some(0.0)
        }
    }
}
