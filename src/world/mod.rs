use std::ops::{Deref, DerefMut};

use crate::{
    chunk::{blocks::Block, layer::Layer},
    world::generator::WorldGenerator,
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

    pub fn update(&mut self) {
        for entity in &mut self.entities {}
    }

    pub fn get_block(&mut self, x: isize, y: isize) -> Block {
        self.terrain.get_block(x, y, &self.generator)
    }

    pub fn set_block(&mut self, x: isize, y: isize, block: Block) {
        self.terrain.set_block(x, y, block, &self.generator)
    }

    pub fn draw(&self) {
        self.terrain.draw();
        for entity in &self.entities {}
    }
}

impl Deref for World {
    type Target = Layer;
    fn deref(&self) -> &Self::Target {
        &self.terrain
    }
}
impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terrain
    }
}
