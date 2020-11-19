use fennec_algebra::*;
use glfw::Glfw;
use std::rc::Rc;
use std::time::{Instant, Duration};

use crate::*;

pub const FRAMERATE: f64 = 60.0;

pub struct Game {
    glfw: Glfw,
    window: Window,
    gfx: GFX,
    bullet_list: BulletList,
    start_instant: Instant,
    previous_frame_instant: Instant,
}

impl Game {
    pub fn start() {
        // Create GLFW object
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // Create window
        let mut window = Window::new(&mut glfw, vector!(2560, 1440), "Hello, world!");

        // Create GFX object
        let gfx = GFX::new(&mut window);

        // Create core assets
        let test_texture = Texture::from_file("game/zor.png", image::ImageFormat::Png);

        let mut test_material = SpriteMaterial::new();
        test_material.set_texture(Rc::new(test_texture));

        let mut bullet_list = BulletList::new(Rc::new(test_material));
        bullet_list.push(Bullet::new(vector!(0.0, 0.0), vector!(1.0, 1.0), vector!(5.0, 0.0), vector!(0.0, 0.0, 0.2, 0.2)));
        bullet_list.push(Bullet::new(vector!(100.0, 0.0), vector!(1.0, 1.0), vector!(0.0, 5.0), vector!(3.0, 0.0, 0.2, 0.2)));
        
        // Get the current time
        let start_instant = Instant::now();

        // Create game object
        let mut game = Game {
            glfw,
            window,
            gfx,
            bullet_list,
            start_instant,
            previous_frame_instant: start_instant.clone(),
        };

        // Start the update loop
        game.update_loop();
    }

    fn update_loop(&mut self) {
        loop {
            // Poll GLFW events
            Window::poll_events(&mut self.glfw);
            // Process events for the window
            self.window.process_events();

            // Exit the loop if the window is closed, otherwise continue
            if self.window.is_closed() {
                break;
            }

            std::thread::sleep(Duration::from_secs_f64(1.0 / FRAMERATE));

            // Get the current time and compute delta time (the time passed since the previous frame)
            let now = Instant::now();
            let delta_time = now.duration_since(self.previous_frame_instant).as_secs_f32().max(0.00001);
            self.previous_frame_instant = now;

            // Clear buffer
            self.gfx
                .clear_color(&mut window_framebuffer(), &vector!(1.0, 0.0, 0.5, 1.0));

            // Set the camera
            self.gfx.set_view(
                Mat4f::view(
                    vector!(0.0, 0.0, -1.0),
                    vector!(0.0, 0.0, 0.0),
                    vector!(0.0, -1.0, 0.0),
                )
                .unwrap(),
            );
            self.gfx.set_projection(
                Mat4f::ortho(vector!(self.window.size()[0] as f32, self.window.size()[1] as f32), 0.00001, 1.0),
            );

            // Update and draw the bullet list
            self.bullet_list.update(delta_time);
            self.bullet_list.draw(&mut self.gfx, now.duration_since(self.start_instant).as_secs_f64());

            // Swap window buffers
            self.window.swap_buffers();
        }
    }
}
