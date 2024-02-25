use core::fmt;

use crate::chunk::{block::Block, layer::Layer, storage::ChunkStorage, Chunk};

mod chunk;
mod wasm;

struct DebugInline<D: fmt::Debug>(D);
impl<D: fmt::Debug> fmt::Debug for DebugInline<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X?}", self.0)
    }
}

#[no_mangle]
pub extern "C" fn main() {
    log!("before");
    let chunks = [
        Chunk {
            x: 0,
            y: 0,
            storage: ChunkStorage::Uniform(Block::AIR),
        },
        Chunk {
            x: 0,
            y: 1,
            storage: ChunkStorage::Uniform(Block::STONE),
        },
        Chunk {
            x: 1,
            y: 0,
            storage: ChunkStorage::Uniform(Block::DIRT),
        },
    ];
    let mut world = Layer::new();
    for chunk in chunks {
        world.add_chunk(chunk);
    }

    for i in 0..8 {
        world.set_block(i, 0, Block::AIR);
    }
    for i in 0..10 {
        world.set_block(i, 0, Block::STONE);
    }
    world.set_block(0, 0, Block::GRASS);
    world.set_block(20, 20, Block::GRASS);
    world.draw();
    log!("{:^#?}", world);
    log!("{}", world.size());
    log!("after");
}

#[no_mangle]
pub extern "C" fn init_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        wasm::error(&info.to_string());
    }));
    wasm::log("Panic Hook successfully initialized");
}
