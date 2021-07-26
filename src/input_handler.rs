use super::scene;
use piston_window::*;

pub fn handle_button_args(
    scene: &mut scene::Scene,
    button_args: ButtonArgs,
    window: &PistonWindow,
) {
    if button_args.state == ButtonState::Press {
        match button_args.button {
            Button::Keyboard(Key::Up) => scene.player.velocity[1] -= 10.0,
            Button::Keyboard(Key::Down) => scene.player.velocity[1] += 10.0,
            Button::Keyboard(Key::Left) => scene.player.velocity[0] -= 10.0,
            Button::Keyboard(Key::Right) => scene.player.velocity[0] += 10.0,
            _ => (),
        }
    }
}
