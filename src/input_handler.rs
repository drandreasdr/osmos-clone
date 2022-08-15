use super::constants::GameAction;
use super::constants::GameSpeed;
use piston_window::*;
use std::collections::hash_map::HashMap;

#[derive(Debug)]
pub enum InputAction {
    LeftMouseClick([f64; 2]),
    RightMouseClick([f64; 2]),
    ButtonPress(Key),
    Other,
}

pub struct InputHandler {
    key_map: HashMap<Key, GameAction>,
    pub mouse_position: [f64; 2],
    pub game_actions: Vec<GameAction>,
}

impl InputHandler {
    pub fn new() -> Self {
        let key_map = HashMap::from([
            (Key::Q, GameAction::SetGameSpeed(GameSpeed::SLOW)),
            (Key::W, GameAction::SetGameSpeed(GameSpeed::NORMAL)),
            (Key::E, GameAction::SetGameSpeed(GameSpeed::FAST)),
        ]);

        InputHandler {
            mouse_position: [0.0; 2],
            game_actions: Vec::new(),
            key_map,
        }
    }

    pub fn handle_mouse_move(&mut self, mouse_position: [f64; 2]) {
        self.mouse_position = mouse_position;
    }

    pub fn handle_button_press_event(&self, _button: Button) {}

    pub fn handle_button_args(&mut self, button_args: ButtonArgs) {
        let input_action = if button_args.state == ButtonState::Press {
            match button_args.button {
                Button::Keyboard(keyboard_key) => InputAction::ButtonPress(keyboard_key),
                Button::Mouse(MouseButton::Left) => {
                    InputAction::LeftMouseClick(self.mouse_position)
                }
                Button::Mouse(MouseButton::Right) => {
                    InputAction::RightMouseClick(self.mouse_position)
                }
                _ => InputAction::Other,
            }
        } else {
            InputAction::Other
        };

        let maybe_game_action: Option<GameAction> = match input_action {
            InputAction::LeftMouseClick(mouse_position) => {
                Some(GameAction::EjectCell(mouse_position))
            }
            InputAction::RightMouseClick(_) => None,
            InputAction::ButtonPress(keyboard_key) => self.key_map.get(&keyboard_key).copied(),
            InputAction::Other => None,
        };
        if let Some(game_action) = maybe_game_action {
            self.game_actions.push(game_action)
        };
    }

    pub fn clear_game_actions(&mut self) {
        self.game_actions.clear();
    }
}
