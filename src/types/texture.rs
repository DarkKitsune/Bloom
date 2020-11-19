use crate::*;
use fennec_algebra::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
    size: Vec2u,
}

impl<const TYPE: crate::TextureType> Texture<TYPE> {
    pub fn new(size: Vec2u, count: i32) -> Vec<Self> {
        // Create handle array
        let mut handles = (0..count)
            .map(|_| Default::default())
            .collect::<Vec<IntHandle>>();

        // Fill the handle array with new handles
        unsafe { gl::CreateTextures(TYPE.gl_enum(), count as i32, handles.as_mut_ptr()) };

        for handle in handles.iter() {
            unsafe {
                gl::TextureStorage2D(
                    *handle,
                    1,
                    gl::RGBA8_SNORM,
                    size[0] as GLsizei,
                    size[1] as GLsizei,
                )
            };
        }

        // Wrap the handles and return the wrappers
        handles
            .iter()
            .map(|&gl_handle| Self {
                gl_handle,
                size: size,
            })
            .collect()
    }

    pub fn from_file(path: impl AsRef<Path>, format: image::ImageFormat) -> Self {
        let file = BufReader::new(File::open(path).unwrap());
        let image = image::load(file, format).unwrap();
        let image = image.into_bgra();
        let mut tex = Self::new(vector!(image.width(), image.height()), 1)
            .pop()
            .unwrap();
        let data = image.into_raw();
        tex.set_data_bytes(&data);
        tex
    }

    pub fn set_data(&self, bgra_data: &[Vector<u8, 4>]) {
        if DEBUG {
            let required_size = self.size[0] as usize * self.size[1] as usize;
            if bgra_data.len() != required_size {
                panic!(
                    "Data is not the correct size for this texture; expected {} pixels",
                    required_size
                );
            }
            unsafe {
                gl::TextureSubImage2D(
                    self.handle(),
                    0,
                    0,
                    0,
                    self.size[0] as GLsizei,
                    self.size[1] as GLsizei,
                    gl::BGRA,
                    gl::UNSIGNED_BYTE,
                    bgra_data.as_ptr() as *const _,
                )
            };
        }
    }

    pub fn set_data_bytes(&mut self, bgra_data: &[u8]) {
        if DEBUG {
            let required_size = self.size[0] as usize * self.size[1] as usize * 4;
            if bgra_data.len() != required_size {
                panic!(
                    "Data is not the correct size for this texture; expected {} bytes",
                    required_size
                );
            }
            unsafe {
                gl::TextureSubImage2D(
                    self.handle(),
                    0,
                    0,
                    0,
                    self.size[0] as GLsizei,
                    self.size[1] as GLsizei,
                    gl::BGRA,
                    gl::UNSIGNED_BYTE,
                    bgra_data.as_ptr() as *const _,
                )
            };
        }
    }

    pub fn size(&self) -> Vec2u {
        self.size
    }
}

impl<const TYPE: crate::TextureType> GLHandle for Texture<TYPE> {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl<const TYPE: crate::TextureType> Drop for Texture<TYPE> {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let deleted_textures = [self.gl_handle];
            unsafe { gl::DeleteTextures(1, deleted_textures.as_ptr()) };
        }
    }
}
