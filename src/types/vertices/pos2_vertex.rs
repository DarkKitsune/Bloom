use crate::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 1] = [VertexAttributeBinding::Float2];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos2Vertex {
    position: Vec2f,
}

impl Pos2Vertex {
    pub fn new(position: Vec2f) -> Self {
        Self { position }
    }
}

impl Vertex for Pos2Vertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
