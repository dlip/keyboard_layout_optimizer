use super::BigramMetric;
use serde::Deserialize;

use keyboard_layout::{
    key::Direction,
    layout::{LayerKey, Layout},
};

pub fn is_same_finger_scissor(lk1: &LayerKey, lk2: &LayerKey) -> bool {
    let k1 = lk1.key.clone();
    let k2 = lk2.key.clone();
    // should also check same row
    let d1 = k1.direction;
    let d2 = k2.direction;
    if k1.hand != k2.hand || k1.finger != k2.finger {
        false
    } else {
        d1 == Direction::North && d2 == Direction::South
            || d1 == Direction::South && d2 == Direction::North
            || d1 == Direction::East && d2 == Direction::West
            || d1 == Direction::West && d2 == Direction::East
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct SameFingerScissors {}

impl SameFingerScissors {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl BigramMetric for SameFingerScissors {
    fn name(&self) -> &str {
        "Same Finger Scissors"
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
        if is_same_finger_scissor(lk1, lk2) {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
