use crate::*;
use std::ffi::CString;

const VERTEX_SHADER: &'static str = "
#[feature(view, projection)]
layout(location = 0) in mat4 i_matrix;
layout(location = 4) in vec3 v_position;
layout(location = 5) in vec3 v_color;

layout(location = 0) out vec4 f_color;

out gl_PerVertex { vec4 gl_Position; };

void main()
{
    f_color = vec4(v_color, 1.0);
    gl_Position = applyProjection(applyView(i_matrix * vec4(v_position, 1.0)));
}";

const FRAGMENT_SHADER: &'static str = "
#version 450

layout(location = 0) in vec4 f_color;

layout(location = 0) out vec4 out_color;

void main()
{
    out_color = f_color;
}";

pub struct HelloTriangleMaterial {
    pipeline: Pipeline,
}

impl HelloTriangleMaterial {
    pub fn new() -> Self {
        let stages = vec![
            Program::new(ShaderStage::Vertex, VERTEX_SHADER),
            Program::new(ShaderStage::Fragment, FRAGMENT_SHADER),
        ];
        let pipeline = Pipeline::new(stages);
        Self { pipeline }
    }
}

impl Material for HelloTriangleMaterial {
    fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
}

impl Default for HelloTriangleMaterial {
    fn default() -> Self {
        Self::new()
    }
}
