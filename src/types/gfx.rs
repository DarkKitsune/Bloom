use crate::*;
use fennec_algebra::*;
use std::cell::RefCell;
use std::ffi::{c_void, CString};

//const MAX_DEBUG_MESSAGES: usize = 10;
//const MAX_DEBUG_MESSAGES_SIZE: usize = MAX_DEBUG_MESSAGES * 256;

pub struct GFX {
    view: Mat4f,
    projection: Mat4f,
    transform: Mat4f,
}

impl GFX {
    pub fn new(main_window: &mut Window) -> Self {
        // Load GL function pointers
        gl::load_with(|s| main_window.proc_address(s) as *const _);

        // Set GL settings
        // Enable debug output
        if DEBUG {
            unsafe {
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
                gl::DebugMessageCallback(Some(debug_message_callback), std::ptr::null());
            }
        }

        Self {
            view: Mat4f::identity(),
            projection: Mat4f::identity(),
            transform: Mat4f::identity(),
        }
    }

    pub fn clear_color(&mut self, framebuffer: &mut Framebuffer, clear_color: &Vec4f) {
        unsafe {
            gl::ClearNamedFramebufferfv(
                framebuffer.handle(),
                gl::COLOR,
                0,
                clear_color as *const Vec4f as *const _,
            );
        };
    }

    pub fn clear_depth_stencil(
        &mut self,
        framebuffer: &mut Framebuffer,
        clear_depth: Option<GLfloat>,
        clear_stencil: GLint,
    ) {
        unsafe {
            gl::ClearNamedFramebufferfi(
                framebuffer.handle(),
                gl::DEPTH_STENCIL,
                0,
                clear_depth.unwrap_or(1.0),
                clear_stencil,
            );
        };
    }

    /*pub fn use_pipeline(&self, pipeline: &Pipeline) {
        gl::BindProgramPipeline(pipeline.handle());
    }*/

    pub fn set_view(&mut self, view: Mat4f) {
        self.view = view;
    }

    pub fn view(&self) -> &Mat4f {
        &self.view
    }

    pub fn set_projection(&mut self, projection: Mat4f) {
        self.projection = projection;
    }

    pub fn projection(&self) -> &Mat4f {
        &self.projection
    }

    pub fn draw_indices(
        &mut self,
        material: &RefCell<dyn Material>,
        vertex_array: &VertexArray,
        instance_count: GLsizei,
        primitive_type: PrimitiveType,
    ) {
        let material = material.borrow();
        unsafe {
            if DEBUG {
                // Avoid trying to draw 0 instances
                if instance_count < 1 {
                    panic!("Must draw at least one instance");
                }

                // Don't allow drawing more instances than possible with the attached instance buffers (if any)
                if let Some(max_instances) = vertex_array.max_instance_count() {
                    if instance_count as GLsizeiptr > max_instances {
                        panic!("Given instance count is too large to draw; at least one vertex buffer with an instance input rate is not large enough");
                    }
                }
            }

            // Bind the material
            material.bind(vertex_array);

            // Set uniforms according to parameters in this gfx object
            if material
                .pipeline()
                .has_shader_feature(ShaderFeature::Camera)
            {
                // Get the location of the view matrix from the vertex program
                let view_uniform = material.pipeline().view_uniform_location();
                // Apply view matrix to view uniform
                let mats = [self.view];
                material
                    .pipeline()
                    .vertex_program()
                    .set_uniform_mat4f(view_uniform, &mats);

                // Get the location of the projection matrix from the vertex program
                let projection_uniform = material.pipeline().projection_uniform_location();
                // Apply projection matrix to projection uniform
                let mats = [self.projection];
                material
                    .pipeline()
                    .vertex_program()
                    .set_uniform_mat4f(projection_uniform, &mats);
            }
            if material
                .pipeline()
                .has_shader_feature(ShaderFeature::Transform)
            {
                // Get the location of the transform matrix from the vertex program
                let transform_uniform = material.pipeline().transform_uniform_location();
                // Apply view matrix to view uniform
                let mats = [self.transform];
                material
                    .pipeline()
                    .vertex_program()
                    .set_uniform_mat4f(transform_uniform, &mats);
            }

            // Draw using the attached buffers
            gl::DrawElementsInstanced(
                primitive_type.gl_primitive_mode(),
                vertex_array.index_count() as GLsizei,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                instance_count,
            );
        }
    }

    pub fn draw_indices_partial(
        &mut self,
        material: &RefCell<dyn Material>,
        vertex_array: &VertexArray,
        instance_count: GLsizei,
        primitive_type: PrimitiveType,
        index_count: GLsizei,
    ) {
        let material = material.borrow();
        unsafe {
            if DEBUG {
                // Avoid trying to draw 0 instances
                if instance_count < 1 {
                    panic!("Must draw at least one instance");
                }

                // Don't allow drawing more instances than possible with the attached instance buffers (if any)
                if let Some(max_instances) = vertex_array.max_instance_count() {
                    if instance_count as GLsizeiptr > max_instances {
                        panic!("Given instance count is too large to draw; at least one vertex buffer with an instance input rate is not large enough");
                    }
                }
            }

            // Bind the material
            material.bind(vertex_array);

            // Set uniforms according to parameters in this gfx object
            if material
                .pipeline()
                .has_shader_feature(ShaderFeature::Camera)
            {
                // Get the location of the view matrix from the vertex program
                let view_uniform = material.pipeline().view_uniform_location();
                // Apply view matrix to view uniform
                let mats = [self.view];
                material
                    .pipeline()
                    .vertex_program()
                    .set_uniform_mat4f(view_uniform, &mats);

                // Get the location of the projection matrix from the vertex program
                let projection_uniform = material.pipeline().projection_uniform_location();
                // Apply projection matrix to projection uniform
                let mats = [self.projection];
                material
                    .pipeline()
                    .vertex_program()
                    .set_uniform_mat4f(projection_uniform, &mats);
            }
            if material
                .pipeline()
                .has_shader_feature(ShaderFeature::Transform)
            {
                // Get the location of the transform matrix from the vertex program
                let transform_uniform = material.pipeline().transform_uniform_location();
                // Apply view matrix to view uniform
                let mats = [self.transform];
                material
                    .pipeline()
                    .vertex_program()
                    .set_uniform_mat4f(transform_uniform, &mats);
            }

            // Draw using the attached buffers
            gl::DrawElementsInstanced(
                primitive_type.gl_primitive_mode(),
                index_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                instance_count,
            );
        }
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, instance_count: GLsizei) {
        self.draw_indices(
            mesh.material(),
            mesh.vertex_array(),
            instance_count,
            mesh.primitive_type(),
        );
    }

