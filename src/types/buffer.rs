use crate::*;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Bound, Index, IndexMut, Range, RangeBounds};

pub struct BufferMap<T: Sized> {
    _phantom_data: PhantomData<T>,
    buffer: IntHandle,
    ptr: *mut T,
    range: Range<GLsizeiptr>,
    access_flags: GLenum,
}

impl<T: Sized> BufferMap<T> {
    fn new(buffer: &Buffer, ptr: *mut T, range: Range<GLsizeiptr>, access_flags: GLenum) -> Self {
        Self {
            _phantom_data: PhantomData,
            buffer: buffer.handle(),
            ptr,
            range,
            access_flags,
        }
    }

    fn element_ptr(&self, idx: usize) -> *const T {
        if DEBUG && idx > (self.range.end - self.range.start) as usize {
            panic!("Offset is outside mapped range {:?}", self.range);
        }

        unsafe { self.ptr.add(idx) }
    }

    fn element_ptr_mut(&mut self, idx: usize) -> *mut T {
        if DEBUG && idx > (self.range.end - self.range.start) as usize {
            panic!("Offset is outside mapped range {:?}", self.range);
        }

        unsafe { self.ptr.add(idx) }
    }

    pub fn unmap(self) {}

    pub fn can_read(&self) -> bool {
        (self.access_flags & gl::MAP_READ_BIT) > 0
    }

    pub fn can_write(&self) -> bool {
        (self.access_flags & gl::MAP_WRITE_BIT) > 0
    }
}

impl<T: Sized> Drop for BufferMap<T> {
    fn drop(&mut self) {
        unsafe { gl::UnmapNamedBuffer(self.buffer) };
    }
}

impl<T: Sized> Index<usize> for BufferMap<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        if DEBUG && !self.can_read() {
            panic!("Buffer access flags do not allow reading");
        }
        unsafe { &*self.element_ptr(idx) }
    }
}

impl<T: Sized> IndexMut<usize> for BufferMap<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        if DEBUG && !self.can_write() {
            panic!("Buffer access flags do not allow writing");
        }
        unsafe { &mut *self.element_ptr_mut(idx) }
    }
}

#[derive(Debug)]
pub struct Buffer {
    gl_handle: IntHandle,
    access_flags: GLenum,
    length: GLsizeiptr,
    element_size: usize,
}

impl Buffer {
    pub fn new<T: Sized>(length: GLsizeiptr, allow_map_read: bool, allow_map_write: bool) -> Self {
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
            gl_handle,
            access_flags,
            length,
            element_size: size_of::<T>(),
        }
    }

    pub fn from_slice<T: Sized>(
        initial_data: &[T],
        allow_map_read: bool,
        allow_map_write: bool,
    ) -> Self {
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
            gl_handle,
            access_flags,
            length: initial_data.len() as GLsizeiptr,
            element_size: size_of::<T>(),
        }
    }

    pub fn from_iterator<T: Sized>(
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

    pub fn element_size(&self) -> usize {
        self.element_size
    }

    pub fn map<T: Sized>(&self, range: impl RangeBounds<GLsizeiptr>) -> BufferMap<T> {
        if DEBUG {
            if (self.access_flags & gl::MAP_READ_BIT) == 0
                && (self.access_flags & gl::MAP_WRITE_BIT) == 0
            {
                panic!("Cannot read or write to the buffer");
            }
            if std::mem::size_of::<T>() != self.element_size {
                panic!(
                    "Size of T is not the same as the size of T which the buffer was created with"
                );
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

        BufferMap::<T>::new(self, ptr as *mut T, range, self.access_flags)
    }
}

impl GLHandle for Buffer {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let deleted_buffers = [self.gl_handle];
            unsafe { gl::DeleteBuffers(1, deleted_buffers.as_ptr()) };
        }
    }
}

impl<T: Sized> FromIterator<T> for Buffer {
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
