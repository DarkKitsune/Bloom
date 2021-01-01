use crate::*;

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(meshes: Vec<Mesh>) -> Self {
        Self { meshes }
    }

    pub fn meshes(&self) -> &[Mesh] {
        &self.meshes
    }

    pub fn meshes_mut(&mut self) -> &mut [Mesh] {
        &mut self.meshes
    }
}
