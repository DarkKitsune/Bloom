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
    vertex_buffer_bindings: Vec<VertexBufferBinding>,
    index_buffer: Buffer<GLuint>,
}

impl VertexArray {
    pub fn new(
        vertex_buffer_bindings: impl IntoIterator<Item = VertexBufferBinding>,
        index_buffer: Buffer<GLuint>,
    ) -> Self {
        // Collect vertex buffers into a vector
        let vertex_buffer_bindings = vertex_buffer_bindings
            .into_iter()
            .collect::<Vec<VertexBufferBinding>>();

        // We will receive the vertex array's handle in gl_handle
        let mut gl_handle: IntHandle = 0;
        unsafe {
            // Create vertex array
            gl::CreateVertexArrays(1, &mut gl_handle);

            // Bind vertex buffers to vertex array
            for (binding_idx, binding) in vertex_buffer_bindings.iter().enumerate() {
                gl::VertexArrayVertexBuffer(
                    gl_handle,
                    binding_idx as GLuint,
                    binding.buffer.handle(),
                    0,
                    binding.buffer.element_size(),
                );
            }

            // Bind index buffer to vertex array
            gl::VertexArrayElementBuffer(gl_handle, index_buffer.handle());

            // Next we will loop through all the vertex buffer bindings
            let mut attribute_idx = 0;
            for (binding_idx, binding) in vertex_buffer_bindings.iter().enumerate() {
                // Set the divisor for this binding
                gl::VertexArrayBindingDivisor(gl_handle, binding_idx as GLuint, binding.divisor);

                // Next we will loop through all of the vertex attribute bindings provided by this binding's vertex buffer
                let mut offset = 0;
                for &binding in binding.buffer.vertex_attribute_bindings().iter() {
                    for add in 0..binding.locations_used() {
                        // Enable this vertex attribute at the next unused location (attribute_idx)
                        gl::EnableVertexArrayAttrib(gl_handle, attribute_idx + add);

                        // Set this vertex attribute to use the current bertex buffer binding to get its data
                        gl::VertexArrayAttribBinding(
                            gl_handle,
                            attribute_idx + add,
                            binding_idx as GLuint,
                        );
                    }

                    // Set the format for this vertex attribute and increment offset based on the format's size
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
                            offset += size_of::<Vec3f>() as GLuint;
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
                            offset += size_of::<Vec3f>() as GLuint;
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
                            offset += size_of::<Vec3f>() as GLuint;
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
                            offset += size_of::<Vec2f>() as GLuint;
                        }
                        VertexAttributeBinding::Transform => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                4,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vec4f>() as GLuint;
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx + 1,
                                4,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vec4f>() as GLuint;
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx + 2,
                                4,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vec4f>() as GLuint;
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx + 3,
                                4,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vec4f>() as GLuint;
                        }
                    }

                    // Increment attribute_idx to use the next unused location for the next vertex attribute binding
                    attribute_idx += binding.locations_used();
                }
            }
        }

        Self {
            gl_handle,
            vertex_buffer_bindings,
            index_buffer,
        }
    }

    pub fn index_count(&self) -> GLsizeiptr {
        self.index_buffer.length()
    }

    pub fn max_instance_count(&self) -> Option<GLsizeiptr> {
        self.vertex_buffer_bindings
            .iter()
            .filter(|binding| binding.divisor > 0)
            .map(|binding| binding.buffer.length() * binding.divisor as GLsizeiptr)
            .min()
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
