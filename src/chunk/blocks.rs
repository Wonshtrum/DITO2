use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Brick = 4,
    Torch = 5,
    Water = 6,
    Bark = 7,
    Leaf = 8,
    Sand = 10,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    pub typ: BlockType,
    pub flags: u8,
}

use BlockType as BT;
#[rustfmt::skip]
impl Block {
    pub const AIR: Block = Block { typ: BT::Air, flags: 0 };
    pub const STONE: Block = Block { typ: BT::Stone, flags: 0 };
    pub const GRASS: Block = Block { typ: BT::Grass, flags: 0 };
    pub const DIRT: Block = Block { typ: BT::Dirt, flags: 0 };
    pub const BRICK: Block = Block { typ: BT::Brick, flags: 0 };
    pub const TORCH: Block = Block { typ: BT::Torch, flags: 0 };
    pub const WATER: Block = Block { typ: BT::Water, flags: 0 };
    pub const BARK: Block = Block { typ: BT::Bark, flags: 0 };
    pub const LEAF: Block = Block { typ: BT::Leaf, flags: 0 };
    pub const SAND: Block = Block { typ: BT::Sand, flags: 0 };

    pub fn texture(&self) -> usize {
        self.typ as usize
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.typ {
            BT::Air => write!(f, "AIR"),
            BT::Stone => write!(f, "STONE"),
            BT::Grass => write!(f, "GRASS"),
            BT::Dirt => write!(f, "DIRT"),
            BT::Brick => write!(f, "BRICK"),
            BT::Torch => write!(f, "TORCH"),
            BT::Water => write!(f, "WATER"),
            BT::Bark => write!(f, "BARK"),
            BT::Leaf => write!(f, "LEAF"),
            BT::Sand => write!(f, "SAND"),
        }
    }
}
