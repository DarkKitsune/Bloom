use crate::*;

pub trait DynVertexBuffer: GLHandle {
    fn element_size(&self) -> GLsizei;
    fn vertex_attribute_bindings(&self) -> &'static [VertexAttributeBinding];
    fn length(&self) -> GLsizeiptr;
    fn map(&self) -> Box<dyn DynVertexBufferMap>;
}
