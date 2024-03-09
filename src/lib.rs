use core::{fmt, mem::size_of};

use chunk::{ChunkGenerator, CHUNK_SIZE};

use crate::{
    chunk::blocks::{Block, BlockType},
    world::World,
};

mod chunk;
mod wasm;
mod world;

trait TotalSize: Sized {
    fn static_size() -> usize {
        size_of::<Self>()
    }
    fn dynamic_size(&self) -> usize;
    fn total_size(&self) -> usize {
        Self::static_size() + self.dynamic_size()
    }
}

struct DebugInline<D: fmt::Debug>(D);
impl<D: fmt::Debug> fmt::Debug for DebugInline<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X?}", self.0)
    }
}

impl World {
    #[no_mangle]
    pub extern "C" fn world_new() -> Box<Self> {
        let mut world = World::new(0);
        for i in -6..6 {
            for j in -2..2 {
                let c = world.generator.get_chunk((i, j));
                world.terrain.add_chunk(c);
            }
        }
        Box::new(world)
    }

    #[no_mangle]
    pub extern "C" fn world_set_block(&mut self, x: f32, y: f32, id: BlockType, flags: u8) {
        self.set_block(
            x as isize,
            y as isize,
            Block {
                typ: id,
                flags: flags | self.terrain.updated_flag,
            },
        );
    }

    #[no_mangle]
    pub extern "C" fn world_update(&mut self) {
        self.update();
    }

    #[no_mangle]
    pub extern "C" fn world_update_meshes(&mut self) {
        self.update_meshes();
    }

    #[no_mangle]
    pub extern "C" fn world_debug_draw(&self) {
        self.debug_draw();
    }

    #[no_mangle]
    pub extern "C" fn world_debug(&self) {
        log!("{:^#?}", self.terrain);
    }

    #[no_mangle]
    pub extern "C" fn world_total_size(&self) -> usize {
        self.total_size()
    }
}

#[no_mangle]
pub extern "C" fn chunk_size() -> usize {
    CHUNK_SIZE
}

#[no_mangle]
pub extern "C" fn init_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        wasm::error(&info.to_string());
    }));
    wasm::log("Panic Hook successfully initialized");
}
