use crate::*;

#[derive(Debug)]
pub struct ComputePipeline {
    gl_handle: IntHandle,
    program: Program,
}

impl ComputePipeline {
    pub fn new(program: Program) -> Self {
        if DEBUG && program.stage() != ShaderStage::Compute {
            panic!(
                "Expected program to use the {:?} stage but it uses the {:?} stage",
                ShaderStage::Compute,
                program.stage()
            );
        }

        // Create program pipeline
        let mut gl_handle = Default::default();
        unsafe { gl::CreateProgramPipelines(1, &mut gl_handle as *mut _) };

        // Tell the pipeline to use the compute stage with this program
        unsafe { gl::UseProgramStages(gl_handle, program.stage().stage_bit(), program.handle()) };

        Self { gl_handle, program }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn program_mut(&mut self) -> &mut Program {
        &mut self.program
    }

    pub fn shader_features(&self) -> &[ShaderFeature] {
        self.program.shader_features()
    }

    pub fn has_shader_feature(&self, feature: ShaderFeature) -> bool {
        self.program.shader_features().iter().any(|&e| e == feature)
    }
}

impl GLHandle for ComputePipeline {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let handles = [self.gl_handle];
            unsafe { gl::DeleteProgramPipelines(1, handles.as_ptr()) };
        }
    }
}
