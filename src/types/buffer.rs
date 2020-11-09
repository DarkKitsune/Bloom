use crate::*;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem::size_of;

/*
pub struct BufferMap<'a, T: Sized> {
    buffer: &'a mut Buffer<T>,
}

impl<'a, T: Sized> BufferMap<'a, T> {
    pub fn unmap(self) {}
}

impl<'a, T: Sized> Drop for BufferMap<'a, T> {
    fn drop(&mut self) {
        // TODO: Make unmap
    }
}*/

pub struct Buffer<T: Sized> {
    _phantom_data: PhantomData<T>,
    gl_handle: IntHandle,
    _allow_map: bool, // TODO: check before mapping
    length: GLsizeiptr,
}

impl<T: Sized> Buffer<T> {
    pub fn new(length: GLsizeiptr, allow_map: bool) -> Self {
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
                if allow_map {
                    gl::MAP_READ_BIT | gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT
                } else {
                    0
                },
            );
        }

        Self {
            _phantom_data: PhantomData,
            gl_handle,
            _allow_map: allow_map,
            length,
        }
    }

    pub fn from_slice(initial_data: &[T], allow_map: bool) -> Self {
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
                if allow_map {
                    gl::MAP_READ_BIT | gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT
                } else {
                    0
                },
            );
        }

        Self {
            _phantom_data: PhantomData,
            gl_handle,
            _allow_map: allow_map,
            length: initial_data.len() as GLsizeiptr,
        }
    }

    pub fn from_iterator(initial_data: impl IntoIterator<Item = T>, allow_map: bool) -> Self {
        // Make vector from contents of initial_data
        let data = initial_data.into_iter().collect::<Vec<T>>();
        // Create a buffer from a full slice of the vector
        Self::from_slice(&data, allow_map)
    }

    pub fn length(&self) -> GLsizeiptr {
        self.length
    }
}

impl<T: Sized> GLHandle for Buffer<T> {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl<T: Sized + Vertex> DynVertexBuffer for Buffer<T> {
    fn element_size(&self) -> GLsizei {
        size_of::<T>() as GLsizei
    }

    fn vertex_attribute_bindings(&self) -> &'static [VertexAttributeBinding] {
        T::vertex_attribute_bindings()
    }

    fn length(&self) -> GLsizeiptr {
        self.length
    }
}

impl<T: Sized> Drop for Buffer<T> {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let deleted_buffers = [self.gl_handle];
            unsafe { gl::DeleteBuffers(1, deleted_buffers.as_ptr()) };
        }
    }
}

impl<T: Sized> FromIterator<T> for Buffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // Make vector from contents of initial_data
        let data = iter.into_iter().collect::<Vec<T>>();
        // Create a buffer from a full slice of the vector
        Self::from_slice(&data, false)
    }
}
