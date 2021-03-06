use crate::*;
use fennec_algebra::*;
use std::collections::HashMap;
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

#[derive(Debug)]
pub struct Texture<const TYPE: crate::TextureType> {
    gl_handle: IntHandle,
    size: Vec2u,
    sprites: HashMap<String, Vec<Vec4f>>,
}

impl<const TYPE: crate::TextureType> Texture<TYPE> {
    pub fn new(size: Vec2u, count: i32) -> Vec<Self> {
        // Check that the width and height are powers of 2
        if DEBUG {
            if !size[0].is_power_of_2() {
                if !size[1].is_power_of_2() {
                    panic!("Dimensions are not a power of 2");
                }
                panic!("Width is not a power of 2");
            }
            if !size[1].is_power_of_2() {
                panic!("Height is not a power of 2");
            }
        }

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
            .drain(..)
            .map(|gl_handle| Self {
                gl_handle,
                size,
                sprites: HashMap::new(),
            })
            .collect()
    }

    pub fn from_bytes(bytes: &[u8], format: image::ImageFormat) -> Self {
        let image = image::load_from_memory_with_format(bytes, format).unwrap();
        let image = image.flipv().into_bgra();
        let mut tex = Self::new(vector!(image.width(), image.height()), 1)
            .pop()
            .unwrap();
        let data = image.into_raw();
        tex.set_data_bytes(&data);
        tex
    }

    pub fn from_file(path: impl AsRef<Path>, format: image::ImageFormat) -> Self {
        let file = BufReader::new(File::open(path).unwrap());
        let image = image::load(file, format).unwrap();
        let image = image.flipv().into_bgra();
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

    pub fn add_sprite_frames(&mut self, name: impl Into<String>, frames: impl Into<Vec<Vec4f>>) {
        self.sprites.insert(name.into(), frames.into());
    }

    pub fn sprite_frames(&self, name: impl AsRef<str>) -> &[Vec4f] {
        let name = name.as_ref();
        self.sprites
            .get(name)
            .unwrap_or_else(|| panic!("No sprite exists with name {:?}", name))
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
