use crate::*;
use std::rc::Rc;

pub struct PlayerList {
    max_players: usize,
    players: Vec<Player>,
    sprite_list: SpriteList,
    start_position: Vec3f,
}

impl PlayerList {
    pub fn new(max_players: usize, start_position: Vec3f) -> Self {
        if DEBUG && max_players == 0 {
            panic!("Max players must be greater than 0");
        }

        let texture = Texture::from_file(
            path!("Game", "Textures", "pl00.png"),
            image::ImageFormat::Png,
        );

        Self {
            max_players,
            players: Vec::new(),
            sprite_list: SpriteList::new(
                SpriteMaterial::new(),
                Rc::new(texture),
                max_players as GLsizeiptr,
            ),
            start_position,
        }
    }

    pub fn add_player(&mut self, scale: Vec2f, sprite_rectangle: Vec4f) {
        if DEBUG && self.players.len() >= self.max_players {
            panic!("Maximum number of players already reached");
        }

        let player = Player::new(
            &mut self.sprite_list,
            self.start_position,
            scale,
            sprite_rectangle,
        );
        self.players.push(player);
    }

    pub fn update(&mut self, _game: &mut Game, delta_time: f64) {
        for player in self.players.iter_mut() {
            player.update(&mut self.sprite_list, delta_time);
        }
    }

    pub fn draw(&mut self, gfx: &mut GFX) {
        self.sprite_list.draw(gfx);
    }
}
