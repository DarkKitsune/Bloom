use crate::*;
use std::rc::Rc;

pub enum VerificationFailure {
    BufferCount(usize),
    BufferAttributeCount(usize, usize),
    Attribute(usize, usize, VertexAttributeBinding),
}

pub trait Material: std::fmt::Debug {
    fn pipeline(&self) -> &Rc<Pipeline>;
    fn vertex_attribute_bindings(&self) -> Vec<Vec<VertexAttributeBinding>>;
    fn _on_bind(&self);

    fn verify_vertex_array(&self, vertex_array: &VertexArray) -> Option<VerificationFailure> {
        let own_bindings = self.vertex_attribute_bindings();
        if own_bindings.len() != vertex_array.vertex_buffer_bindings().len() {
            return Some(VerificationFailure::BufferCount(own_bindings.len()));
        }
        for (buffer_idx, (a, b)) in own_bindings
            .iter()
            .zip(vertex_array.vertex_buffer_bindings().iter())
            .enumerate()
        {
            let b = b.vertex_attribute_bindings();
            if a.len() != b.len() {
                return Some(VerificationFailure::BufferAttributeCount(
                    buffer_idx,
                    a.len(),
                ));
            }
            for (attribute_idx, (binding_a, binding_b)) in a.iter().zip(b.iter()).enumerate() {
                if binding_a != binding_b {
                    return Some(VerificationFailure::Attribute(
                        buffer_idx,
                        attribute_idx,
                        *binding_a,
                    ));
                }
            }
        }
        None
    }

    fn bind(&self, vertex_array: &VertexArray) {
        if DEBUG {
            if let Some(failure) = self.verify_vertex_array(vertex_array) {
                let failure_reason = match failure {
                    VerificationFailure::BufferCount(count) => {
                        format!("Buffer count should be {}", count)
                    }
                    VerificationFailure::BufferAttributeCount(buffer_idx, count) => format!(
                        "# of attributes in buffer {} should be {}",
                        buffer_idx, count
                    ),
                    VerificationFailure::Attribute(buffer_idx, attribute_idx, attribute_type) => {
                        format!(
                            "attribute {} in buffer {} should have the type {:?}",
                            attribute_idx, buffer_idx, attribute_type
                        )
                    }
                };
                panic!(
                    "Vertex array not suitable for the material; {}",
                    failure_reason
                );
            }
        }

        unsafe {
            gl::BindVertexArray(vertex_array.handle());
            gl::BindProgramPipeline(self.pipeline().handle());
            self._on_bind();
        }
    }
}
