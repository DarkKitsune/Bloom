use crate::*;
use fennec_algebra::*;
use std::ffi::{c_void, CString};

//const MAX_DEBUG_MESSAGES: usize = 10;
//const MAX_DEBUG_MESSAGES_SIZE: usize = MAX_DEBUG_MESSAGES * 256;

pub struct GFX {}

impl GFX {
    pub fn new(main_window: &mut Window) -> Self {
        // Load GL function pointers
        gl::load_with(|s| main_window.proc_address(s) as *const _);

        // Set GL settings
        // Enable debug output
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(debug_message_callback), std::ptr::null());
        }

        Self {}
    }

    pub fn clear_color(&self, framebuffer: &mut Framebuffer, clear_color: &Vector<f32, 4>) {
        unsafe {
            gl::ClearNamedFramebufferfv(
                framebuffer.handle(),
                gl::COLOR,
                0,
                clear_color as *const Vector<f32, 4> as *const _,
            )
        };
    }

    /*pub fn use_pipeline(&self, pipeline: &Pipeline) {
        gl::BindProgramPipeline(pipeline.handle());
    }*/

    pub fn draw_indices(
        &self,
        material: &impl Material,
        vertex_array: &VertexArray,
        instance_count: GLsizei,
    ) {
        unsafe {
            if instance_count == 1 {
                panic!("Must draw at least one instance");
            }
            if let Some(max_instances) = vertex_array.max_instance_count() {
                if instance_count as GLsizeiptr > max_instances {
                    panic!("Given instance count is too large to draw; at least one vertex buffer with an instance input rate is not large enough");
                }
            }
            gl::BindProgramPipeline(material.pipeline().handle());
            gl::BindVertexArray(vertex_array.handle());
            gl::DrawArraysInstanced(
                gl::TRIANGLE_STRIP,
                0,
                vertex_array.index_count() as GLsizei,
                instance_count,
            );
            gl::BindVertexArray(0);
            gl::BindProgramPipeline(0);
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
