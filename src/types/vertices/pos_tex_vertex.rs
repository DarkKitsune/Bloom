use crate::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 2] = [
    VertexAttributeBinding::Float2,
    VertexAttributeBinding::Float2,
];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos2TexVertex {
    position: Vec2f,
    tex_coord: Vec2f,
}

impl Pos2TexVertex {
    pub fn new(position: Vec2f, tex_coord: Vec2f) -> Self {
        Self {
            position,
            tex_coord,
        }
    }
}

impl Vertex for Pos2TexVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
