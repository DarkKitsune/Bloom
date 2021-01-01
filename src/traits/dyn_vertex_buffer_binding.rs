use crate::*;
use std::cell::RefCell;

pub trait DynVertexBufferBinding: std::fmt::Debug {
    fn buffer(&self) -> &RefCell<Box<dyn DynVertexBuffer>>;
    fn divisor(&self) -> GLuint;
}
