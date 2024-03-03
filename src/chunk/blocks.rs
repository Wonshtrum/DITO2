use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    pub id: u8,
    pub flags: u8,
}

impl Block {
    pub const AIR: Block = Block { id: 0, flags: 0 };
    pub const STONE: Block = Block { id: 1, flags: 0 };
    pub const GRASS: Block = Block { id: 2, flags: 0 };
    pub const DIRT: Block = Block { id: 3, flags: 0 };
    pub const BRICK: Block = Block { id: 4, flags: 0 };
    pub const TORCH: Block = Block { id: 5, flags: 0 };
    pub const WATER: Block = Block { id: 6, flags: 0 };
    pub const BARK: Block = Block { id: 7, flags: 0 };
    pub const LEAF: Block = Block { id: 8, flags: 0 };

    pub fn texture(&self) -> usize {
        self.id as usize
    }
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
