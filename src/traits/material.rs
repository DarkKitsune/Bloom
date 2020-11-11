use crate::*;

pub trait Material {
    fn pipeline(&self) -> &Pipeline;
    fn pipeline_mut(&mut self) -> &mut Pipeline;
}
