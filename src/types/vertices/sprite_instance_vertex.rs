use crate::*;
use fennec_algebra::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 2] = [
    VertexAttributeBinding::Mat4f,
    VertexAttributeBinding::Float4,
];

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteInstanceVertex {
    transform: Mat4f,
    rectangle: Vec4f,
}

impl SpriteInstanceVertex {
    pub fn new(rectangle: Vec4f) -> Self {
        Self {
            transform: Mat4f::identity(),
            rectangle,
        }
    }

    pub fn with_position(mut self, position: Vec3f) -> Self {
        self.transform.set_position(position);
        self
    }

    pub fn with_scale(mut self, scale: Vec3f) -> Self {
        self.transform
            .set_scale(self.transform.scale().unwrap() * scale);
        self
    }

    pub fn with_rotation(mut self, rotation: Quaternion) -> Self {
        self.transform = Mat4f::new_rotation(&rotation).unwrap() * self.transform;
        self
    }

    pub fn with_rectangle(mut self, rectangle: Vec4f) -> Self {
        self.rectangle = rectangle;
        self
    }
}

impl Vertex for SpriteInstanceVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
