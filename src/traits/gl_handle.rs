pub use gl::types::GLuint as IntHandle;

pub trait GLHandle {
    fn handle(&self) -> IntHandle;
}
