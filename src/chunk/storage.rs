use core::{fmt, mem::size_of};

use crate::{
    chunk::{block::Block, CHUNK_AREA, CHUNK_HALF_SIZE, CHUNK_SIZE, LOW_NIBBLE, PALETTE_SIZE},
    log, DebugInline,
};

pub enum ChunkStorage {
    Uniform(Block),
    Grid(Box<[Block; CHUNK_AREA]>),
    Palette(Box<Palette>),
}

impl ChunkStorage {
    pub fn new_uniform(block: Block) -> Self {
        Self::Uniform(block)
    }

    pub fn new_grid(block: Block) -> Self {
        Self::Grid(Box::new([block; CHUNK_AREA]))
    }

    pub fn new_palette(block: Block) -> Self {
        let mut palette = [(block, 0); 16];
        palette[0].1 = CHUNK_AREA as u16;
        Self::Palette(Box::new(Palette {
            palette,
            data: [0; PALETTE_SIZE],
        }))
    }

    pub fn get_block(&self, x: usize, y: usize) -> Block {
        match self {
            Self::Uniform(blocks) => *blocks,
            Self::Grid(blocks) => blocks[y * CHUNK_SIZE + x],
            Self::Palette(blocks) => {
                let Palette { palette, data, .. } = &**blocks;
                let idx = y * CHUNK_SIZE + x;
                let id = if idx % 2 == 0 {
                    data[idx >> 1]
                } else {
                    data[idx >> 1] >> 4
                };
                palette[(id & 0x0F) as usize].0
            }
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, block: Block) {
        match self {
            Self::Uniform(blocks) => {
                if blocks != &block {
                    *self = Self::new_palette(*blocks);
                    self.set_block(x, y, block);
                }
            }
            Self::Grid(blocks) => {
                blocks[y * CHUNK_SIZE + x] = block;
            }
            Self::Palette(blocks) => {
                let Palette { palette, data } = &mut **blocks;

                let idx = y * CHUNK_SIZE + x;
                let half_idx = idx >> 1;
                let nibbles = data[half_idx];
                let old_id = if idx % 2 == LOW_NIBBLE {
                    data[half_idx]
                } else {
                    data[half_idx] >> 4
                };
                let old_id = (old_id & 0x0F) as usize;

                let (old_block, old_n) = &mut palette[old_id];
                if old_block != &block {
                    *old_n -= 1;
                    let mut id = None;
                    for (i, (b, n)) in palette.iter_mut().enumerate() {
                        if *n == 0 && id.is_none() {
                            *b = block;
                            id = Some(i);
                        } else if b == &block {
                            id = Some(i);
                            break;
                        }
                    }
                    let id = match id {
                        Some(id) => {
                            palette[id].1 += 1;
                            id as u8
                        }
                        None => todo!("PALETTE FULL"),
                    };
                    if idx % 2 == LOW_NIBBLE {
                        data[half_idx] = (nibbles & 0xF0) | id;
                    } else {
                        data[half_idx] = (nibbles & 0x0F) | (id << 4);
                    }
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Uniform(_) => 0,
            Self::Grid(_) => size_of::<Block>() * CHUNK_AREA,
            Self::Palette(_) => size_of::<Palette>(),
        }
    }
}

impl fmt::Debug for ChunkStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uniform(blocks) => write!(f, "Uniform({blocks:?})"),
            Self::Grid(_blocks) => write!(f, "Grid(...)"),
            Self::Palette(palette) => palette.fmt(f),
        }
    }
}

pub struct Palette {
    /// palette stores up to 16 entries corresponding to the 16 combinations of a nibble.
    /// u16 stores blocks for chunks of size up to 256.
    pub palette: [(Block, u16); 16],
    pub data: [u8; PALETTE_SIZE],
}

impl fmt::Debug for Palette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        log!("{:?}", f.align());
        if let Some(fmt::Alignment::Center) = f.align() {
            f.debug_struct("Palette")
                .field(
                    "palette",
                    &self.palette.iter().map(DebugInline).collect::<Vec<_>>(),
                )
                .field(
                    "data",
                    &self
                        .data
                        .chunks(CHUNK_HALF_SIZE)
                        .map(DebugInline)
                        .collect::<Vec<_>>(),
                )
                .finish()
        } else {
            f.write_str("Palette(")?;
            f.debug_list()
                .entries(self.palette.iter().map(DebugInline))
                .finish()?;
            f.write_str(")")
        }
    }
}
