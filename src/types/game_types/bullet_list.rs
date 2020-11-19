use crate::*;
use fennec_algebra::*;
use std::rc::Rc;

const MAX_BULLETS: GLsizeiptr = 5000;

pub struct BulletList {
    material: Rc<SpriteMaterial>,
    bullets: Vec<Bullet>,
    vertex_array: VertexArray,
}

impl BulletList {
    pub fn new(material: Rc<SpriteMaterial>) -> Self {
        let instance_buffer = VertexBufferBinding::new(Rc::new(Buffer::<SpriteInstanceVertex>::new(MAX_BULLETS, false, true)), 1);
        let (vertex_buffer, index_buffer) = material.create_vertex_index_buffers();
        let vertex_array = VertexArray::new(vec![Box::new(instance_buffer) as Box<dyn DynVertexBufferBinding>, Box::new(vertex_buffer)], index_buffer);
        Self {
            material,
            bullets: Vec::new(),
            vertex_array,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for bullet in self.bullets.iter_mut() {
            bullet.update(delta_time);
        }
    }

    pub fn draw(&self, gfx: &mut GFX, time: f64) {
        let scale_multiplier = self.material.texture().expect("Texture not set in sprite material").size();
        let scale_multiplier = vector!(scale_multiplier[0] as f32, scale_multiplier[1] as f32);
        let instance_buffer = self.vertex_array.vertex_buffer_bindings()[0].buffer();
        let mapped_instance_buffer = instance_buffer.map();
        for (idx, bullet) in self.bullets.iter().enumerate() {
            unsafe {
                let rectangle_scale = vector!(bullet.rectangle()[2], bullet.rectangle()[3]);
                let ptr = mapped_instance_buffer.add(idx) as *mut _;
                *ptr = SpriteInstanceVertex::new(bullet.position(), bullet.scale() * rectangle_scale * scale_multiplier, Quaternion::IDENTITY, bullet.rectangle());
            }
        }
        mapped_instance_buffer.unmap();
        gfx.draw_indices(self.material.as_ref(),  &self.vertex_array, self.bullets.len() as GLint);
    }

    pub fn push(&mut self, bullet: Bullet) {
        self.bullets.push(bullet);
    }
}