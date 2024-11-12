use super::BigramMetric;
use serde::Deserialize;

use keyboard_layout::layout::{LayerKey, Layout};

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct DirectionalChange {}

impl DirectionalChange {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl BigramMetric for DirectionalChange {
    fn name(&self) -> &str {
        "Directional Change"
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
        if k1.key.hand != k2.key.hand || k1.key.finger == k2.key.finger {
            return Some(0.0);
        }

        if k1.key.direction != k2.key.direction {
            return Some(weight);
        }

        Some(0.0)
    }
}
