use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureType {
    Texture2D,
}

impl TextureType {
    pub fn gl_enum(self) -> gl::types::GLenum {
        match self {
            TextureType::Texture2D => gl::TEXTURE_2D,
        }
    }
}

pub struct Texture<const TYPE: crate::TextureType> {
    gl_handle: IntHandle,
    _size: Vec2u,
}

impl<const TYPE: crate::TextureType> Texture<TYPE> {
    fn _new<const COUNT: usize>(size: Vec2u) -> [Self; COUNT] {
        // Create handle array
        let mut handles = [Default::default(); COUNT];

        // Fill the handle array with new handles
        unsafe { gl::CreateTextures(TYPE.gl_enum(), COUNT as i32, handles.as_mut_ptr()) };

        // Wrap the handles and return the wrappers
        handles
            .iter()
            .map(|&gl_handle| Self {
                gl_handle,
                _size: size,
            })
            .collect_array()
    }
}

impl<const TYPE: crate::TextureType> GLHandle for Texture<TYPE> {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}
