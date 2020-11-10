use crate::*;
use fennec_algebra::*;

const VERTEX_ATTRIBUTE_BINDINGS: [VertexAttributeBinding; 1] = [VertexAttributeBinding::Transform];

#[repr(C)]
pub struct InstanceModelVertex {
    model: Mat4f,
}

impl InstanceModelVertex {
    pub fn new(position: Vec3f, scale: Vec3f) -> Self {
        Self {
            model: Mat4f::new_position_scale(position, scale).unwrap(),
        }
    }
}

impl Vertex for InstanceModelVertex {
    fn vertex_attribute_bindings() -> &'static [VertexAttributeBinding] {
        &VERTEX_ATTRIBUTE_BINDINGS
    }
}
