use super::constants;
use super::renderer;
use super::scene::*;
use piston_window::*;

pub struct Game {
    scene: Scene,
}

impl Game {
    pub fn new() -> Self {
        let scene = Scene::new();
        Game { scene }
    }

    pub fn draw(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _| {
            clear(constants::BLACK, graphics);
            renderer::Renderer::render(&self.scene, context.transform, graphics);
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
