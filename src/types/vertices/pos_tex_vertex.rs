use crate::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 2] = [
    VertexAttributeBinding::Float3,
    VertexAttributeBinding::Float2,
];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PosTexVertex {
    position: Vec3f,
    tex_coord: Vec2f,
}

impl PosTexVertex {
    pub fn new(position: Vec3f, tex_coord: Vec2f) -> Self {
        Self {
            position,
            tex_coord,
        }
    }
}

impl Vertex for PosTexVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
