use crate::metrics::bigram_metrics::directional_scissors::is_directional_scissor;

use super::TrigramMetric;
use serde::Deserialize;

use keyboard_layout::layout::{LayerKey, Layout};

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct TrigramDirectionalScissors {}

impl TrigramDirectionalScissors {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl TrigramMetric for TrigramDirectionalScissors {
    fn name(&self) -> &str {
        "Trigram Directional Scissors"
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
        if is_directional_scissor(lk1, lk2) || is_directional_scissor(lk2, lk3) {
            Some(weight)
        } else {
            Some(0.0)
        }
    }
}
