extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use super::constants;
use super::input_handler;
use super::physics;
use super::renderer;
use super::scene;
use piston_window::*;
use std::time::Instant;

const WINDOW_SIZE: u32 = 512;

pub struct Game {
    scene: scene::Scene,
}

impl Game {
    pub fn new() -> Self {
        let scene = scene::Scene::new();
        Game { scene }
    }

    pub fn run(&mut self) {
        let mut window: PistonWindow =
            piston_window::WindowSettings::new("Osmos clone", (WINDOW_SIZE, WINDOW_SIZE))
                .exit_on_esc(true)
                .build()
                .unwrap();

        let frames_per_second = 60;
        let updates_per_second = 60;
        window.set_max_fps(frames_per_second);
        window.set_ups(updates_per_second);

        while let Some(event) = window.next() {
            if let Some(update_args) = event.update_args() {
                physics::integrate(&mut self.scene, update_args.dt);
            }

            if let Some(_) = event.render_args() {
                window.draw_2d(&event, |context, graphics, _| {
                    clear(constants::BLACK, graphics);
                    renderer::render(&self.scene, context.transform, graphics);
                });
            }

            if let Some(button_args) = event.button_args() {
                input_handler::handle_button_args(&mut self.scene, button_args, &window);
            }
        }
    }
}