    pub fn draw_mesh_partial(
        &mut self,
        mesh: &Mesh,
        instance_count: GLsizei,
        index_count: GLsizei,
    ) {
        self.draw_indices_partial(
            mesh.material(),
            mesh.vertex_array(),
            instance_count,
            mesh.primitive_type(),
            index_count,
        );
    }

    pub fn draw_model(&mut self, model: &Model, instance_count: GLsizei) {
        for mesh in model.meshes() {
            self.draw_mesh(mesh, instance_count);
        }
    }

    pub fn draw_model_partial(
        &mut self,
        model: &Model,
        instance_count: GLsizei,
        index_count: GLsizei,
    ) {
        for mesh in model.meshes() {
            self.draw_mesh_partial(mesh, instance_count, index_count);
        }
    }

    pub fn depth_test(&mut self, enabled: bool) {
        if enabled {
            unsafe {
                gl::Enable(gl::DEPTH_TEST);
            }
        } else {
            unsafe {
                gl::Disable(gl::DEPTH_TEST);
            }
        }
    }

    pub fn depth_write(&mut self, enabled: bool) {
        if enabled {
            unsafe {
                gl::DepthMask(gl::TRUE);
            }
        } else {
            unsafe {
                gl::DepthMask(gl::FALSE);
            }
        }
    }

    pub fn cull(&mut self, enabled: bool) {
        if enabled {
            unsafe {
                gl::Enable(gl::CULL_FACE);
            }
        } else {
            unsafe {
                gl::Disable(gl::CULL_FACE);
            }
        }
    }

    /*pub fn poll_debug_messages(&self) {
        let mut sources: [GLenum; MAX_DEBUG_MESSAGES] = [0; MAX_DEBUG_MESSAGES];
        let mut types: [GLenum; MAX_DEBUG_MESSAGES] = [0; MAX_DEBUG_MESSAGES];
        let mut ids: [GLuint; MAX_DEBUG_MESSAGES] = [0; MAX_DEBUG_MESSAGES];
        let mut severities: [GLenum; MAX_DEBUG_MESSAGES] = [0; MAX_DEBUG_MESSAGES];
        let mut lengths: [GLsizei; MAX_DEBUG_MESSAGES] = [0; MAX_DEBUG_MESSAGES];
        let mut messages: [GLchar; MAX_DEBUG_MESSAGES_SIZE] = [0; MAX_DEBUG_MESSAGES_SIZE];
        let message_count = unsafe { gl::GetDebugMessageLog(MAX_DEBUG_MESSAGES as GLuint, MAX_DEBUG_MESSAGES_SIZE as GLsizei, sources.as_mut_ptr(), types.as_mut_ptr(), ids.as_mut_ptr(), severities.as_mut_ptr(), lengths.as_mut_ptr(), messages.as_mut_ptr()) };
        let mut message_ptr = messages.as_ptr();
        for (idx, &length) in lengths.iter().take(message_count as usize).enumerate() {
            debug_message_callback(sources[idx], types[idx], ids[idx], severities[idx], length, message_ptr);
            message_ptr = unsafe { message_ptr.add(length as usize) };
        }
    }*/

    pub fn viewport(&mut self, viewport: Vec4i, set_scissor: bool) {
        unsafe { gl::Viewport(*viewport.x(), *viewport.y(), *viewport.z(), *viewport.w()) };
        if set_scissor {
            unsafe { gl::Scissor(*viewport.x(), *viewport.y(), *viewport.z(), *viewport.w()) };
        }
    }
}

extern "system" fn debug_message_callback(
    source: GLenum,
    type_: GLenum,
    id: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    let color = match severity {
        gl::DEBUG_SEVERITY_NOTIFICATION => "\x1B[34m",
        gl::DEBUG_SEVERITY_LOW => "\x1B[32m",
        gl::DEBUG_SEVERITY_MEDIUM => "\x1B[33m",
        gl::DEBUG_SEVERITY_HIGH => "\x1B[31m",
        _ => panic!("Severity {} unhandled!", severity),
    };
    let source_string = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD PARTY",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        _ => panic!("Source {} unhandled!", source),
    };
    let type_string = match type_ {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED BEHAVIOR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED BEHAVIOR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        gl::DEBUG_TYPE_OTHER => "OTHER",
        _ => panic!("Type {} unhandled!", type_),
    };
    let message_slice =
        unsafe { std::slice::from_raw_parts(message as *const u8, length as usize) };
    let message_vec = message_slice.to_owned();
    let message = CString::new(message_vec).unwrap();

    println!(
        "{}[{}][{}] #{} {}\x1B[37m",
        color,
        source_string,
        type_string,
        id,
        message.to_str().unwrap()
    );
    if severity == gl::DEBUG_SEVERITY_HIGH {
        panic!("GL error occurred!");
    }
}
