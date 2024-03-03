use core::mem::size_of;
use std::collections::HashMap;

use crate::chunk::{blocks::Block, get_key, Chunk, ChunkGenerator, ChunkKey};

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

    pub fn get_block_immut(&self, x: isize, y: isize) -> Option<Block> {
        let (key, x, y) = get_key(x, y);
        self.chunks.get(&key).map(|chunk| chunk.get_block(x, y))
    }

    pub fn get_block<G: ChunkGenerator>(&mut self, x: isize, y: isize, g: &G) -> Block {
        let (key, x, y) = get_key(x, y);
        self.chunks
            .entry(key)
            .or_insert_with(|| g.get_chunk(key))
            .get_block(x, y)
    }

    pub fn set_block<G: ChunkGenerator>(&mut self, x: isize, y: isize, block: Block, g: &G) {
        let (key, x, y) = get_key(x, y);
        self.chunks
            .entry(key)
            .or_insert_with(|| g.get_chunk(key))
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
