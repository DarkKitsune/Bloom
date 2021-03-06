use crate::*;
use glfw::Key;

pub trait Scene {
    fn event_start(&mut self, game: &mut Game);
    fn event_update(&mut self, game: &mut Game, delta_time: f64, current_time: f64);
    fn event_draw(&mut self, game: &mut Game, delta_time: f64, current_time: f64);
    fn event_key(&mut self, game: &mut Game, key: Key, pressed: bool, current_time: f64);
}
