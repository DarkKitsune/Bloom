use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VertexAttributeBinding {
    Float,
    Float2,
    Float3,
    Float4,
    Mat4f,
}

impl VertexAttributeBinding {
    pub fn locations_used(self) -> GLuint {
        match self {
            VertexAttributeBinding::Float => 1,
            VertexAttributeBinding::Float2 => 1,
            VertexAttributeBinding::Float3 => 1,
            VertexAttributeBinding::Float4 => 1,
            VertexAttributeBinding::Mat4f => 4,
        }
    }
}

pub trait Vertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding];
}
