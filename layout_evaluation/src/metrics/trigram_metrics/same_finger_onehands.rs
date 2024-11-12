use super::{
    same_finger_rolls::{get_same_finger_roll_direction, SameFingerRollDirection},
    TrigramMetric,
};
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
pub struct SameFingerOnehands {
    factor_inward: f64,
    factor_outward: f64,
}

impl SameFingerOnehands {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
        }
    }
}

impl TrigramMetric for SameFingerOnehands {
    fn name(&self) -> &str {
        "Same Finger Onehands"
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

        if !(h1 == h2 && h2 == h3) {
            return Some(0.0);
        }

        let f1 = k1.key.finger;
        let f2 = k2.key.finger;
        let f3 = k3.key.finger;

        // must be using the same finger
        if f1 != f2 || f2 != f3 {
            return Some(0.0);
        }

        let direction1 = get_same_finger_roll_direction(k1, k2);
        let direction2 = get_same_finger_roll_direction(k2, k3);
        if direction1 == SameFingerRollDirection::Inward
            && direction2 == SameFingerRollDirection::Inward
        {
            Some(self.factor_inward * weight)
        } else if direction1 == SameFingerRollDirection::Outward
            && direction2 == SameFingerRollDirection::Outward
        {
            Some(self.factor_outward * weight)
        } else {
            Some(0.0)
        }
    }
}
