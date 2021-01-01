use crate::*;
use fennec_algebra::*;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

#[derive(Debug)]
pub struct SpriteReference {
    idx: usize,
}

#[derive(Debug)]
pub struct SpriteList {
    max_sprites: GLsizeiptr,
    sprite_instances: Vec<Option<SpriteInstanceVertex>>,
    sprite_material: Rc<RefCell<SpriteMaterial>>,
    sprite_model: Option<Model>,
    range_changed: Option<Range<GLsizeiptr>>,
    texture_size: Vec2f,
}

impl SpriteList {
    pub fn new(
        mut sprite_material: SpriteMaterial,
        texture: Rc<Texture<{ TextureType::Texture2D }>>,
        max_sprites: GLsizeiptr,
    ) -> Self {
        if DEBUG && max_sprites <= 0 {
            panic!("Max sprites must be greater than 0");
        }
        let texture_size = vector!(*texture.size().x() as f32, *texture.size().y() as f32);
        let (vertex_buffer_binding, index_buffer) = sprite_material.get_vertex_index_buffers();
        sprite_material.set_texture(texture);
        let instance_buffer_binding = VertexBufferBinding::new::<SpriteInstanceVertex>(
            Rc::new(RefCell::new(Buffer::new::<SpriteInstanceVertex>(
                max_sprites,
                false,
                true,
            ))),
            1,
        );
        let vertex_buffer_bindings = vec![instance_buffer_binding, vertex_buffer_binding];
        let vertex_array = VertexArray::new(vertex_buffer_bindings, index_buffer);
        let sprite_material = Rc::new(RefCell::new(sprite_material));
        let sprite_meshes = vec![Mesh::new(
            sprite_material.clone(),
            vertex_array,
            PrimitiveType::TriangleList,
        )];
        let sprite_model = Model::new(sprite_meshes);

        Self {
            max_sprites,
            sprite_instances: Vec::with_capacity(max_sprites as usize),
            sprite_material,
            sprite_model: Some(sprite_model),
            range_changed: None,
            texture_size,
        }
    }

    fn next_available_idx(&self) -> Option<usize> {
        for idx in 0..self.sprite_instances.len() {
            if self.sprite_instances[idx].is_none() {
                return Some(idx);
            }
        }
        if self.sprite_instances.len() < self.max_sprites as usize {
            Some(self.sprite_instances.len())
        } else {
            None
        }
    }

    fn mark_sprite_changed(&mut self, idx: usize) {
        if let Some(range_changed) = self.range_changed.as_mut() {
            *range_changed =
                range_changed.start.min(idx as isize)..range_changed.end.max(idx as isize + 1);
        } else {
            self.range_changed = Some(idx as isize..idx as isize + 1);
        }
    }

    pub fn add_sprite(&mut self, sprite: SpriteInstanceVertex) -> SpriteReference {
        if let Some(idx) = self.next_available_idx() {
            if idx < self.sprite_instances.len() {
                self.sprite_instances[idx] = Some(sprite);
            } else {
                self.sprite_instances.push(Some(sprite));
            }
            self.mark_sprite_changed(idx);
            SpriteReference { idx }
        } else {
            panic!("No available sprite indices left");
        }
    }

    pub fn remove_sprite(&mut self, sprite: SpriteReference) {
        self.sprite_instances[sprite.idx] = None;
        self.mark_sprite_changed(sprite.idx);
    }

    pub fn change_sprite(
        &mut self,
        sprite: &SpriteReference,
        f: impl Fn(&mut SpriteInstanceVertex),
    ) {
        if let Some(sprite_instance) = self.sprite_instances[sprite.idx].as_mut() {
            f(sprite_instance);
            self.mark_sprite_changed(sprite.idx);
        } else {
            panic!(
                "SpriteReference {:?} does not point to a valid sprite",
                sprite
            );
        }
    }

    pub fn sprite_instance(&self, idx: usize) -> Option<&SpriteInstanceVertex> {
        if DEBUG && idx > self.sprite_instances.len() {
            panic!("Idx {:?} is outside the valid range of sprites", idx);
        }
        self.sprite_instances[idx].as_ref()
    }

    pub fn draw(&mut self, gfx: &mut GFX) {
        if let Some(range_changed) = &self.range_changed {
            let range_changed =
                range_changed.start..range_changed.end.min(self.sprite_instances.len() as isize);
            let mut model = None;
            std::mem::swap(&mut model, &mut self.sprite_model);
            {
                let instance_buffer = model.as_mut().unwrap().meshes_mut()[0]
                    .vertex_array_mut()
                    .vertex_buffer_bindings_mut()[0]
                    .buffer()
                    .borrow_mut();
                {
                    let mut mapped = instance_buffer.map(range_changed.clone());
                    for idx in 0..range_changed.end - range_changed.start {
                        if let Some(sprite_instance) =
                            self.sprite_instance((range_changed.start + idx) as usize)
                        {
                            mapped[idx as usize] = *sprite_instance;
                        }
                    }
                }
            }
            std::mem::swap(&mut model, &mut self.sprite_model);
            self.range_changed = None;
        }
        gfx.draw_model(
            self.sprite_model.as_ref().unwrap(),
            self.sprite_instances.len() as GLsizei,
        );
    }

    pub fn rectangle_to_texcoord(&self, sprite_rectangle: Vec4f) -> Vec4f {
        let texture_size = self.texture_size;
        sprite_rectangle
            / vector!(
                *texture_size.x(),
                *texture_size.y(),
                *texture_size.x(),
                *texture_size.y()
            )
    }
}
