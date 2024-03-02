use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub id: u8,
    pub flags: u8,
}

impl Block {
    pub const AIR: Block = Block { id: 0, flags: 0 };
    pub const STONE: Block = Block { id: 1, flags: 0 };
    pub const DIRT: Block = Block { id: 2, flags: 0 };
    pub const GRASS: Block = Block { id: 3, flags: 0 };
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id {
            0 => write!(f, "AIR"),
            1 => write!(f, "STONE"),
            2 => write!(f, "DIRT"),
            3 => write!(f, "GRASS"),
            _ => write!(f, "Unknown"),
        }
    }
}
