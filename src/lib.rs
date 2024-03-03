use core::fmt;

use chunk::ChunkGenerator;

use crate::{chunk::blocks::Block, world::World};

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
    let mut world = World::new(0);
    for i in -6..6 {
        for j in -2..2 {
            let c = world.generator.get_chunk((i, j));
            world.add_chunk(c);
        }
    }
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
