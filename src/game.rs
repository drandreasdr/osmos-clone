extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use super::constants;
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

    pub fn run(&self) {
        let mut window: PistonWindow =
            piston_window::WindowSettings::new("Osmos clone", (WINDOW_SIZE, WINDOW_SIZE))
                .exit_on_esc(true)
                .build()
                .unwrap();

        let frames_per_second = 60;
        let updates_per_second = 10; //TODO not used
        let mut previous_now = Instant::now();
        let mut now = Instant::now();

        let mut game = Game::new();

        window.set_max_fps(frames_per_second);
        window.set_ups(updates_per_second);

        while let Some(event) = window.next() {
            if let Some(_) = event.render_args() {
                previous_now = now;
                now = Instant::now();
                let time_elapsed = now - previous_now;

                game.update(time_elapsed.as_secs_f64());

                game.draw(&mut window, &event);
            }

            if let Some(button_args) = event.button_args() {
                game.handle_button_args(button_args, &window);
            }
        }
    }

    pub fn update(&mut self, time_step: f64) {
        physics::integrate(&mut self.scene, time_step);
    }

    pub fn draw(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _| {
            clear(constants::BLACK, graphics);
            renderer::render(&self.scene, context.transform, graphics);
        });
    }

    pub fn handle_button_args(&mut self, button_args: ButtonArgs, window: &PistonWindow) {
        if button_args.state == ButtonState::Press {
            match button_args.button {
                Button::Keyboard(Key::Up) => self.scene.player.position[1] -= 10.0,
                Button::Keyboard(Key::Down) => self.scene.player.position[1] += 10.0,
                Button::Keyboard(Key::Left) => self.scene.player.position[0] -= 10.0,
                Button::Keyboard(Key::Right) => self.scene.player.position[0] += 10.0,
                _ => (),
            }
        }
    }
}
