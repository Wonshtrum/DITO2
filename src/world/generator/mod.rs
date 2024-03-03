use crate::{
    chunk::{
        blocks::Block, storage::ChunkStorage, Chunk, ChunkGenerator, ChunkKey, CHUNK_AREA,
        CHUNK_SIZE,
    },
    world::generator::noise::MultiOctaves,
};

use self::noise::{Perlin2D, Sampler2D};

pub mod noise;

pub struct WorldGenerator {
    seed: usize,
    sampler: MultiOctaves<Perlin2D>,
}

impl WorldGenerator {
    pub fn new(seed: usize) -> WorldGenerator {
        Self {
            seed,
            sampler: MultiOctaves {
                noise: Perlin2D::new(seed),
                base_f: 32.,
                mul_f: 0.5,
                mul_a: 0.5,
                offset: 0.,
                octaves: 4,
            },
        }
    }
}

const WATER_LEVEL: isize = 24;
impl Sampler2D for WorldGenerator {
    fn sample(&self, x: f32, y: f32) -> f32 {
        3. - self.sampler.sample(x, y) - y / 10.
    }
}

impl ChunkGenerator for WorldGenerator {
    fn get_chunk(&self, key: ChunkKey) -> Chunk {
        let (ox, oy) = key;
        let mut chunk = [Block::AIR; CHUNK_AREA];
        for y in 0..CHUNK_SIZE {
            let ay = oy * CHUNK_SIZE as isize + y as isize;
            for x in 0..CHUNK_SIZE {
                let ax = ox * CHUNK_SIZE as isize + x as isize;
                let v = self.sample(ax as f32, ay as f32);
                chunk[x + y * CHUNK_SIZE] = if v < 0.5 {
                    if ay <= WATER_LEVEL {
                        Block::WATER
                    } else {
                        Block::AIR
                    }
                } else if v < 0.8 {
                    let v = self.sample(ax as f32, (ay + 1) as f32);
                    if ay > WATER_LEVEL && v < 0.5 {
                        Block::GRASS
                    } else {
                        Block::DIRT
                    }
                } else {
                    Block::STONE
                }
            }
        }

        Chunk {
            x: ox,
            y: oy,
            storage: ChunkStorage::Grid(Box::new(chunk)),
        }
    }
}
