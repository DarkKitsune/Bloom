use crate::*;
use fennec_algebra::*;

pub struct PlayingField {
    size: Vec2f,
    viewport_ratio: Vec4f,
}

impl PlayingField {
    pub fn new(size: Vec2f, viewport_ratio: Vec4f) -> Self {
        Self {
            size,
            viewport_ratio,
        }
    }

    pub fn size(&self) -> Vec2f {
        self.size
    }

    pub fn viewport(&self, window_size: Vec2u) -> Vec4f {
        vector!(
            (window_size[0] as f32 * self.viewport_ratio[0]),
            (window_size[1] as f32 * self.viewport_ratio[1]),
            (window_size[0] as f32 * self.viewport_ratio[2]),
            (window_size[1] as f32 * self.viewport_ratio[3]),
        )
    }

    pub fn viewport_pixels(&self, window_size: Vec2u) -> Vec4i {
        let viewport = self.viewport(window_size);
        vector!(
            viewport[0] as i32,
            viewport[1] as i32,
            viewport[2] as i32,
            viewport[3] as i32,
        )
    }

    pub fn viewport_aspect_ratio(&self, window_size: Vec2u) -> f32 {
        let viewport = self.viewport(window_size);
        viewport[2] / viewport[3]
    }
}
