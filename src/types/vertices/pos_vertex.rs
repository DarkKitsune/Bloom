use crate::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 1] = [VertexAttributeBinding::Float3];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PosVertex {
    position: Vec3f,
}

impl PosVertex {
    pub fn new(position: Vec3f) -> Self {
        Self { position }
    }
}

impl Vertex for PosVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
