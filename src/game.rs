extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use super::constants;
use super::input_handler;
use super::renderer;
use super::scene;
use piston_window::*;

const WINDOW_SIZE: u32 = 512;

pub struct Game {
    scene: scene::Scene,
    input_handler: input_handler::InputHandler,
}

impl Game {
    pub fn new() -> Self {
        let scene = scene::Scene::new();
        let input_handler = input_handler::InputHandler::new();
        Game {
            scene,
            input_handler,
        }
    }

    pub fn run(&mut self) {
        let mut window: PistonWindow =
            piston_window::WindowSettings::new("Osmos clone", (WINDOW_SIZE, WINDOW_SIZE))
                .exit_on_esc(true)
                .build()
                .unwrap();

        let frames_per_second = 1;
        let updates_per_second = 1;
        window.set_max_fps(frames_per_second);
        window.set_ups(updates_per_second);

        while let Some(event) = window.next() {
            if let Some(mouse_coordinates) = event.mouse_cursor_args() {
                self.input_handler
                    .handle_mouse_move(&mut self.scene, mouse_coordinates);
            }

            if let Some(button) = event.press_args() {
                self.input_handler
                    .handle_button_press_event(&mut self.scene, button);
            }

            if let Some(button_args) = event.button_args() {
                self.input_handler
                    .handle_button_args(&mut self.scene, button_args, &window);
            }

            if let Some(update_args) = event.update_args() {
                self.scene.update(update_args.dt, &self.input_handler);
                self.input_handler.clear_input_actions();
            }

            if let Some(_) = event.render_args() {
                window.draw_2d(&event, |context, graphics, _| {
                    clear(constants::BLACK, graphics);
                    renderer::render(&self.scene, context.transform, graphics);
                });
            }
        }
    }
}
