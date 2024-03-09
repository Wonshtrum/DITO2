use crate::chunk::{
    blocks::{Block, BlockType},
    layer::Layer,
    ChunkGenerator, CHUNK_SIZE,
};

impl Layer {
    pub fn update<G: ChunkGenerator>(&mut self, g: &G) {
        self.updated_flag = 1 - self.updated_flag;
        let mut keys = self.chunks.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let chunk = unsafe { self.chunks.get_mut(&key).unwrap_unchecked() };
                    let mut block = chunk.get_block(x, y);
                    if block.flags & 1 == self.updated_flag {
                        continue;
                    }
                    block.flags = (block.flags & !1) | self.updated_flag;
                    chunk.set_block(x, y, block); // not dirty
                    self.update_block(
                        block,
                        key.0 * CHUNK_SIZE as isize + x as isize,
                        key.1 * CHUNK_SIZE as isize + y as isize,
                        g,
                    );
                }
            }
            // self.chunks.insert(key, chunk);
        }
    }

    #[inline(always)]
    pub fn update_block<G: ChunkGenerator>(&mut self, block: Block, x: isize, y: isize, g: &G) {
        match block.typ {
            BlockType::Sand => {
                let down = self.get_block(x, y - 1, g).typ == BlockType::Air;
                if down {
                    self.set_block(x, y, Block::AIR, g);
                    self.set_block(x, y - 1, block, g);
                    return;
                }
                let left = self.get_block(x - 1, y, g).typ == BlockType::Air;
                let left_down = self.get_block(x - 1, y - 1, g).typ == BlockType::Air;
                let right = self.get_block(x + 1, y, g).typ == BlockType::Air;
                let right_down = self.get_block(x + 1, y - 1, g).typ == BlockType::Air;
                match (
                    left_down && left,
                    right_down && right,
                    true,
                ) {
                    // slide left
                    (true, false, _) | (true, true, true) => {
                        self.set_block(x, y, Block::AIR, g);
                        self.set_block(x - 1, y - 1, block, g);
                    }
                    // slide right
                    (false, true, _) | (true, true, false) => {
                        self.set_block(x, y, Block::AIR, g);
                        self.set_block(x + 1, y - 1, block, g);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
