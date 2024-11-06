use super::BigramMetric;
use serde::Deserialize;

use keyboard_layout::{
    key::{Direction, Hand},
    layout::{LayerKey, Layout},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum SameFingerRoll {
    None,    // 0
    Inward,  // 1
    Outward, // 2
}

pub fn get_same_finger_roll(lk1: &LayerKey, lk2: &LayerKey) -> SameFingerRoll {
    let k1 = lk1.key.clone();
    let k2 = lk2.key.clone();
    // should also check same row
    if k1.hand != k2.hand || k1.finger != k2.finger {
        return SameFingerRoll::None;
    }

    let d1 = k1.direction;
    let d2 = k2.direction;
    if d1 == Direction::North && d2 == Direction::East
        || d1 == Direction::East && d2 == Direction::South
        || d1 == Direction::South && d2 == Direction::West
        || d1 == Direction::West && d2 == Direction::North
    {
        if k1.hand == Hand::Left {
            SameFingerRoll::Inward
        } else {
            SameFingerRoll::Outward
        }
    } else if d1 == Direction::North && d2 == Direction::West
        || d1 == Direction::West && d2 == Direction::South
        || d1 == Direction::South && d2 == Direction::East
        || d1 == Direction::East && d2 == Direction::North
    {
        if k1.hand == Hand::Left {
            SameFingerRoll::Outward
        } else {
            SameFingerRoll::Inward
        }
    } else {
        SameFingerRoll::None
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
pub struct SameFingerRolls {
    factor_inward: f64,
    factor_outward: f64,
}

impl SameFingerRolls {
    pub fn new(params: &Parameters) -> Self {
        Self {
            factor_inward: params.factor_inward,
            factor_outward: params.factor_outward,
        }
    }
}

impl BigramMetric for SameFingerRolls {
    fn name(&self) -> &str {
        "Same Finger Rolls"
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
        let direction = get_same_finger_roll(lk1, lk2);
        match direction {
            SameFingerRoll::Inward => Some(self.factor_inward * weight),
            SameFingerRoll::Outward => Some(self.factor_outward * weight),
            SameFingerRoll::None => Some(0.0),
        }
    }
}
