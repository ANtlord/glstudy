/// 0000
/// ^^^^
/// ||||- forward
/// |||-- backward
/// ||--- left
/// |---- right
pub struct MoveBitMap(u16);

pub enum Way {
    Forward,
    Backward,
    Left,
    Right,
}

impl MoveBitMap {
    pub fn set(&self, value: Way) -> Self {
        match value {
            Way::Forward => Self(self.0 | 0b0001),
            Way::Backward => Self(self.0 | 0b0010),
            Way::Left => Self(self.0 | 0b0100),
            Way::Right => Self(self.0 | 0b1000),
        }
    }

    pub fn unset(&self, value: Way) -> Self {
        match value {
            Way::Forward => Self(self.0 & 0b1110),
            Way::Backward => Self(self.0 & 0b1101),
            Way::Left => Self(self.0 & 0b1011),
            Way::Right => Self(self.0 & 0b0111),
        }
    }

    pub fn is(&self, value: Way) -> bool {
        match value {
            Way::Forward => self.0 & 0b0001 > 0,
            Way::Backward => self.0 & 0b0010 > 0,
            Way::Left => self.0 & 0b0100 > 0,
            Way::Right => self.0 & 0b1000 > 0,
        }
    }
}

impl Default for MoveBitMap {
    fn default() -> Self {
        Self(0)
    }
}
