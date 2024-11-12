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
pub struct UnidirectionalOnehands {
    factor_inward: f64,
    factor_outward: f64,
}

impl UnidirectionalOnehands {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
        }
    }
}

#[inline(always)]
fn inwards(k1: &LayerKey, k2: &LayerKey) -> bool {
    if k1.key.hand == Hand::Left {
        k1.key.matrix_position.0 < k2.key.matrix_position.0
    } else {
        k1.key.matrix_position.0 > k2.key.matrix_position.0
    }
}

impl TrigramMetric for UnidirectionalOnehands {
    fn name(&self) -> &str {
        "Unidirectional Onehands"
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

        // same-finger is not a (onehand) unidirectional roll
        if f1 == f2 || f2 == f3 || f1 == f3 {
            return Some(0.0);
        }

        // Check if unidirectional
        if k1.key.direction != k2.key.direction || k2.key.direction != k3.key.direction {
            return Some(0.0);
        }

        let inwards1 = inwards(k1, k2);
        let inwards2 = inwards(k2, k3);

        let outwards1 = inwards(k2, k1);
        let outwards2 = inwards(k3, k2);

        if inwards1 && inwards2 {
            Some(self.factor_inward * weight)
        } else if outwards1 && outwards2 {
            Some(self.factor_outward * weight)
        } else {
            Some(0.0)
        }
    }
}
