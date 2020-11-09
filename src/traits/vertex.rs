#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VertexAttributeBinding {
    PositionF3,
    TexCoordF2,
    NormalF3,
    ColorF3,
}

pub trait Vertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding];
}
