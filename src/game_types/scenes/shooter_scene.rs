use crate::*;
use fennec_algebra::*;
use glfw::Key;

const STARTING_FIELD_SIZE: Vec2f = vector!(2.0, 2.0);
const STARTING_FIELD_VIEWPORT: Vec4f = vector!(0.05, 0.05, 0.75, 0.9);
const PLAYER_SPAWN_POINT: Vec2f = vector!(0.0, 0.8);

pub struct ShooterScene {
    playing_field: PlayingField,
    player_list: PlayerList,
}

impl ShooterScene {
    pub fn new(current_time: f64) -> Self {
        let playing_field = PlayingField::new(STARTING_FIELD_SIZE, STARTING_FIELD_VIEWPORT);
        let mut player_list = PlayerList::new(1, PLAYER_SPAWN_POINT * playing_field.size() * 0.5);
        player_list.add_player(
            vector!(0.15, 0.15),
            vector!(0.0, 0.0, 64.0, 64.0),
            current_time,
        );
        Self {
            playing_field,
            player_list,
        }
    }
}

impl Scene for ShooterScene {
    fn event_start(&mut self, _game: &mut Game) {}

    fn event_update(&mut self, game: &mut Game, delta_time: f64, current_time: f64) {
        // Update player
        self.player_list.update(game, delta_time, current_time);
    }

    fn event_draw(&mut self, game: &mut Game, delta_time: f64, _current_time: f64) {
        let window_size = game.window().size();

        // Clear buffer
        game.gfx_mut()
            .clear_color(&mut window_framebuffer(), &vector!(0.0, 0.0, 0.0, 1.0));
        game.gfx_mut()
            .clear_depth_stencil(&mut window_framebuffer(), None, 0);
        /*
        // Set viewport for background
        game.gfx_mut().viewport(
            vector!(0, 0, window_size[0] as i32, window_size[1] as i32,),
            true,
        );*/

        // Set viewport for playing field
        game.gfx_mut()
            .viewport(self.playing_field.viewport_pixels(window_size), true);

        // Set camera
        game.gfx_mut().set_view(
            Mat4f::view(
                vector!(0.0, 0.0, 1.0),
                Vector::zero(),
                vector!(0.0, -1.0, 0.0),
            )
            .unwrap(),
        );
        game.gfx_mut().set_projection(Mat4f::ortho(
            vector!(
                self.playing_field.viewport_aspect_ratio(window_size) * 2.0,
                2.0
            ),
            -1.0,
            1.0,
        ));

        // Draw player
        self.player_list.draw(game.gfx_mut(), delta_time);
    }

    fn event_key(&mut self, _game: &mut Game, key: Key, pressed: bool, current_time: f64) {}
}
