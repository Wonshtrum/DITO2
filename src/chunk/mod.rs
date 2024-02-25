use core::{
    mem::size_of,
    ops::{Deref, DerefMut},
};

use crate::{
    chunk::storage::{ChunkStorage, Palette},
    wasm::{
        self,
        draw::{BoundingBox, Rectangle, RGB},
    },
};

pub mod block;
pub mod layer;
pub mod storage;

const CHUNK_SIZE: usize = 16;
const CHUNK_HALF_SIZE: usize = CHUNK_SIZE / 2;
const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
const PALETTE_SIZE: usize = CHUNK_SIZE * CHUNK_HALF_SIZE;
const LOW_NIBBLE: usize = 1;

#[derive(Debug)]
pub struct Chunk {
    pub x: isize,
    pub y: isize,
    pub storage: ChunkStorage,
}

const COLORS: &[RGB] = &[RGB::BLACK, RGB::GREY, RGB::RED, RGB::GREEN];
impl Chunk {
    pub fn size(&self) -> usize {
        size_of::<Self>() + self.storage.size()
    }

    pub fn draw(&self) {
        let ox = self.x * (CHUNK_SIZE as isize);
        let oy = self.y * (CHUNK_SIZE as isize);
        match &self.storage {
            ChunkStorage::Uniform(blocks) => wasm::fill_rect(Rectangle {
                bbox: BoundingBox::square(ox, oy, CHUNK_SIZE),
                fill: COLORS[blocks.id as usize],
            }),
            ChunkStorage::Palette(blocks) => {
                let Palette { palette, data, .. } = &**blocks;
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_HALF_SIZE {
                        let byte = data[y * CHUNK_HALF_SIZE + x];
                        let block1 = palette[(byte & 0x0F) as usize].0;
                        let block2 = palette[((byte >> 4) & 0x0F) as usize].0;
                        let ox = ox + (x * 2) as isize;
                        wasm::fill_rect(Rectangle {
                            bbox: BoundingBox::square(ox + 1, oy + y as isize, 1),
                            fill: COLORS[block1.id as usize],
                        });
                        wasm::fill_rect(Rectangle {
                            bbox: BoundingBox::square(ox, oy + y as isize, 1),
                            fill: COLORS[block2.id as usize],
                        });
                    }
                }
            }
            ChunkStorage::Grid(blocks) => {
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        let block = blocks[y * CHUNK_SIZE + x];
                        wasm::fill_rect(Rectangle {
                            bbox: BoundingBox::square(ox + x as isize, oy + y as isize, 1),
                            fill: COLORS[block.id as usize],
                        })
                    }
                }
            }
        }
    }
}

impl Deref for Chunk {
    type Target = ChunkStorage;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for Chunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}
