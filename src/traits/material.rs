use crate::*;

pub trait Material {
    fn pipeline(&self) -> &Pipeline;
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding];
}
