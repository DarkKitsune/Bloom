use fennec_algebra::*;
use glfw::Glfw;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::*;

pub const SMOOTHED_FRAMERATE_SAMPLES: usize = 5;
pub const FOV: f32 = 90.0 * std::f32::consts::PI / 180.0;

pub struct Game {
    glfw: Glfw,
    window: Window,
    gfx: GFX,
    input: Input,
    start_instant: Instant,
    previous_frame_instant: Instant,
    smoothed_framerate: f64,
    task_schedule: Option<TaskSchedule>,
    current_scene: Option<Box<dyn Scene>>,
}

impl Game {
    pub fn start() {
        // Create GLFW object
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // Create window
        let mut window = Window::new(&mut glfw, vector!(2560, 1440), "Hello, world!");

        // Create GFX object
        let gfx = GFX::new(&mut window);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        // Create input handler
        let mut input = Input::new(11);
        input.use_default_key_bindings();

        // Create task schedule
        let mut task_schedule = TaskSchedule::new();

        // Get the current time
        let start_instant = Instant::now();

        // Create game object
        let mut game = Game {
            glfw,
            window,
            gfx,
            input,
            start_instant,
            previous_frame_instant: start_instant.clone(),
            smoothed_framerate: 0.0,
            task_schedule: Some(task_schedule),
            current_scene: Some(Box::new(ShooterScene::new())),
        };

        // Start the update loop
        game.update_loop();
    }

    fn update_loop(&mut self) {
        loop {
            // Copy all input states to previous states
            self.input.copy_state_to_previous();

            // Poll GLFW events
            Window::poll_events(&mut self.glfw);
            // Process events for the window
            let events = self.window.process_events();
            // Handle key events
            for event in events {
                match event {
                    glfw::WindowEvent::Key(key, _, action, _) => match action {
                        glfw::Action::Press => {
                            self.input.set_key_state(key, true);
                            let mut scene = None;
                            std::mem::swap(&mut self.current_scene, &mut scene);
                            scene.as_mut().unwrap().event_key(self, key, true);
                            std::mem::swap(&mut self.current_scene, &mut scene);
                        }
                        glfw::Action::Release => {
                            self.input.set_key_state(key, false);
                            let mut scene = None;
                            std::mem::swap(&mut self.current_scene, &mut scene);
                            scene.as_mut().unwrap().event_key(self, key, false);
                            std::mem::swap(&mut self.current_scene, &mut scene);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }

            // Exit the loop if the window is closed, otherwise continue
            if self.window.is_closed() {
                break;
            }

            // Get the current time and compute delta time (the time passed since the previous frame)
            let now = Instant::now();
            let current_time = now.duration_since(self.start_instant).as_secs_f64();
            let delta_time = now
                .duration_since(self.previous_frame_instant)
                .as_secs_f64()
                .max(0.00001);
            self.previous_frame_instant = now;

            // Calculate the smoothed framerate
            self.smoothed_framerate = (self.smoothed_framerate
                * (SMOOTHED_FRAMERATE_SAMPLES - 1) as f64
                + 1.0 / delta_time)
                / SMOOTHED_FRAMERATE_SAMPLES as f64;
            self.window
                .set_title(format!("FPS: {:.1}", self.smoothed_framerate));

            // Do tasks
            // Swap out task schedule so that we can borrow this game object while executing tasks
            let mut temp_ts = None;
            std::mem::swap(&mut temp_ts, &mut self.task_schedule);
            // Execute tasks
            temp_ts
                .as_mut()
                .unwrap()
                .execute(self, delta_time, current_time);
            // Swap task schedule back into the game object
            std::mem::swap(&mut temp_ts, &mut self.task_schedule);

            let mut scene = None;
            std::mem::swap(&mut self.current_scene, &mut scene);

            // Do update
            scene
                .as_mut()
                .unwrap()
                .event_update(self, delta_time, current_time);

            // Do draw
            scene
                .as_mut()
                .unwrap()
                .event_draw(self, delta_time, current_time);

            std::mem::swap(&mut self.current_scene, &mut scene);

            // Swap window buffers
            self.window.swap_buffers();
        }
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn gfx(&self) -> &GFX {
        &self.gfx
    }

    pub fn gfx_mut(&mut self) -> &mut GFX {
        &mut self.gfx
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
