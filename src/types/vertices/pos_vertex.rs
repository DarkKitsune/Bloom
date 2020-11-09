use crate::*;
use fennec_algebra::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 1] = [VertexAttributeBinding::PositionF3];

#[repr(C)]
pub struct PosVertex {
    position: Vector<f32, 3>,
}

impl PosVertex {
    pub fn new(position: Vector<f32, 3>) -> Self {
        Self { position }
    }
}

impl Vertex for PosVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
