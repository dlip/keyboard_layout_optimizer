use super::TrigramMetric;
use serde::Deserialize;

use keyboard_layout::{
    key::{Direction, Hand},
    layout::{LayerKey, Layout},
};
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum SameFingerRollDirection {
    None,    // 0
    Inward,  // 1
    Outward, // 2
}

pub fn get_same_finger_roll_direction(lk1: &LayerKey, lk2: &LayerKey) -> SameFingerRollDirection {
    let d1 = lk1.key.direction;
    let d2 = lk2.key.direction;
    if d1 == Direction::North && d2 == Direction::East
        || d1 == Direction::East && d2 == Direction::South
        || d1 == Direction::South && d2 == Direction::West
        || d1 == Direction::West && d2 == Direction::North
    {
        if lk1.key.hand == Hand::Left {
            SameFingerRollDirection::Inward
        } else {
            SameFingerRollDirection::Outward
        }
    } else if d1 == Direction::North && d2 == Direction::West
        || d1 == Direction::West && d2 == Direction::South
        || d1 == Direction::South && d2 == Direction::East
        || d1 == Direction::East && d2 == Direction::North
    {
        if lk1.key.hand == Hand::Left {
            SameFingerRollDirection::Outward
        } else {
            SameFingerRollDirection::Inward
        }
    } else {
        SameFingerRollDirection::None
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

impl TrigramMetric for SameFingerRolls {
    fn name(&self) -> &str {
        "Same Finger Rolls"
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

        let first_roll = h1 == h2 && h2 != h3;
        let second_roll = h1 != h2 && h2 == h3;

        if !(first_roll || second_roll) {
            return Some(0.0);
        }

        let (kr1, kr2) = if first_roll { (k1, k2) } else { (k2, k3) };
        // must be using the same finger
        if kr1.key.finger != kr2.key.finger {
            return Some(0.0);
        }
        // must be the same directional switch (row)
        if kr1.key.position.1 != kr2.key.position.1 {
            return Some(0.0);
         }

        let direction = get_same_finger_roll_direction(kr1, kr2);

        if direction == SameFingerRollDirection::Inward {
            Some(self.factor_inward * weight)
        } else if direction == SameFingerRollDirection::Outward {
            Some(self.factor_outward * weight)
        } else {
            Some(0.0)
        }
    }
}
