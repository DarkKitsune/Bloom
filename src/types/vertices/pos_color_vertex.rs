use crate::*;
use fennec_algebra::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 2] = [
    VertexAttributeBinding::PositionF3,
    VertexAttributeBinding::ColorF3,
];

#[repr(C)]
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
