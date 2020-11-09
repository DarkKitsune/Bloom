use crate::*;
use std::ffi::CStr;

pub struct Pipeline {
    gl_handle: IntHandle,
    _stages: Vec<Program>,
}

impl Pipeline {
    pub fn new(stages: impl IntoIterator<Item = Program>) -> Self {
        let stages = stages.into_iter().collect::<Vec<Program>>();
        // Create the shader program object
        let mut gl_handle = Default::default();
        unsafe { gl::CreateProgramPipelines(1, &mut gl_handle as *mut _) };
        for stage in stages.iter() {
            unsafe { gl::UseProgramStages(gl_handle, stage.stage().stage_bit(), stage.handle()) };
        }
        Self {
            gl_handle,
            _stages: stages,
        }
    }
}

impl GLHandle for Pipeline {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let handles = [self.gl_handle];
            unsafe { gl::DeleteProgramPipelines(1, handles.as_ptr()) };
        }
    }
}
