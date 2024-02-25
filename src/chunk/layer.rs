use core::mem::size_of;
use std::collections::HashMap;

use crate::chunk::{block::Block, storage::ChunkStorage, Chunk, CHUNK_SIZE};

type ChunkKey = (isize, isize);
#[derive(Debug)]
pub struct Layer {
    pub chunks: HashMap<ChunkKey, Chunk>,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert((chunk.x, chunk.y), chunk);
    }

    pub fn generate_chunk(key: ChunkKey) -> Chunk {
        Chunk {
            x: key.0,
            y: key.1,
            storage: ChunkStorage::Uniform(Block::AIR),
        }
    }

    pub fn get_key(x: isize, y: isize) -> (ChunkKey, usize, usize) {
        (
            (x / CHUNK_SIZE as isize, y / CHUNK_SIZE as isize),
            (x % CHUNK_SIZE as isize) as usize,
            (y % CHUNK_SIZE as isize) as usize,
        )
    }

    pub fn get_block_immut(&self, x: isize, y: isize) -> Option<Block> {
        let (key, x, y) = Self::get_key(x, y);
        self.chunks.get(&key).map(|chunk| chunk.get_block(x, y))
    }

    pub fn get_block(&mut self, x: isize, y: isize) -> Block {
        let (key, x, y) = Self::get_key(x, y);
        self.chunks
            .entry(key)
            .or_insert(Self::generate_chunk(key))
            .get_block(x, y)
    }

    pub fn set_block(&mut self, x: isize, y: isize, block: Block) {
        let (key, x, y) = Self::get_key(x, y);
        self.chunks
            .entry(key)
            .or_insert(Self::generate_chunk(key))
            .set_block(x, y, block);
    }

    pub fn size(&self) -> usize {
        size_of::<Self>()
            + self
                .chunks
                .values()
                .map(|chunk| chunk.size())
                .sum::<usize>()
    }

    pub fn draw(&self) {
        for chunk in self.chunks.values() {
            chunk.draw()
        }
    }
}
