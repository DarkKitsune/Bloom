use fennec_algebra::*;
use glfw::Glfw;

use crate::*;

pub struct Game {
    glfw: Glfw,
    window: Window,
    gfx: GFX,
    hello_triangle_material: HelloTriangleMaterial,
    hello_triangle_vertex_array: VertexArray,
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
        // Material
        let hello_triangle_material = HelloTriangleMaterial::new();

        // Instance buffer
        let hello_triangle_instances = Buffer::from_slice(
            &[
                InstanceModelVertex::new(vector!(-0.8, -0.8, 0.0), vector!(0.5, 0.5, 1.0)),
                InstanceModelVertex::new(vector!(0.8, 0.8, 0.0), vector!(0.9, 0.9, 1.0)),
            ],
            false,
        );

        // Vertex buffer
        let hello_triangle_vertices = Buffer::from_slice(
            &[
                PosColorVertex::new(vector!(-1.0, -1.0, 0.0), vector!(1.0, 0.0, 0.0)),
                PosColorVertex::new(vector!(1.0, -1.0, 0.0), vector!(0.0, 1.0, 0.0)),
                PosColorVertex::new(vector!(0.0, 1.0, 0.0), vector!(0.0, 0.0, 1.0)),
            ],
            false,
        );

        // Instance & vertex buffers combined
        let hello_triangle_vertex_bindings = vec![
            VertexBufferBinding::new(Box::new(hello_triangle_instances), 1),
            VertexBufferBinding::new(Box::new(hello_triangle_vertices), 0),
        ];

        // Index buffer
        let hello_triangle_indices = Buffer::from_slice(&[0, 1, 2], false);

        // Final vertex array
        let hello_triangle_vertex_array =
            VertexArray::new(hello_triangle_vertex_bindings, hello_triangle_indices);

        // Create game object
        let mut game = Game {
            glfw,
            window,
            gfx,
            hello_triangle_material,
            hello_triangle_vertex_array,
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

            // Clear buffer
            self.gfx
                .clear_color(&mut window_framebuffer(), &vector!(1.0, 0.0, 0.5, 1.0));

            // Set the camera
            self.gfx.set_view(
                Mat4f::view(
                    vector!(1.0, 2.0, 5.0),
                    vector!(0.0, 0.0, 0.0),
                    vector!(0.0, 0.0, 1.0),
                )
                .unwrap(),
            );
            self.gfx.set_projection(
                Mat4f::projection(
                    std::f32::consts::PI * 0.5,
                    self.window.aspect_ratio(),
                    0.01,
                    100.0,
                )
                .unwrap(),
            );

            // Draw test triangles
            self.gfx.draw_indices(
                &self.hello_triangle_material,
                &self.hello_triangle_vertex_array,
                2,
            );

            // Swap window buffers
            self.window.swap_buffers();
        }
    }
}
