use crate::*;
use std::rc::Rc;

#[derive(Debug)]
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

    pub fn vertex_program(&self) -> &Program {
        self.stages
            .iter()
            .find(|e| e.stage() == ShaderStage::Vertex)
            .expect("Pipeline does not have a vertex stage")
    }

    pub fn vertex_program_mut(&mut self) -> &mut Program {
        self.stages
            .iter_mut()
            .find(|e| e.stage() == ShaderStage::Vertex)
            .expect("Pipeline does not have a vertex stage")
    }

    pub fn fragment_program(&self) -> &Program {
        self.stages
            .iter()
            .find(|e| e.stage() == ShaderStage::Fragment)
            .expect("Pipeline does not have a fragment stage")
    }

    pub fn fragment_program_mut(&mut self) -> &mut Program {
        self.stages
            .iter_mut()
            .find(|e| e.stage() == ShaderStage::Fragment)
            .expect("Pipeline does not have a fragment stage")
    }

    pub fn view_uniform_location(&self) -> GLuint {
        self.vertex_program()
            .uniform_location(FEATURE_CAMERA_VIEW_UNIFORM_NAME)
            .expect("Vertex shader does not use the 'camera' feature")
    }

    pub fn projection_uniform_location(&self) -> GLuint {
        self.vertex_program()
            .uniform_location(FEATURE_CAMERA_PROJECTION_UNIFORM_NAME)
            .expect("Vertex shader does not use the 'camera' feature")
    }

    pub fn transform_uniform_location(&self) -> GLuint {
        self.vertex_program()
            .uniform_location(FEATURE_TRANSFORM_UNIFORM_NAME)
            .expect("Vertex shader does not use the 'transform' feature")
    }

    pub fn shader_features(&self) -> Vec<ShaderFeature> {
        self.stages
            .iter()
            .map(|stage| stage.shader_features().iter().cloned())
            .flatten()
            .collect()
    }

    pub fn has_shader_feature(&self, feature: ShaderFeature) -> bool {
        self.stages
            .iter()
            .map(|stage| stage.shader_features().iter().cloned())
            .flatten()
            .find(|&e| e == feature)
            .is_some()
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
