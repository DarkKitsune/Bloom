use crate::*;
use fennec_algebra::*;
use std::mem::size_of;

pub struct VertexBufferBinding {
    buffer: Box<dyn DynVertexBuffer>,
    divisor: GLuint,
}

impl VertexBufferBinding {
    pub fn new(buffer: Box<dyn DynVertexBuffer>, divisor: GLuint) -> Self {
        Self { buffer, divisor }
    }
}

pub struct VertexArray {
    gl_handle: IntHandle,
    vertex_buffers: Vec<VertexBufferBinding>,
    index_buffer: Buffer<GLuint>,
}

impl VertexArray {
    pub fn new(
        vertex_buffers: impl IntoIterator<Item = VertexBufferBinding>,
        index_buffer: Buffer<GLuint>,
    ) -> Self {
        let vertex_buffers = vertex_buffers
            .into_iter()
            .collect::<Vec<VertexBufferBinding>>();
        // We will receive the vertex array's handle in gl_handle
        let mut gl_handle: IntHandle = 0;
        unsafe {
            // Create vertex array
            gl::CreateVertexArrays(1, &mut gl_handle);
            for (binding_idx, buffer) in vertex_buffers.iter().enumerate() {
                gl::VertexArrayVertexBuffer(
                    gl_handle,
                    binding_idx as GLuint,
                    buffer.buffer.handle(),
                    0,
                    buffer.buffer.element_size(),
                );
            }
            gl::VertexArrayElementBuffer(gl_handle, index_buffer.handle());

            let mut attribute_idx = 0;
            for (binding_idx, buffer) in vertex_buffers.iter().enumerate() {
                gl::VertexArrayBindingDivisor(gl_handle, binding_idx as GLuint, buffer.divisor);
                let mut offset = 0;
                for &binding in buffer.buffer.vertex_attribute_bindings().iter() {
                    gl::EnableVertexArrayAttrib(gl_handle, attribute_idx);
                    gl::VertexArrayAttribBinding(gl_handle, attribute_idx, binding_idx as GLuint);
                    match binding {
                        VertexAttributeBinding::PositionF3 => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vector<f32, 3>>() as GLuint;
                        }
                        VertexAttributeBinding::NormalF3 => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vector<f32, 3>>() as GLuint;
                        }
                        VertexAttributeBinding::ColorF3 => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vector<f32, 3>>() as GLuint;
                        }
                        VertexAttributeBinding::TexCoordF2 => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                2,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vector<f32, 2>>() as GLuint;
                        }
                    }
                    attribute_idx += 1;
                }
            }
        }

        Self {
            gl_handle,
            vertex_buffers,
            index_buffer,
        }
    }

    pub fn index_count(&self) -> GLsizeiptr {
        self.index_buffer.length()
    }

    pub fn max_instance_count(&self) -> Option<GLsizeiptr> {
        self.vertex_buffers
            .iter()
            .filter(|binding| binding.divisor > 0)
            .map(|binding| binding.buffer.length() * binding.divisor as GLsizeiptr)
            .max()
    }
}

impl GLHandle for VertexArray {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let deleted_arrays = [self.gl_handle];
            unsafe { gl::DeleteVertexArrays(1, deleted_arrays.as_ptr()) };
        }
    }
}
