use crate::*;

pub trait DynVertexBufferMap {
    fn buffer_handle(&self) -> IntHandle;
    fn element_size(&self) -> usize;
    fn range(&self) -> std::ops::Range<GLsizeiptr>;
    fn ptr(&self) -> *const std::ffi::c_void;

    unsafe fn add(&self, idx: usize) -> *const std::ffi::c_void {
        if DEBUG && idx > (self.range().end - self.range().start) as usize {
            panic!("Idx is outside mapped range {:?}", self.range());
        }

        self.ptr().add(self.element_size() as usize * idx)
    }
}
