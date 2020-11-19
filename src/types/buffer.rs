use crate::*;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ffi::c_void;


pub struct BufferMap<T: Sized + Copy> {
    _phantom_data: PhantomData<T>,
    buffer: IntHandle,
    ptr: *mut c_void,
}

impl<T: Sized + Copy> BufferMap<T> {
    fn new(buffer: &Buffer<T>, ptr: *mut c_void) -> Self {
        Self {
            _phantom_data: PhantomData,
            buffer: buffer.handle(),
            ptr,
        }
    }

    pub fn unmap(self) {}
}

impl<T: Sized + Copy> Drop for BufferMap<T> {
    fn drop(&mut self) {
        unsafe { gl::UnmapNamedBuffer(self.buffer) };
    }
}

impl<T: Sized + Copy + Vertex> DynVertexBufferMap for BufferMap<T> {
    fn buffer_handle(&self) -> IntHandle {
        self.buffer
    }

    fn element_size(&self) -> usize {
        std::mem::size_of::<T>()
    }
    
    fn ptr(&self) -> *const c_void {
        self.ptr
    }
}

pub struct Buffer<T: Sized + Copy> {
    _phantom_data: PhantomData<T>,
    gl_handle: IntHandle,
    allow_map_read: bool,
    allow_map_write: bool,
    length: GLsizeiptr,
}

impl<T: Sized + Copy> Buffer<T> {
    pub fn new(length: GLsizeiptr, allow_map_read: bool, allow_map_write: bool,) -> Self {
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
                if allow_map_read && allow_map_write {
                    gl::MAP_READ_BIT | gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
                } else if allow_map_read {
                    gl::MAP_READ_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
                } else if allow_map_write {
                    gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
                } else {
                    0
                },
            );
        }

        Self {
            _phantom_data: PhantomData,
            gl_handle,
            allow_map_read,
            allow_map_write,
            length,
        }
    }

    pub fn from_slice(initial_data: &[T], allow_map_read: bool, allow_map_write: bool,) -> Self {
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
                if allow_map_read && allow_map_write {
                    gl::MAP_READ_BIT | gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
                } else if allow_map_read {
                    gl::MAP_READ_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
                } 
                else if allow_map_write {
                    gl::MAP_WRITE_BIT | gl::MAP_COHERENT_BIT | gl::MAP_PERSISTENT_BIT
                } else {
                    0
                },
            );
        }

        Self {
            _phantom_data: PhantomData,
            gl_handle,
            allow_map_read,
            allow_map_write,
            length: initial_data.len() as GLsizeiptr,
        }
    }

    pub fn from_iterator(initial_data: impl IntoIterator<Item = T>, allow_map_read: bool, allow_map_write: bool,) -> Self {
        // Make vector from contents of initial_data
        let data = initial_data.into_iter().collect::<Vec<T>>();
        // Create a buffer from a full slice of the vector
        Self::from_slice(&data, allow_map_read, allow_map_write)
    }

    pub fn length(&self) -> GLsizeiptr {
        self.length
    }

    pub fn map(&self) -> BufferMap<T> {
        if DEBUG {
            if !self.allow_map_read && !self.allow_map_write {
                panic!("Cannot read or write to the buffer");
            }
        }

        let ptr = unsafe { gl::MapNamedBuffer(
            self.gl_handle,
            if self.allow_map_read && self.allow_map_write {
                gl::READ_WRITE 
            }
            else {
                (if self.allow_map_read {
                    gl::READ_ONLY
                }
                else {
                    0
                }) |
                (if self.allow_map_write {
                    gl::WRITE_ONLY
                }
                else {
                    0
                })
            }
        ) };

        BufferMap::<T>::new(self, ptr)
    }
}

impl<T: Sized + Copy> GLHandle for Buffer<T> {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl<T: Sized + Copy + Vertex + 'static> DynVertexBuffer for Buffer<T> {
    fn element_size(&self) -> GLsizei {
        size_of::<T>() as GLsizei
    }

    fn vertex_attribute_bindings(&self) -> &'static [VertexAttributeBinding] {
        T::vertex_attribute_bindings()
    }

    fn length(&self) -> GLsizeiptr {
        self.length
    }

    fn map<'a>(&'a self) -> Box<dyn DynVertexBufferMap> {
        Box::new(self.map())
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
