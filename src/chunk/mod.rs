use core::mem::MaybeUninit;

use crate::{
    chunk::{blocks::Block, storage::ChunkStorage},
    wasm::draw::{draw_quad, free_mesh, new_mesh, update_mesh, MeshRef, Rectangle, RGBA},
    TotalSize,
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

#[allow(dead_code)]
struct Vertex {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    tex: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
pub struct Chunk {
    pub x: isize,
    pub y: isize,
    pub mesh: MeshRef,
    pub storage: ChunkStorage,
}

impl Chunk {
    pub fn get_block(&self, x: usize, y: usize) -> Block {
        self.storage.get_block(x, y)
    }

    pub fn set_block(&mut self, x: usize, y: usize, block: Block) {
        self.mesh.dirty = true;
        self.storage.set_block(x, y, block)
    }

    pub fn update_mesh(&mut self) {
        let mut vertex_buffer: [Vertex; CHUNK_AREA] = unsafe {
            #[allow(invalid_value)]
            MaybeUninit::uninit().assume_init()
        };
        let mut count = 0;
        let ox = self.x * (CHUNK_SIZE as isize);
        let oy = self.y * (CHUNK_SIZE as isize);

        self.storage.for_each(|x, y, block| {
            if block.id != 0 {
                vertex_buffer[count] = Vertex {
                    x: (ox + x as isize) as f32,
                    y: (oy + y as isize) as f32,
                    w: 1.,
                    h: 1.,
                    tex: block.texture() as f32,
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                };
                count += 1;
            }
        });
        if self.mesh.id == 0 {
            self.mesh = new_mesh(&vertex_buffer[..count]);
        } else {
            update_mesh(&mut self.mesh, &vertex_buffer[..count]);
        }
    }

    pub fn debug_draw(&self) {
        let ox = self.x * (CHUNK_SIZE as isize);
        let oy = self.y * (CHUNK_SIZE as isize);
        self.storage.for_each(|x, y, block| {
            draw_quad(Rectangle::new(
                ox + x as isize,
                oy + y as isize,
                1,
                1,
                block.texture(),
                RGBA::WHITE,
            ))
        });
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        if self.mesh.id != 0 {
            free_mesh(&self.mesh);
        }
    }
}

impl TotalSize for Chunk {
    fn dynamic_size(&self) -> usize {
        self.storage.dynamic_size()
    }
}
