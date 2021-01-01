use crate::*;
use fennec_algebra::*;
use std::cell::RefCell;
use std::rc::Rc;

const VERTEX_SHADER: &'static str = "
#[feature(camera)]
layout(location = 0) in vec2 i_position;
layout(location = 1) in vec2 i_scale;
layout(location = 2) in vec4 i_rotation;
layout(location = 3) in vec4 i_rectangle;
layout(location = 4) in vec2 v_position;
layout(location = 5) in vec2 v_texCoord;

layout(location = 0) out vec2 f_texCoord;
layout(location = 1) out vec4 f_rectangle;

out gl_PerVertex { vec4 gl_Position; };

void main()
{
    f_texCoord = v_texCoord;
    f_rectangle = i_rectangle;
    mat4 i_matrix = mat4(
        vec4(i_scale.x, 0.0, 0.0, 0.0),
        vec4(0.0, i_scale.y, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(i_position, 0.0, 1.0)
    );
    gl_Position = applyProjection(applyView(i_matrix * vec4(v_position, 0.0, 1.0)));
}";

const FRAGMENT_SHADER: &'static str = "
layout(location = 0) in vec2 f_texCoord;
layout(location = 1) in vec4 f_rectangle;

layout(location = 0) out vec4 out_color;

uniform sampler2D u_texture;

void main()
{
    vec2 texCoord = vec2(0.0, 1.0) + (f_rectangle.xy + f_texCoord * f_rectangle.zw) * vec2(1.0, -1.0);
    out_color = texture(u_texture, texCoord);
}";

#[derive(Clone, Debug)]
pub struct SpriteMaterial {
    pipeline: Rc<Pipeline>,
    texture: Option<Rc<Texture<{ TextureType::Texture2D }>>>,
    vertex_buffer: VertexBufferBinding,
    index_buffer: Rc<Buffer<GLuint>>,
}

impl SpriteMaterial {
    pub fn new() -> Self {
        let stages = vec![
            Program::new(ShaderStage::Vertex, VERTEX_SHADER),
            Program::new(ShaderStage::Fragment, FRAGMENT_SHADER),
        ];

        let pipeline = Rc::new(Pipeline::new(stages));

        let vertices = [
            Pos2TexVertex::new(vector!(-0.5, -0.5), vector!(0.0, 0.0)),
            Pos2TexVertex::new(vector!(0.5, -0.5), vector!(1.0, 0.0)),
            Pos2TexVertex::new(vector!(0.5, 0.5), vector!(1.0, 1.0)),
            Pos2TexVertex::new(vector!(-0.5, 0.5), vector!(0.0, 1.0)),
        ];
        let indices = [0, 1, 2, 0, 2, 3];
        let vertex_buffer = VertexBufferBinding::new(
            Rc::new(RefCell::new(Box::new(Buffer::from_slice(
                &vertices, false, false,
            )))),
            0,
        );
        let index_buffer = Rc::new(Buffer::from_slice(&indices, false, false));

        Self {
            pipeline,
            texture: None,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn texture(&self) -> Option<&Rc<Texture<{ TextureType::Texture2D }>>> {
        self.texture.as_ref()
    }

    pub fn set_texture(&mut self, texture: Rc<Texture<{ TextureType::Texture2D }>>) {
        self.texture = Some(texture);
    }

    pub fn get_vertex_index_buffers(&self) -> (VertexBufferBinding, Rc<Buffer<GLuint>>) {
        (self.vertex_buffer.clone(), self.index_buffer.clone())
    }
}

impl Material for SpriteMaterial {
    fn pipeline(&self) -> &Rc<Pipeline> {
        &self.pipeline
    }

    fn vertex_attribute_bindings(&self) -> Vec<Vec<VertexAttributeBinding>> {
        vec![
            vec![
                VertexAttributeBinding::Float2,
                VertexAttributeBinding::Float2,
                VertexAttributeBinding::Float4,
                VertexAttributeBinding::Float4,
            ],
            vec![
                VertexAttributeBinding::Float2,
                VertexAttributeBinding::Float2,
            ],
        ]
    }

    fn _on_bind(&self) {
        let frag_program = self.pipeline().fragment_program();
        let texture_location = frag_program.uniform_location("u_texture");
        if let Some(texture_location) = texture_location {
            frag_program.set_uniform_texture_unit(texture_location, 0);
            unsafe {
                gl::BindTextureUnit(0, self.texture.as_ref().expect("Texture not set").handle())
            };
        }
    }
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self::new()
    }
}
