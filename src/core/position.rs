use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Position{
    data: u8,
}

impl Position{
    pub fn new(x: u8, y: u8) -> Self{
        Position { data: (x << 4) | y }
    }

    pub fn x(&self) -> u8{
        self.data >> 4
    }

    pub fn y(&self) -> u8{
        self.data & 0x0F
    }

    pub fn is_valid(&self) -> bool{
        self.x() < 9 && self.y() < 10
    }
}