use crate::*;

pub trait Material {
    fn pipeline(&self) -> &Pipeline;
}
