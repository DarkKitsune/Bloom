use crate::*;

pub trait DynVertexBuffer: GLHandle + std::fmt::Debug {
    fn element_size(&self) -> GLsizei;
    fn vertex_attribute_bindings(&self) -> &'static [VertexAttributeBinding];
    fn length(&self) -> GLsizeiptr;
    fn map(&self, range: std::ops::Range<GLsizeiptr>) -> Box<dyn DynVertexBufferMap>;
}
