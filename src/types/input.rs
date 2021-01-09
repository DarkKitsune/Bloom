use crate::*;
use fennec_algebra::*;
use glfw::Key;
use std::ops::Sub;

pub const INPUT_LEFT: usize = 0;
pub const INPUT_RIGHT: usize = 1;
pub const INPUT_UP: usize = 2;
pub const INPUT_DOWN: usize = 3;
pub const INPUT_SHOOT1: usize = 4;
pub const INPUT_SHOOT2: usize = 5;
pub const INPUT_BOMB: usize = 6;
pub const INPUT_SLOW: usize = 7;
pub const INPUT_SPEED_SCALE_UP: usize = 8;
pub const INPUT_SPEED_SCALE_DOWN: usize = 9;
pub const INPUT_LAND: usize = 10;

pub struct Input {
    states: Vec<bool>,
    previous_states: Vec<bool>,
    key_bindings: Vec<Option<glfw::Key>>,
}

impl Input {
    pub fn new(state_count: usize) -> Self {
        Self {
            states: (0..state_count).map(|_| false).collect(),
            previous_states: (0..state_count).map(|_| false).collect(),
            key_bindings: (0..state_count).map(|_| None).collect(),
        }
    }

    pub fn use_default_key_bindings(&mut self) {
        self.bind_key(INPUT_LEFT, Some(Key::Left));
        self.bind_key(INPUT_RIGHT, Some(Key::Right));
        self.bind_key(INPUT_UP, Some(Key::Up));
        self.bind_key(INPUT_DOWN, Some(Key::Down));
        self.bind_key(INPUT_SHOOT1, Some(Key::Z));
        self.bind_key(INPUT_SHOOT2, Some(Key::X));
        self.bind_key(INPUT_BOMB, Some(Key::C));
        self.bind_key(INPUT_SLOW, Some(Key::LeftShift));
        self.bind_key(INPUT_SPEED_SCALE_UP, Some(Key::Num1));
        self.bind_key(INPUT_SPEED_SCALE_DOWN, Some(Key::Num2));
        self.bind_key(INPUT_LAND, Some(Key::Num3));
    }

    pub fn set_state(&mut self, idx: usize, state: bool) {
        if DEBUG && idx > self.states.len() {
            panic!(
                "Cannot set input state {} as there are only {} defined states",
                idx,
                self.states.len()
            );
        }
        self.states[idx] = state;
    }

    pub fn set_key_state(&mut self, key: glfw::Key, state: bool) {
        for idx in self
            .key_bindings
            .iter()
            .enumerate()
            .filter_map(|(idx, &binding)| {
                if binding == Some(key) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
        {
            self.set_state(idx, state);
        }
    }

    pub fn state(&self, idx: usize) -> bool {
        if DEBUG && idx > self.states.len() {
            panic!(
                "Cannot get input state {} as there are only {} defined states",
                idx,
                self.states.len()
            );
        }
        self.states[idx]
    }

    pub fn bind_key(&mut self, idx: usize, key: Option<glfw::Key>) {
        if DEBUG && idx > self.key_bindings.len() {
            panic!(
                "Cannot bind input state {} as there are only {} defined states",
                idx,
                self.key_bindings.len()
            );
        }
        self.key_bindings[idx] = key;
    }

    pub fn axis_state<T: One + Zero + Sub<T, Output = T>>(
        &self,
        pos_idx: usize,
        neg_idx: usize,
    ) -> T {
        (if self.state(pos_idx) {
            T::one()
        } else {
            T::zero()
        }) - (if self.state(neg_idx) {
            T::one()
        } else {
            T::zero()
        })
    }

    pub fn copy_state_to_previous(&mut self) {
        self.previous_states = self.states.clone();
    }

    pub fn previous_state(&self, idx: usize) -> bool {
        if DEBUG && idx > self.previous_states.len() {
            panic!(
                "Cannot get input state {} as there are only {} defined states",
                idx,
                self.previous_states.len()
            );
        }
        self.previous_states[idx]
    }

    pub fn state_changed(&self, idx: usize) -> bool {
        if DEBUG && idx > self.states.len() {
            panic!(
                "Cannot get input state {} as there are only {} defined states",
                idx,
                self.states.len()
            );
        }
        self.states[idx] != self.previous_states[idx]
    }

    pub fn just_pressed(&self, idx: usize) -> bool {
        self.state_changed(idx) && self.state(idx)
    }

    pub fn just_released(&self, idx: usize) -> bool {
        self.state_changed(idx) && !self.state(idx)
    }
}
