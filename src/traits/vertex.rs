use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VertexAttributeBinding {
    PositionF3,
    TexCoordF2,
    NormalF3,
    ColorF3,
    Transform,
}

impl VertexAttributeBinding {
    pub fn locations_used(self) -> GLuint {
        match self {
            VertexAttributeBinding::PositionF3 => 1,
            VertexAttributeBinding::TexCoordF2 => 1,
            VertexAttributeBinding::NormalF3 => 1,
            VertexAttributeBinding::ColorF3 => 1,
            VertexAttributeBinding::Transform => 4,
        }
    }
}

pub trait Vertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding];
}
