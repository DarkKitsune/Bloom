use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum PrimitiveType {
    TriangleList,
    PointList,
    LineList,
}

impl PrimitiveType {
    pub fn gl_primitive_mode(self) -> GLenum {
        match self {
            PrimitiveType::TriangleList => gl::TRIANGLES,
            PrimitiveType::PointList => gl::POINTS,
            PrimitiveType::LineList => gl::LINES,
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    material: Rc<RefCell<dyn Material>>,
    vertex_array: VertexArray,
    primitive_type: PrimitiveType,
}

impl Mesh {
    pub fn new(
        material: Rc<RefCell<dyn Material>>,
        vertex_array: VertexArray,
        primitive_type: PrimitiveType,
    ) -> Self {
        Self {
            material,
            vertex_array,
            primitive_type,
        }
    }

    pub fn material(&self) -> &Rc<RefCell<dyn Material>> {
        &self.material
    }

    pub fn vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }

    pub fn vertex_array_mut(&mut self) -> &mut VertexArray {
        &mut self.vertex_array
    }

    pub fn primitive_type(&self) -> PrimitiveType {
        self.primitive_type
    }
}
