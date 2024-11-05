use super::BigramMetric;
use serde::Deserialize;

use keyboard_layout::{
    key::{Direction, Hand},
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
pub struct DirectionalRolls {
    factor_inward: f64,
    factor_outward: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[repr(u8)]
pub enum RollDirection {
    None,             // 0
    Clockwise,        // 1
    CounterClockwise, // 2
}

impl DirectionalRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
        }
    }
}

impl BigramMetric for DirectionalRolls {
    fn name(&self) -> &str {
        "Directional Rolls"
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
        let k1 = lk1.key.clone();
        let k2 = lk2.key.clone();
        // should also check same row
        if k1.hand != k2.hand || k1.finger != k2.finger {
            return Some(0.0);
        }
        let d1 = k1.direction;
        let d2 = k2.direction;
        let direction: RollDirection = if d1 == Direction::North && d2 == Direction::East
            || d1 == Direction::East && d2 == Direction::South
            || d1 == Direction::South && d2 == Direction::West
            || d1 == Direction::West && d2 == Direction::North
        {
            RollDirection::Clockwise
        } else if d1 == Direction::North && d2 == Direction::West
            || d1 == Direction::West && d2 == Direction::South
            || d1 == Direction::South && d2 == Direction::East
            || d1 == Direction::East && d2 == Direction::North
        {
            RollDirection::CounterClockwise
        } else {
            RollDirection::None
        };

        if k1.hand == Hand::Left && direction == RollDirection::Clockwise
            || k1.hand == Hand::Right && direction == RollDirection::CounterClockwise
        {
            Some(self.factor_inward * weight)
        } else if k1.hand == Hand::Left && direction == RollDirection::CounterClockwise
            || k1.hand == Hand::Right && direction == RollDirection::Clockwise
        {
            Some(self.factor_outward * weight)
        } else {
            Some(0.0)
        }
    }
}
