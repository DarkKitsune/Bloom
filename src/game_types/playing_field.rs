use crate::*;
use fennec_algebra::*;

pub struct PlayingField {
    size: Vec3f,
    viewport_ratio: Vec4f,
}

impl PlayingField {
    pub fn new(size: Vec3f, viewport_ratio: Vec4f) -> Self {
        Self {
            size,
            viewport_ratio,
        }
    }

    pub fn size(&self) -> Vec3f {
        self.size
    }

    pub fn viewport(&self, window_size: Vec2u) -> Vec4f {
        vector!(
            (*window_size.x() as f32 * *self.viewport_ratio.x()),
            (*window_size.y() as f32 * *self.viewport_ratio.y()),
            (*window_size.x() as f32 * *self.viewport_ratio.z()),
            (*window_size.y() as f32 * *self.viewport_ratio.w()),
        )
    }

    pub fn viewport_pixels(&self, window_size: Vec2u) -> Vec4i {
        let viewport = self.viewport(window_size);
        vector!(
            *viewport.x() as i32,
            *viewport.y() as i32,
            *viewport.z() as i32,
            *viewport.w() as i32,
        )
    }

    pub fn viewport_aspect_ratio(&self, window_size: Vec2u) -> f32 {
        let viewport = self.viewport(window_size);
        *viewport.z() / *viewport.w()
    }
}
