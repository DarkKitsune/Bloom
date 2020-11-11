use crate::*;
use std::ffi::CString;

pub struct HelloTriangleMaterial {
    pipeline: Pipeline,
}

impl HelloTriangleMaterial {
    pub fn new() -> Self {
        // Create vertex shader code
        let vertex_shader = CString::new(
            "
#version 450

layout(location = 0) in mat4 i_matrix;
layout(location = 4) in vec3 v_position;
layout(location = 5) in vec3 v_color;

layout(location = 0) out vec4 f_color;

out gl_PerVertex { vec4 gl_Position; };

layout(location = 0) uniform mat4 view;
layout(location = 5) uniform mat4 projection;

void main()
{
    f_color = vec4(v_color, 1.0);
    gl_Position = projection * view * i_matrix * vec4(v_position, 1.0);
}",
        )
        .unwrap();

        // Create fragment shader code
        let fragment_shader = CString::new(
            "
#version 450

layout(location = 0) in vec4 f_color;

layout(location = 0) out vec4 out_color;

void main()
{
    out_color = f_color;
}",
        )
        .unwrap();

        let stages = vec![
            Program::new(ShaderStage::Vertex, &vertex_shader),
            Program::new(ShaderStage::Fragment, &fragment_shader),
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
