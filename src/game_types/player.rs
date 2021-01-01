use crate::*;
use fennec_algebra::*;

pub struct Player {
    sprite_reference: SpriteReference,
    phys_base: PhysicsObjectBase,
    scale: Vec2f,
    sprite_rectangle: Vec4f,
}

impl Player {
    pub fn new(
        sprite_list: &mut SpriteList,
        position: Vec3f,
        scale: Vec2f,
        sprite_rectangle: Vec4f,
    ) -> Self {
        let sprite_rectangle = sprite_list.rectangle_to_texcoord(sprite_rectangle);
        Self {
            sprite_reference: sprite_list.add_sprite(create_sprite_instance_from_parts(
                position,
                scale,
                sprite_rectangle,
            )),
            phys_base: PhysicsObjectBase::new(position, vector!(0.0, 0.0, 0.0)),
            scale,
            sprite_rectangle,
        }
    }

    pub fn update(
        &mut self, /*, game: &mut Game*/
        sprite_list: &mut SpriteList,
        delta_time: f64,
    ) {
        self.phys_base.update(delta_time);
        sprite_list.change_sprite(&self.sprite_reference, |sprite| {
            *sprite = self.create_sprite_instance()
        });
    }

    fn create_sprite_instance(&self) -> SpriteInstanceVertex {
        create_sprite_instance_from_parts(
            self.phys_base.position(),
            self.scale,
            self.sprite_rectangle,
        )
    }
}

fn create_sprite_instance_from_parts(
    position: Vec3f,
    scale: Vec2f,
    sprite_rectangle: Vec4f,
) -> SpriteInstanceVertex {
    SpriteInstanceVertex::new(
        vector!(*position.x(), *position.y()),
        scale,
        Quaternion::IDENTITY,
        sprite_rectangle,
    )
}
