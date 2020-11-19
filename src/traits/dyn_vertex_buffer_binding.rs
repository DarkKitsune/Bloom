use crate::*;

pub trait DynVertexBufferBinding {
    fn buffer(&self) -> &dyn DynVertexBuffer;
    fn divisor(&self) -> GLuint;
}