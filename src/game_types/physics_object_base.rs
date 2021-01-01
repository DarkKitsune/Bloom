use crate::*;

pub struct PhysicsObjectBase {
    position: Vec3f,
    velocity: Vec3f,
}

impl PhysicsObjectBase {
    pub fn new(position: Vec3f, velocity: Vec3f) -> Self {
        Self { position, velocity }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.position += self.velocity * delta_time as f32;
    }

    pub fn position(&self) -> Vec3f {
        self.position
    }

    pub fn velocity(&self) -> Vec3f {
        self.velocity
    }
}
