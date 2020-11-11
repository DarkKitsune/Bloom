use crate::*;

pub fn window_framebuffer() -> Framebuffer {
    Framebuffer {
        gl_handle: 0,
        _allow_draw: true,
        _allow_read: false,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AttachmentType {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil,
}

impl AttachmentType {
    pub fn gl_enum(self) -> gl::types::GLenum {
        match self {
            AttachmentType::Color(num) => gl::COLOR_ATTACHMENT0 + num,
            AttachmentType::Depth => gl::DEPTH_ATTACHMENT,
            AttachmentType::Stencil => gl::STENCIL_ATTACHMENT,
            AttachmentType::DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}

pub struct Framebuffer {
    gl_handle: IntHandle,
    _allow_draw: bool, // TODO: use these
    _allow_read: bool,
}

impl Framebuffer {
    fn _new<const COUNT: usize>(allow_draw: bool, allow_read: bool) -> [Self; COUNT] {
        // Create handle array
        let mut handles = [Default::default(); COUNT];

        // Fill the handle array with new handles
        unsafe { gl::CreateFramebuffers(COUNT as i32, handles.as_mut_ptr()) };

        // Wrap the handles and return the wrappers
        handles
            .iter()
            .map(|&gl_handle| {
                /*if allow_draw {
                    let status = unsafe { gl::CheckNamedFramebufferStatus(gl_handle, gl::DRAW_FRAMEBUFFER) };
                    if status != gl::FRAMEBUFFER_COMPLETE {
                        panic!("Framebuffer is not complete for drawing: {:?}", status);
                    }
                }
                if allow_read {
                    let status = unsafe { gl::CheckNamedFramebufferStatus(gl_handle, gl::READ_FRAMEBUFFER) };
                    if status != gl::FRAMEBUFFER_COMPLETE {
                        panic!("Framebuffer is not complete for reading: {:?}", status);
                    }
                }*/
                Self {
                    gl_handle,
                    _allow_draw: allow_draw,
                    _allow_read: allow_read,
                }
            })
            .collect_array()
    }

    fn _set_attachment<const TEXTURE_TYPE: TextureType>(
        &mut self,
        attachment: AttachmentType,
        texture: Texture<TEXTURE_TYPE>,
        level: GLint,
    ) {
        unsafe {
            gl::NamedFramebufferTexture(
                self.handle(),
                attachment.gl_enum(),
                texture.handle(),
                level,
            )
        };
    }
}

impl GLHandle for Framebuffer {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            let deleted_buffers = [self.gl_handle];
            unsafe { gl::DeleteFramebuffers(1, deleted_buffers.as_ptr()) };
        }
    }
}
