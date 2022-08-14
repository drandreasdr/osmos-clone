extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use super::constants;
use super::constants::WINDOW_SIZE;
use super::entities;
use super::input_handler;
use super::scene;
use piston_window::*;

pub struct Game {
    scene: scene::Scene,
    input_handler: input_handler::InputHandler,
    game_speed: constants::GameSpeed,
}

impl Game {
    pub fn new() -> Self {
        let window_size = (WINDOW_SIZE as f64, WINDOW_SIZE as f64);
        let mut cell_collection_factory = entities::CellCollectionFactory::new(window_size);
        cell_collection_factory.generate();

        let scene = scene::Scene::new(
            window_size,
            cell_collection_factory.cell_collection.unwrap(),
            cell_collection_factory.player_index.unwrap(),
        );
        let input_handler = input_handler::InputHandler::new();
        Game {
            scene,
            input_handler,
            game_speed: constants::GameSpeed::SLOW,
        }
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
            if let Some(mouse_coordinates) = event.mouse_cursor_args() {
                self.input_handler.handle_mouse_move(mouse_coordinates);
            }

            if let Some(button) = event.press_args() {
                self.input_handler.handle_button_press_event(button);
            }

            if let Some(button_args) = event.button_args() {
                self.input_handler
                    .handle_button_args(&mut self.scene.get_player_mut(), button_args);
            }

            if let Some(update_args) = event.update_args() {
                self.scene
                    .update(update_args.dt, &self.game_speed, &self.input_handler);
                self.input_handler.clear_input_actions();
            }

            if let Some(_) = event.render_args() {
                window.draw_2d(&event, |context, graphics, _| {
                    clear(constants::BLACK, graphics);
                    self.scene.render(context.transform, graphics);
                });
            }
        }
    }
}
