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
    actors: Vec<Option<SpriteActorVertex>>,
    sprite_material: Rc<RefCell<SpriteMaterial>>,
    sprite_animator: SpriteAnimator,
    sprite_model: Option<Model>,
    range_changed: Option<Range<GLsizeiptr>>,
    texture_size: Vec2f,
}

impl SpriteList {
    pub fn new(
        mut sprite_material: SpriteMaterial,
        mut sprite_animator: SpriteAnimator,
        texture: Rc<Texture<{ TextureType::Texture2D }>>,
        max_sprites: GLsizeiptr,
    ) -> Self {
        if DEBUG && max_sprites <= 0 {
            panic!("Max sprites must be greater than 0");
        }
        let texture_size = vector!(texture.size()[0] as f32, texture.size()[1] as f32);
        let (vertex_buffer_binding, index_buffer) = sprite_material.get_vertex_index_buffers();
        sprite_material.set_texture(texture);
        let actor_buffer = Rc::new(RefCell::new(Buffer::new::<SpriteActorVertex>(
            max_sprites,
            false,
            true,
        )));
        let instance_buffer = Rc::new(RefCell::new(Buffer::new::<SpriteInstanceVertex>(
            max_sprites,
            false,
            true,
        )));
        sprite_animator.set_buffers(actor_buffer, instance_buffer.clone());
        let instance_buffer_binding =
            VertexBufferBinding::new::<SpriteInstanceVertex>(instance_buffer, 1);
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
            actors: Vec::with_capacity(max_sprites as usize),
            sprite_material,
            sprite_animator,
            sprite_model: Some(sprite_model),
            range_changed: None,
            texture_size,
        }
    }

    fn next_available_idx(&self) -> Option<usize> {
        for idx in 0..self.actors.len() {
            if self.actors[idx].is_none() {
                return Some(idx);
            }
        }
        if self.actors.len() < self.max_sprites as usize {
            Some(self.actors.len())
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

    pub fn new_sprite_object(&mut self, sprite: SpriteActorVertex) -> SpriteObject {
        SpriteObject::new(self.add_sprite(sprite))
    }

    pub fn add_sprite(&mut self, sprite: SpriteActorVertex) -> SpriteReference {
        if let Some(idx) = self.next_available_idx() {
            if idx < self.actors.len() {
                self.actors[idx] = Some(sprite);
            } else {
                self.actors.push(Some(sprite));
            }
            self.mark_sprite_changed(idx);
            SpriteReference { idx }
        } else {
            panic!("No available sprite indices left");
        }
    }

    pub fn remove_sprite(&mut self, sprite: SpriteReference) {
        self.actors[sprite.idx] = None;
        self.mark_sprite_changed(sprite.idx);
    }

    pub fn change_sprite(&mut self, sprite: &SpriteReference, f: impl Fn(&mut SpriteActorVertex)) {
        if let Some(sprite_instance) = self.actors[sprite.idx].as_mut() {
            f(sprite_instance);
            self.mark_sprite_changed(sprite.idx);
        } else {
            panic!(
                "SpriteReference {:?} does not point to a valid sprite",
                sprite
            );
        }
    }

    pub fn sprite_actor(&self, sprite: &SpriteReference) -> Option<&SpriteActorVertex> {
        if DEBUG && sprite.idx > self.actors.len() {
            panic!("Idx {:?} is outside the valid range of sprites", sprite.idx);
        }
        self.actors[sprite.idx].as_ref()
    }

    pub fn sprite_actor_mut(&mut self, sprite: &SpriteReference) -> Option<&mut SpriteActorVertex> {
        if DEBUG && sprite.idx > self.actors.len() {
            panic!("Idx {:?} is outside the valid range of sprites", sprite.idx);
        }
        self.actors[sprite.idx].as_mut()
    }

    pub fn draw(&mut self, gfx: &mut GFX, delta_time: f64) {
        if let Some(range_changed) = &self.range_changed {
            let range_changed =
                range_changed.start..range_changed.end.min(self.actors.len() as isize);
            {
                let actor_buffer = self
                    .sprite_animator
                    .actor_buffer()
                    .expect("Sprite animator does not have its buffers set")
                    .borrow();
                {
                    let mut mapped = actor_buffer.map(range_changed.clone());
                    for idx in 0..range_changed.end - range_changed.start {
                        if let Some(sprite_instance) =
                            self.sprite_actor(&SpriteReference {idx: (range_changed.start + idx) as usize})
                        {
                            mapped[idx as usize] = *sprite_instance;
                        }
                    }
                }
            }
            self.range_changed = None;
        }
        self.sprite_animator.animate(gfx, delta_time);
        gfx.draw_model(
            self.sprite_model.as_ref().unwrap(),
            self.actors.len() as GLsizei,
        );
    }

    pub fn rectangle_to_texcoord(&self, sprite_rectangle: Vec4f) -> Vec4f {
        let texture_size = self.texture_size;
        sprite_rectangle
            / vector!(
                texture_size[0],
                texture_size[1],
                texture_size[0],
                texture_size[1]
            )
    }
}
