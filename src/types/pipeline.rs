use crate::*;

pub struct Pipeline {
    gl_handle: IntHandle,
    stages: Vec<Program>,
}

impl Pipeline {
    pub fn new(stages: impl IntoIterator<Item = Program>) -> Self {
        // Gather the stages (programs) into a vector for storing later
        let stages = stages.into_iter().collect::<Vec<Program>>();

        // Create program pipeline unifying the stages
        let mut gl_handle = Default::default();
        unsafe { gl::CreateProgramPipelines(1, &mut gl_handle as *mut _) };

        // Tell the pipeline to use the stages
        for stage in stages.iter() {
            unsafe { gl::UseProgramStages(gl_handle, stage.stage().stage_bit(), stage.handle()) };
        }
        
        Self { gl_handle, stages }
    }

    pub fn vertex_program(&self) -> Option<&Program> {
        self.stages
            .iter()
            .find(|e| e.stage() == ShaderStage::Vertex)
    }

    pub fn vertex_program_mut(&mut self) -> Option<&mut Program> {
        self.stages
            .iter_mut()
            .find(|e| e.stage() == ShaderStage::Vertex)
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
