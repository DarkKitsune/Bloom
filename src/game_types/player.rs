use crate::*;
use fennec_algebra::*;
use glfw::Key;

pub struct Player {
    sprite_object: SpriteObject,
    scale: Vec2f,
    sprite_rectangle: Vec4f,
}

impl Player {
    pub fn new(
        sprite_list: &mut SpriteList,
        position: Vec2f,
        scale: Vec2f,
        sprite_rectangle: Vec4f,
        current_time: f64,
    ) -> Self {
        let sprite_rectangle = sprite_list.rectangle_to_texcoord(sprite_rectangle);
        Self {
            sprite_object: sprite_list.new_sprite_object(SpriteActorVertex::new(sprite_rectangle, current_time).with_position(position).with_scale(scale)),
            scale,
            sprite_rectangle,
        }
    }

    pub fn update(
        &mut self,
        game: &mut Game,
        sprite_list: &mut SpriteList,
        delta_time: f64,
        current_time: f64,
    ) {
        if game.input().just_pressed(INPUT_LEFT) {
            self.sprite_object.set_velocity(sprite_list, vector!(0.3, 0.0), current_time);
        }
        if game.input().just_pressed(INPUT_RIGHT) {
            self.sprite_object.set_velocity(sprite_list, vector!(-0.3, 0.0), current_time);
        }
    }
}