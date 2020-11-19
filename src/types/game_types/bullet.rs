use crate::*;
use fennec_algebra::*;

pub struct Bullet {
    position: Vec2f,
    scale: Vec2f,
    velocity: Vec2f,
    rectangle: Vec4f,
}

impl Bullet {
    pub fn new(position: Vec2f, scale: Vec2f, velocity: Vec2f, rectangle: Vec4f) -> Self {
        Self {
            position,
            scale,
            velocity,
            rectangle,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }

    pub fn position(&self) -> Vec2f {
        self.position
    }

    pub fn scale(&self) -> Vec2f {
        self.scale
    }

    pub fn velocity(&self) -> Vec2f {
        self.velocity
    }

    pub fn rectangle(&self) -> Vec4f {
        self.rectangle
    }
}