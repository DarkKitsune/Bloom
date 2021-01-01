use crate::*;
use std::cell::RefCell;
use std::mem::size_of;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct VertexBufferBinding {
    buffer: Rc<RefCell<Buffer>>,
    divisor: GLuint,
    vertex_attribute_bindings: &'static [VertexAttributeBinding],
}

impl VertexBufferBinding {
    pub fn new<T: Vertex>(buffer: Rc<RefCell<Buffer>>, divisor: GLuint) -> Self {
        let vertex_attribute_bindings = T::vertex_attribute_bindings();
        Self {
            buffer,
            divisor,
            vertex_attribute_bindings,
        }
    }

    pub fn buffer(&self) -> &Rc<RefCell<Buffer>> {
        &self.buffer
    }

    pub fn divisor(&self) -> GLuint {
        self.divisor
    }

    pub fn vertex_attribute_bindings(&self) -> &'static [VertexAttributeBinding] {
        self.vertex_attribute_bindings
    }
}

#[derive(Debug)]
pub struct VertexArray {
    gl_handle: IntHandle,
    vertex_buffer_bindings: Vec<VertexBufferBinding>,
    index_buffer: Rc<Buffer>,
}

impl VertexArray {
    pub fn new(
        vertex_buffer_bindings: impl IntoIterator<Item = VertexBufferBinding>,
        index_buffer: Rc<Buffer>,
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
                    binding.buffer().borrow().handle(),
                    0,
                    binding.buffer().borrow().element_size() as GLint,
                );
            }

            // Bind index buffer to vertex array
            gl::VertexArrayElementBuffer(gl_handle, index_buffer.handle());

            // Next we will loop through all the vertex buffer bindings
            let mut attribute_idx = 0;
            for (binding_idx, binding) in vertex_buffer_bindings.iter().enumerate() {
                // Set the divisor for this binding
                gl::VertexArrayBindingDivisor(gl_handle, binding_idx as GLuint, binding.divisor());

                // Next we will loop through all of the vertex attribute bindings provided by this binding's vertex buffer
                let mut offset = 0;
                for &binding in binding.vertex_attribute_bindings().iter() {
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
                        VertexAttributeBinding::Float => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                1,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<f32>() as GLuint;
                        }
                        VertexAttributeBinding::Float2 => {
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
                        VertexAttributeBinding::Float3 => {
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
                        VertexAttributeBinding::Float4 => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                4,
                                gl::FLOAT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<Vec4f>() as GLuint;
                        }
                        VertexAttributeBinding::Mat4f => {
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
                        VertexAttributeBinding::Int => {
                            gl::VertexArrayAttribFormat(
                                gl_handle,
                                attribute_idx,
                                1,
                                gl::INT,
                                gl::FALSE,
                                offset,
                            );
                            offset += size_of::<GLint>() as GLuint;
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
            .filter(|binding| binding.divisor() > 0)
            .map(|binding| binding.buffer().borrow().length() * binding.divisor() as GLsizeiptr)
            .min()
    }

    pub fn vertex_buffer_bindings(&self) -> &[VertexBufferBinding] {
        &self.vertex_buffer_bindings
    }

    pub fn vertex_buffer_bindings_mut(&mut self) -> &mut [VertexBufferBinding] {
        &mut self.vertex_buffer_bindings
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
