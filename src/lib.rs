use core::fmt;

use crate::{
    chunk::{blocks::Block, storage::ChunkStorage, Chunk},
    world::World,
};

mod chunk;
mod wasm;
mod world;

struct DebugInline<D: fmt::Debug>(D);
impl<D: fmt::Debug> fmt::Debug for DebugInline<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X?}", self.0)
    }
}

#[no_mangle]
pub extern "C" fn create_world() -> Box<World> {
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

    let mut world = World::new();
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
    log!("after");
    Box::new(world)
}

#[no_mangle]
pub extern "C" fn set_block(world: &mut World, x: f32, y: f32, id: u8, flags: u8) {
    world.set_block(x as isize, y as isize, Block { id, flags });
}

#[no_mangle]
pub extern "C" fn draw(world: &mut World) {
    world.draw();
}

#[no_mangle]
pub extern "C" fn update(world: &mut World) {
    world.update();
}

#[no_mangle]
pub extern "C" fn debug(world: &mut World) {
    log!("{:^#?}", world.terrain);
    log!("{}", world.size());
}

#[no_mangle]
pub extern "C" fn init_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        wasm::error(&info.to_string());
    }));
    wasm::log("Panic Hook successfully initialized");
}
