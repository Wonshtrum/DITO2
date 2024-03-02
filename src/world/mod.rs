use std::ops::{Deref, DerefMut};

use crate::chunk::layer::Layer;

pub enum Entity {}

pub struct World {
    pub terrain: Layer,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            terrain: Layer::new(),
            entities: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        for entity in &mut self.entities {}
    }

    pub fn draw(&self) {
        self.terrain.draw();
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
