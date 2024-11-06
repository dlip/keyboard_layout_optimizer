use keyboard_layout::{
    key::{Direction, Hand},
    layout::LayerKey,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum DirectionalRoll {
    None,    // 0
    Inward,  // 1
    Outward, // 2
}

impl DirectionalRoll {
    pub fn new(lk1: &LayerKey, lk2: &LayerKey) -> Self {
        let k1 = lk1.key.clone();
        let k2 = lk2.key.clone();
        // should also check same row
        if k1.hand != k2.hand || k1.finger != k2.finger {
            return DirectionalRoll::None;
        }

        let d1 = k1.direction;
        let d2 = k2.direction;
        if d1 == Direction::North && d2 == Direction::East
            || d1 == Direction::East && d2 == Direction::South
            || d1 == Direction::South && d2 == Direction::West
            || d1 == Direction::West && d2 == Direction::North
        {
            if k1.hand == Hand::Left {
                DirectionalRoll::Inward
            } else {
                DirectionalRoll::Outward
            }
        } else if d1 == Direction::North && d2 == Direction::West
            || d1 == Direction::West && d2 == Direction::South
            || d1 == Direction::South && d2 == Direction::East
            || d1 == Direction::East && d2 == Direction::North
        {
            if k1.hand == Hand::Left {
                DirectionalRoll::Outward
            } else {
                DirectionalRoll::Inward
            }
        } else {
            DirectionalRoll::None
        }
    }
}
