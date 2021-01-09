use crate::*;

pub struct SpriteObject {
    sprite_reference: SpriteReference,
}

impl SpriteObject {
    pub fn new(sprite_reference: SpriteReference,) -> Self {
        Self {
            sprite_reference,
        }
    }
    
    pub fn sprite(&self) -> &SpriteReference {
        &self.sprite_reference
    }
    
    pub fn using_sprite_actor<R>(&self, sprite_list: &SpriteList, f: impl Fn(&SpriteActorVertex) -> R) -> R {
        f(sprite_list.sprite_actor(&self.sprite_reference).expect("Sprite actor was lost somehow"))
    }

    pub fn position(&self, sprite_list: &SpriteList, current_time: f64) -> Vec2f {
        self.using_sprite_actor(sprite_list, |actor| actor.position(current_time))
    }

    pub fn velocity(&self, sprite_list: &SpriteList) -> Vec2f {
        self.using_sprite_actor(sprite_list, |actor| actor.velocity())
    }

    pub fn scale(&self, sprite_list: &SpriteList, current_time: f64) -> Vec2f {
        self.using_sprite_actor(sprite_list, |actor| actor.scale(current_time))
    }

    pub fn scalar_velocity(&self, sprite_list: &SpriteList) -> Vec2f {
        self.using_sprite_actor(sprite_list, |actor| actor.scalar_velocity())
    }

    pub fn rotation(&self, sprite_list: &SpriteList) -> f32 {
        self.using_sprite_actor(sprite_list, |actor| actor.rotation())
    }

    pub fn rectangle(&self, sprite_list: &SpriteList) -> Vec4f {
        self.using_sprite_actor(sprite_list, |actor| actor.rectangle())
    }

    pub fn set_position(&mut self, sprite_list: &mut SpriteList, position: Vec2f, current_time: f64) {
        sprite_list.change_sprite(&self.sprite_reference, |actor| actor.set_position(position, current_time))
    }

    pub fn set_velocity(&mut self, sprite_list: &mut SpriteList, velocity: Vec2f, current_time: f64) {
        sprite_list.change_sprite(&self.sprite_reference, |actor| actor.set_velocity(velocity, current_time))
    }

    pub fn set_scale(&mut self, sprite_list: &mut SpriteList, scale: Vec2f, current_time: f64) {
        sprite_list.change_sprite(&self.sprite_reference, |actor| actor.set_scale(scale, current_time))
    }

    pub fn set_scalar_velocity(&mut self, sprite_list: &mut SpriteList, scalar_velocity: Vec2f, current_time: f64) {
        sprite_list.change_sprite(&self.sprite_reference, |actor| actor.set_scalar_velocity(scalar_velocity, current_time))
    }

    pub fn set_rotation(&mut self, sprite_list: &mut SpriteList, rotation: f32, current_time: f64) {
        sprite_list.change_sprite(&self.sprite_reference, |actor| actor.set_rotation(rotation, current_time))
    }

    pub fn set_rectangle(&mut self, sprite_list: &mut SpriteList, rectangle: Vec4f, current_time: f64) {
        sprite_list.change_sprite(&self.sprite_reference, |actor| actor.set_rectangle(rectangle, current_time))
    }
}