use super::BigramMetric;
use serde::Deserialize;

use keyboard_layout::{
    key::Direction,
    layout::{LayerKey, Layout},
};

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
        k1: &LayerKey,
        k2: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        if k1.key.hand != k2.key.hand || k1.key.finger != k2.key.finger || k1.key.position.1 != k2.key.position.1 {
            return Some(0.0);
        }

        let d1 = k1.key.direction;
        let d2 = k2.key.direction;

        if (d1 == Direction::North && d2 == Direction::South)
            || (d1 == Direction::South && d2 == Direction::North)
            || (d1 == Direction::East && d2 == Direction::West)
            || (d1 == Direction::West && d2 == Direction::East)
        {
            return Some(weight);
        }

        Some(0.0)
    }
}
