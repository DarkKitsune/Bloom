use crate::*;
use fennec_algebra::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 4] = [
    VertexAttributeBinding::Float4,
    VertexAttributeBinding::Float4,
    VertexAttributeBinding::Float4,
    VertexAttributeBinding::Float4,
];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteActorVertex {
    position: Vec2f,
    velocity: Vec2f,
    scale: Vec2f,
    scalar_velocity: Vec2f,
    rotation: f32,
    last_updated: f32,
    animation_time: f32,
    _0: f32,
    rectangle: Vec4f,
}

impl SpriteActorVertex {
    pub fn new(rectangle: Vec4f, current_time: f64) -> Self {
        Self {
            position: Vector::zero(),
            velocity: Vector::zero(),
            scale: Vector::one(),
            scalar_velocity: Vector::zero(),
            rotation: 0.0,
            last_updated: current_time as f32,
            animation_time: 0.0,
            _0: 0.0,
            rectangle,
        }
    }

    pub fn with_position(mut self, position: Vec2f) -> Self {
        self.position = position;
        self
    }

    pub fn with_velocity(mut self, velocity: Vec2f) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_scale(mut self, scale: Vec2f) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_scalar_velocity(mut self, scalar_velocity: Vec2f) -> Self {
        self.scalar_velocity = scalar_velocity;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_rectangle(mut self, rectangle: Vec4f) -> Self {
        self.rectangle = rectangle;
        self
    }

    pub fn position(&self, current_time: f64) -> Vec2f {
        let time_passed = (current_time - self.last_updated as f64) as f32;
        self.position + self.velocity() * time_passed
    }

    pub fn velocity(&self) -> Vec2f {
        self.velocity
    }

    pub fn scale(&self, current_time: f64) -> Vec2f {
        let time_passed = (current_time - self.last_updated as f64) as f32;
        self.scale + self.scalar_velocity() * time_passed
    }

    pub fn scalar_velocity(&self) -> Vec2f {
        self.scalar_velocity
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn rectangle(&self) -> Vec4f {
        self.rectangle
    }

    pub fn set_position(&mut self, position: Vec2f, current_time: f64) {
        self.apply_time_changes(current_time);
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: Vec2f, current_time: f64) {
        self.apply_time_changes(current_time);
        self.velocity = velocity;
    }

    pub fn set_scale(&mut self, scale: Vec2f, current_time: f64) {
        self.apply_time_changes(current_time);
        self.scale = scale;
    }

    pub fn set_scalar_velocity(&mut self, scalar_velocity: Vec2f, current_time: f64) {
        self.apply_time_changes(current_time);
        self.scalar_velocity = scalar_velocity;
    }

    pub fn set_rotation(&mut self, rotation: f32, current_time: f64) {
        self.apply_time_changes(current_time);
        self.rotation = rotation;
    }

    pub fn set_rectangle(&mut self, rectangle: Vec4f, current_time: f64) {
        self.apply_time_changes(current_time);
        self.rectangle = rectangle;
    }

    pub fn apply_time_changes(&mut self, current_time: f64) {
        self.position = self.position(current_time);
        self.scale = self.scale(current_time);
        self.last_updated = current_time as f32;
    }
}

impl Vertex for SpriteActorVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
