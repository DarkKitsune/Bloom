use crate::*;

pub trait DynVertexBufferMap {
    fn buffer_handle(&self) -> IntHandle;
    fn element_size(&self) -> usize;
    fn ptr(&self) -> *const std::ffi::c_void;
    unsafe fn add(&self, idx: usize) -> *const std::ffi::c_void {
        self.ptr().add(self.element_size() as usize * idx)
    }
}

pub trait DynVertexBufferMapBox: Sized {
    fn unmap(self) {}
}

impl DynVertexBufferMapBox for Box<dyn DynVertexBufferMap> {}