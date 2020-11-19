use crate::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 2] = [
    VertexAttributeBinding::Float3,
    VertexAttributeBinding::Float3,
];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PosColorVertex {
    position: Vec3f,
    color: Vec3f,
}

impl PosColorVertex {
    pub fn new(position: Vec3f, color: Vec3f) -> Self {
        Self { position, color }
    }
}

impl Vertex for PosColorVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
