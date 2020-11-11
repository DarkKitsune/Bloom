use crate::*;
use std::ffi::{CStr, CString};

const MAX_PROGRAM_INFO_LOG_SIZE: usize = 1024;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShaderStage {
    Vertex,
    Fragment,
}

impl ShaderStage {
    pub fn gl_enum(&self) -> GLenum {
        match self {
            ShaderStage::Vertex => gl::VERTEX_SHADER,
            ShaderStage::Fragment => gl::FRAGMENT_SHADER,
        }
    }

    pub fn stage_bit(&self) -> GLenum {
        match self {
            ShaderStage::Vertex => gl::VERTEX_SHADER_BIT,
            ShaderStage::Fragment => gl::FRAGMENT_SHADER_BIT,
        }
    }
}

pub struct Program {
    gl_handle: IntHandle,
    stage: ShaderStage,
}

impl Program {
    pub fn new(stage: ShaderStage, code: &CStr) -> Self {
        // Store pointer to code in an array so we can create a pointer to the pointer safely
        let code = [code.as_ptr() as *const _];
        // Create the shader program object
        let gl_handle = unsafe { gl::CreateShaderProgramv(stage.gl_enum(), 1, code.as_ptr()) };
        let mut length: GLsizei = 0;
        let mut info_log: [GLchar; MAX_PROGRAM_INFO_LOG_SIZE] = [0; MAX_PROGRAM_INFO_LOG_SIZE];
        unsafe {
            gl::GetProgramInfoLog(
                gl_handle,
                MAX_PROGRAM_INFO_LOG_SIZE as GLsizei,
                &mut length as *mut _,
                info_log.as_mut_ptr(),
            )
        };
        if length > 0 {
            let message_slice = unsafe {
                std::slice::from_raw_parts(info_log.as_ptr() as *const u8, length as usize)
            };
            let message_vec = message_slice.to_owned();
            let message = CString::new(message_vec).unwrap();
            println!("\x1B[35m{}\x1B[37m", message.to_str().unwrap());
        }

        Self { gl_handle, stage }
    }

    pub fn stage(&self) -> ShaderStage {
        self.stage
    }

    pub fn uniform_location(&self, name: impl Into<CString>) -> Option<GLuint> {
        let name = name.into();
        let location = unsafe { gl::GetUniformLocation(self.handle(), name.as_ptr()) };
        if location < 0 {
            None
        } else {
            Some(location as GLuint)
        }
    }

    pub fn set_uniform_mat4f(&self, location: GLuint, mats: &[Mat4f]) {
        unsafe {
            gl::ProgramUniformMatrix4fv(
                self.handle(),
                location as GLint,
                mats.len() as i32,
                gl::FALSE,
                mats.as_ptr() as *const _,
            )
        };
    }
}

impl GLHandle for Program {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            unsafe { gl::DeleteProgram(self.gl_handle) };
        }
    }
}
