use crate::*;
use std::rc::Rc;
use glfw::Key;

pub struct PlayerList {
    max_players: usize,
    players: Vec<Player>,
    sprite_list: SpriteList,
    start_position: Vec2f,
}

impl PlayerList {
    pub fn new(max_players: usize, start_position: Vec2f) -> Self {
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
                SpriteAnimator::new(),
                Rc::new(texture),
                max_players as GLsizeiptr,
            ),
            start_position,
        }
    }

    pub fn add_player(&mut self, scale: Vec2f, sprite_rectangle: Vec4f, current_time: f64) {
        if DEBUG && self.players.len() >= self.max_players {
            panic!("Maximum number of players already reached");
        }

        let player = Player::new(
            &mut self.sprite_list,
            self.start_position,
            scale,
            sprite_rectangle,
            current_time,
        );
        self.players.push(player);
    }

    pub fn update(&mut self, game: &mut Game, delta_time: f64, current_time: f64) {
        for player in self.players.iter_mut() {
            player.update(game, &mut self.sprite_list, delta_time, current_time);
        }
    }

    pub fn draw(&mut self, gfx: &mut GFX, delta_time: f64) {
        self.sprite_list.draw(gfx, delta_time);
    }
}
