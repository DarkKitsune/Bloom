use crate::*;
use fennec_algebra::*;

pub struct Player {
    position: Vec2f,
    velocity: Vec2f,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: vector!(0.0, 0.0),
            velocity: vector!(0.0, 0.0),
        }
    }

    pub fn draw() {}
}
