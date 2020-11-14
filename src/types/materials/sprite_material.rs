use crate::*;
use fennec_algebra::*;

const VERTEX_SHADER: &'static str = "
#[feature(camera)]
layout(location = 0) in mat4 i_matrix;
layout(location = 4) in vec3 v_position;
layout(location = 5) in vec2 v_texCoord;

layout(location = 0) out vec2 f_texCoord;

out gl_PerVertex { vec4 gl_Position; };

void main()
{
    f_texCoord = v_texCoord;
    gl_Position = applyProjection(applyView(i_matrix * vec4(v_position, 1.0)));
}";

const FRAGMENT_SHADER: &'static str = "
layout(location = 0) in vec2 f_texCoord;

layout(location = 0) out vec4 out_color;

uniform sampler2D u_texture;

void main()
{
    out_color = texture(u_texture, f_texCoord);
}";

pub struct SpriteMaterial {
    pipeline: Pipeline,
    texture_handle: IntHandle,
}

impl SpriteMaterial {
    pub fn new() -> Self {
        let stages = vec![
            Program::new(ShaderStage::Vertex, VERTEX_SHADER),
            Program::new(ShaderStage::Fragment, FRAGMENT_SHADER),
        ];
        let pipeline = Pipeline::new(stages);
        Self {
            pipeline,
            texture_handle: 0,
        }
    }

    pub fn set_texture(&mut self, texture: &Texture<{ TextureType::Texture2D }>) {
        self.texture_handle = texture.handle();
    }

    pub fn create_vertex_index_buffers() -> (VertexBufferBinding, Buffer<GLuint>) {
        let vertices = [
            PosTexVertex::new(vector!(-0.5, -0.5, 0.0), vector!(0.0, 0.0)),
            PosTexVertex::new(vector!(0.5, -0.5, 0.0), vector!(1.0, 0.0)),
            PosTexVertex::new(vector!(0.5, 0.5, 0.0), vector!(1.0, 1.0)),
            PosTexVertex::new(vector!(-0.5, 0.5, 0.0), vector!(0.0, 1.0)),
        ];
        let indices = [0, 1, 2, 0, 2, 3];
        (
            VertexBufferBinding::new(Box::new(Buffer::from_slice(&vertices, false)), 0),
            Buffer::from_slice(&indices, false),
        )
    }
}

impl Material for SpriteMaterial {
    fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    fn vertex_attribute_bindings(&self) -> Vec<Vec<VertexAttributeBinding>> {
        vec![
            vec![VertexAttributeBinding::Transform],
            vec![
                VertexAttributeBinding::PositionF3,
                VertexAttributeBinding::TexCoordF2,
            ],
        ]
    }

    fn _on_bind(&self) {
        let frag_program = self.pipeline().fragment_program();
        let texture_location = frag_program.uniform_location("u_texture").unwrap();
        frag_program.set_uniform_texture_unit(texture_location, 0);

        unsafe { gl::BindTextureUnit(0, self.texture_handle) };
    }
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self::new()
    }
}
