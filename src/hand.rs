extern crate cage;

use cage::Offset;

#[derive(Copy, Clone, Debug)]
pub struct Hand {
    pub offset: Offset
}

impl Default for Hand {
    fn default() -> Self {
        Hand { offset: Offset { x: 0.0, y: 0.0, z: 0.05 } }
    }
}

impl Hand {
    pub fn plus_offset(&self, offset: &Offset) -> Self {
        Hand { offset: self.offset.shift(offset.x, offset.y, offset.z) }
    }
    pub fn minus_offset(&self, offset: &Offset) -> Self {
        Hand { offset: self.offset.shift(-offset.x, -offset.y, -offset.z) }
    }
}
