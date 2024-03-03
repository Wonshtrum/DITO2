use core::{
    mem::size_of,
    ops::{Deref, DerefMut},
};

use crate::{
    chunk::storage::{ChunkStorage, Palette},
    wasm::draw::{draw_quad, Rectangle},
};

pub mod blocks;
pub mod layer;
pub mod storage;

pub trait ChunkGenerator {
    fn get_chunk(&self, key: ChunkKey) -> Chunk;
}

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HALF_SIZE: usize = CHUNK_SIZE / 2;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const PALETTE_SIZE: usize = CHUNK_SIZE * CHUNK_HALF_SIZE;
pub const LOW_NIBBLE: usize = 1;

pub type ChunkKey = (isize, isize);
pub fn get_key(x: isize, y: isize) -> (ChunkKey, usize, usize) {
    (
        (
            x.div_euclid(CHUNK_SIZE as isize),
            y.div_euclid(CHUNK_SIZE as isize),
        ),
        x.rem_euclid(CHUNK_SIZE as isize) as usize,
        y.rem_euclid(CHUNK_SIZE as isize) as usize,
    )
}

#[derive(Debug)]
pub struct Chunk {
    pub x: isize,
    pub y: isize,
    pub storage: ChunkStorage,
}

impl Chunk {
    pub fn size(&self) -> usize {
        size_of::<Self>() + self.storage.size()
    }

    pub fn draw(&self) {
        let ox = self.x * (CHUNK_SIZE as isize);
        let oy = self.y * (CHUNK_SIZE as isize);
        match &self.storage {
            ChunkStorage::Uniform(blocks) => {
                draw_quad(Rectangle::square(ox, oy, CHUNK_SIZE, blocks.texture()))
            }
            ChunkStorage::Palette(blocks) => {
                let Palette { palette, data, .. } = &**blocks;
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_HALF_SIZE {
                        let byte = data[y * CHUNK_HALF_SIZE + x];
                        let block1 = palette[(byte & 0x0F) as usize].0;
                        let block2 = palette[((byte >> 4) & 0x0F) as usize].0;
                        let ox = ox + (x * 2) as isize;
                        draw_quad(Rectangle::square(
                            ox + 1,
                            oy + y as isize,
                            1,
                            block1.texture(),
                        ));
                        draw_quad(Rectangle::square(ox, oy + y as isize, 1, block2.texture()));
                    }
                }
            }
            ChunkStorage::Grid(blocks) => {
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        let block = blocks[y * CHUNK_SIZE + x];
                        draw_quad(Rectangle::square(
                            ox + x as isize,
                            oy + y as isize,
                            1,
                            block.texture(),
                        ))
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
