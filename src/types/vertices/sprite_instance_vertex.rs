use crate::*;
use fennec_algebra::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 4] = [
    VertexAttributeBinding::Float2,
    VertexAttributeBinding::Float2,
    VertexAttributeBinding::Float4,
    VertexAttributeBinding::Float4,
];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteInstanceVertex {
    position: Vec2f,
    scale: Vec2f,
    rotation: Quaternion,
    rectangle: Vec4f,
}

impl SpriteInstanceVertex {
    pub fn new(position: Vec2f, scale: Vec2f, rotation: Quaternion, rectangle: Vec4f) -> Self {
        Self {
            position,
            scale,
            rotation,
            rectangle,
        }
    }
}

impl Vertex for SpriteInstanceVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
