use crate::metrics::bigram_metrics::same_finger_scissors::is_same_finger_scissor;

use super::TrigramMetric;
use serde::Deserialize;

use keyboard_layout::layout::{LayerKey, Layout};

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct TrigramSameFingerScissors {}

impl TrigramSameFingerScissors {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl TrigramMetric for TrigramSameFingerScissors {
    fn name(&self) -> &str {
        "Trigram Same Finger Scissors"
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
        if is_same_finger_scissor(lk1, lk2) || is_same_finger_scissor(lk2, lk3) {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
