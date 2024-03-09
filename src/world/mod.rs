use crate::{
    chunk::{blocks::Block, layer::Layer},
    world::generator::WorldGenerator,
    TotalSize,
};

pub mod generator;

pub enum Entity {}

pub struct World {
    pub generator: WorldGenerator,
    pub terrain: Layer,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new(seed: usize) -> Self {
        Self {
            generator: WorldGenerator::new(seed),
            terrain: Layer::new(),
            entities: Vec::new(),
        }
    }

    pub fn get_block(&mut self, x: isize, y: isize) -> Block {
        self.terrain.get_block(x, y, &self.generator)
    }

    pub fn set_block(&mut self, x: isize, y: isize, block: Block) {
        self.terrain.set_block(x, y, block, &self.generator)
    }

    pub fn update(&mut self) {
        for entity in &self.entities {}
        self.terrain.update(&self.generator);
    }

    pub fn update_meshes(&mut self) {
        for entity in &self.entities {}
        self.terrain.update_meshes();
    }

    pub fn debug_draw(&self) {
        for entity in &self.entities {}
        self.terrain.debug_draw();
    }
}

impl TotalSize for World {
    fn dynamic_size(&self) -> usize {
        self.terrain.dynamic_size()
    }
}
