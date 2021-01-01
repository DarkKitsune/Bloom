use crate::*;
use std::ffi::c_void;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Bound, Index, IndexMut, Range, RangeBounds};

pub struct BufferMap<T: Sized + Copy> {
    _phantom_data: PhantomData<T>,
    buffer: IntHandle,
    ptr: *mut c_void,
    range: Range<GLsizeiptr>,
    access_flags: GLenum,
}

impl<T: Sized + Copy> BufferMap<T> {
    fn new(
        buffer: &Buffer<T>,
        ptr: *mut c_void,
        range: Range<GLsizeiptr>,
        access_flags: GLenum,
    ) -> Self {
        Self {
            _phantom_data: PhantomData,
            buffer: buffer.handle(),
            ptr,
            range,
            access_flags,
        }
    }

    unsafe fn element_ptr<P: Sized>(&self, idx: usize) -> *const P {
        if DEBUG && idx > (self.range.end - self.range.start) as usize {
            panic!("Offset is outside mapped range {:?}", self.range);
        }

        (self.ptr as *const P).add(idx)
    }

    unsafe fn element_ptr_mut<P: Sized>(&self, idx: usize) -> *mut P {
        if DEBUG && idx > (self.range.end - self.range.start) as usize {
            panic!("Offset is outside mapped range {:?}", self.range);
        }

        (self.ptr as *mut P).add(idx)
    }

    pub fn unmap(self) {}
}

impl<T: Sized + Copy> Drop for BufferMap<T> {
    fn drop(&mut self) {
        unsafe { gl::UnmapNamedBuffer(self.buffer) };
    }
}

impl<T: Sized + Copy> Index<usize> for BufferMap<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        unsafe { &*self.element_ptr(idx) }
    }
}

impl<T: Sized + Copy> IndexMut<usize> for BufferMap<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        unsafe { &mut *self.element_ptr_mut(idx) }
    }
}

impl<T: Sized + Copy + Vertex> DynVertexBufferMap for BufferMap<T> {
    fn buffer_handle(&self) -> IntHandle {
        self.buffer
    }

    fn element_size(&self) -> usize {
        std::mem::size_of::<T>()
    }

    fn range(&self) -> Range<GLsizeiptr> {
        self.range.clone()
    }

    fn ptr(&self) -> *const c_void {
        self.ptr
    }
}

#[derive(Debug)]
pub struct Buffer<T: Sized + Copy> {
    _phantom_data: PhantomData<T>,
    gl_handle: IntHandle,
    access_flags: GLenum,
    length: GLsizeiptr,
}

impl<T: Sized + Copy> Buffer<T> {
    pub fn new(length: GLsizeiptr, allow_map_read: bool, allow_map_write: bool) -> Self {
        // Choose access flags for what we need
        let access_flags = choose_access_flags(allow_map_read, allow_map_write);

        // We will receive the buffer's handle in gl_handle
        let mut gl_handle: IntHandle = 0;
        unsafe {
            // Create buffer
            gl::CreateBuffers(1, &mut gl_handle as *mut _);
            // Give it sufficient storage for the capacity of T that we need
            gl::NamedBufferStorage(
                gl_handle,
                length * size_of::<T>() as GLsizeiptr,
                std::ptr::null(),
                access_flags,
            );
        }

        Self {
            _phantom_data: PhantomData,
            gl_handle,
            access_flags,
            length,
        }
    }

    pub fn from_slice(initial_data: &[T], allow_map_read: bool, allow_map_write: bool) -> Self {
        // Choose access flags for what we need
        let access_flags = choose_access_flags(allow_map_read, allow_map_write);

        // We will receive the buffer's handle in gl_handle
        let mut gl_handle: IntHandle = 0;
        unsafe {
            // Create buffer
            gl::CreateBuffers(1, &mut gl_handle as *mut _);
            // Give it sufficient storage for the capacity of T that we need
            gl::NamedBufferStorage(
                gl_handle,
                (initial_data.len() * size_of::<T>()) as GLsizeiptr,
                initial_data.as_ptr() as *const _,
                access_flags,
            );
        }

        Self {
            _phantom_data: PhantomData,
            gl_handle,
            access_flags,
            length: initial_data.len() as GLsizeiptr,
        }
    }

    pub fn from_iterator(
        initial_data: impl IntoIterator<Item = T>,
        allow_map_read: bool,
        allow_map_write: bool,
    ) -> Self {
        // Make vector from contents of initial_data
        let data = initial_data.into_iter().collect::<Vec<T>>();
        // Create a buffer from a full slice of the vector
        Self::from_slice(&data, allow_map_read, allow_map_write)
    }

    pub fn length(&self) -> GLsizeiptr {
        self.length
    }

    pub fn map(&self, range: impl RangeBounds<GLsizeiptr>) -> BufferMap<T> {
        if DEBUG {
            if (self.access_flags & gl::MAP_READ_BIT) == 0
                && (self.access_flags & gl::MAP_WRITE_BIT) == 0
            {
                panic!("Cannot read or write to the buffer");
            }
        }

        let range = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => 0,
        } * size_of::<T>() as isize..match range.end_bound() {
            Bound::Included(end) => *end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.length(),
        } * size_of::<T>() as isize;
        if DEBUG && range.end <= range.start {
            panic!("Range length is 0 or negative: {:?}", range);
        }

        let ptr = unsafe {
            gl::MapNamedBufferRange(
                self.gl_handle,
                range.start as GLintptr,
                range.end - range.start,
                self.access_flags,
            )
        };

        BufferMap::<T>::new(self, ptr, range, self.access_flags)
    }
}

impl<T: Sized + Copy> GLHandle for Buffer<T> {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl<T: Sized + Copy + Vertex + std::fmt::Debug + 'static> DynVertexBuffer for Buffer<T> {
    fn element_size(&self) -> GLsizei {
        size_of::<T>() as GLsizei
    }

    fn vertex_attribute_bindings(&self) -> &'static [VertexAttributeBinding] {
        T::vertex_attribute_bindings()
    }

    fn length(&self) -> GLsizeiptr {
        self.length
    }

    fn map<'a>(&'a self, range: Range<GLsizeiptr>) -> Box<dyn DynVertexBufferMap> {
        Box::new(self.map(range))
    }
}

impl<T: Sized + Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let deleted_buffers = [self.gl_handle];
            unsafe { gl::DeleteBuffers(1, deleted_buffers.as_ptr()) };
        }
    }
}

impl<T: Sized + Copy> FromIterator<T> for Buffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // Make vector from contents of initial_data
        let data = iter.into_iter().collect::<Vec<T>>();
        // Create a buffer from a full slice of the vector
        Self::from_slice(&data, false, false)
    }
}

fn choose_access_flags(allow_map_read: bool, allow_map_write: bool) -> GLenum {
    if allow_map_read && allow_map_write {
        gl::MAP_READ_BIT | gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
    } else if allow_map_read {
        gl::MAP_READ_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
    } else if allow_map_write {
        gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
    } else {
        0
    }
}
