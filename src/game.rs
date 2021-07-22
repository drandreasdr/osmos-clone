use super::ring::*;
use piston_window::*;

type Colour = [f32; 4];

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const YELLOW: Colour = [1.0, 1.0, 0.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const TRANSPARENT: Colour = [0.0, 0.0, 0.0, 0.0];

pub struct Game {
    player: Ring,
    enemy: Ring,
}

impl Game {
    pub fn new() -> Self {
        let player = Ring::new(0.0, 0.0, 50.0, RED);
        let enemy = Ring::new(50.0, 50.0, 20.0, YELLOW);
        Game {
            player,
            enemy
        }
    }

    pub fn draw(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _| {
            clear(BLACK, graphics);
            self.player.draw(context.transform,
                             graphics);
            self.enemy.draw(context.transform,
                            graphics);
        });
    }

    pub fn handle_button_args(&mut self, button_args: ButtonArgs, window: &PistonWindow) {
        if button_args.state == ButtonState::Press {
            match button_args.button {
                Button::Keyboard(Key::Up) => self.player.y -= 10.0,
                Button::Keyboard(Key::Down) => self.player.y += 10.0,
                Button::Keyboard(Key::Left) => self.player.x -= 10.0,
                Button::Keyboard(Key::Right) => self.player.x += 10.0,
                _ => (),
            }
        }
    }
}