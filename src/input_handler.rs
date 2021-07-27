use super::scene;
use nalgebra::Vector2;
use piston_window::*;

#[derive(Debug)]
pub enum InputAction {
    LeftMouseClick(Vector2<f64>),
    RightMouseClick(Vector2<f64>),
}

pub struct InputHandler {
    pub mouse_position: Vector2<f64>,
    pub input_actions: Vec<InputAction>,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            mouse_position: Vector2::<f64>::new(0.0, 0.0),
            input_actions: Vec::new(),
        }
    }

    pub fn handle_mouse_move(&mut self, scene: &mut scene::Scene, mouse_position: [f64; 2]) {
        self.mouse_position[0] = mouse_position[0];
        self.mouse_position[1] = mouse_position[1];
    }

    pub fn handle_button_press_event(&self, scene: &mut scene::Scene, button: Button) {}

    pub fn handle_button_args(
        &mut self,
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
                Button::Mouse(MouseButton::Left) => self
                    .input_actions
                    .push(InputAction::LeftMouseClick(self.mouse_position)),
                Button::Mouse(MouseButton::Right) => self
                    .input_actions
                    .push(InputAction::RightMouseClick(self.mouse_position)),
                _ => (),
            }
        }
    }

    pub fn clear_input_actions(&mut self) {
        self.input_actions.clear();
    }
}
