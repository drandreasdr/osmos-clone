use super::constants;
use super::enemy::*;
use super::input_handler;
use super::physics;
use super::player::*;
use nalgebra::Vector2;

pub struct Scene {
    pub player: Player,
    pub direction_marker: Enemy, //TODO fix - shouldn't be an Enemy
    pub enemies: Vec<Enemy>,
}

impl Scene {
    pub fn new() -> Self {
        let player = Player::new(
            Vector2::<f64>::new(300.0, 300.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
            constants::RED,
        );

        let direction_marker = Enemy::new(
            Vector2::<f64>::new(0.0, 0.0),
            Vector2::<f64>::new(0.0, 0.0),
            5.0,
            constants::WHITE,
        );

        let enemies = vec![
            Enemy::new(
                Vector2::<f64>::new(50.0, 50.0),
                Vector2::<f64>::new(0.0, 0.0),
                20.0,
                constants::YELLOW,
            ),
            Enemy::new(
                Vector2::<f64>::new(50.0, 150.0),
                Vector2::<f64>::new(1.0, 0.0),
                20.0,
                constants::BLUE,
            ),
        ];

        Scene {
            player,
            direction_marker,
            enemies,
        }
    }

    pub fn update(&mut self, dt: f64, input_handler: &input_handler::InputHandler) {
        physics::integrate(self, dt);

        let direction_marker_relative_position =
            (input_handler.mouse_position - self.player.position).normalize();
        self.direction_marker.position =
            self.player.position + direction_marker_relative_position * self.player.radius;

        for input_action in input_handler.input_actions.iter() {
            if let input_handler::InputAction::LeftMouseClick(mouse_coordinates) = input_action {
                self.enemies.push(Enemy::new(
                    mouse_coordinates.clone(),
                    Vector2::<f64>::new(50.0, 0.0),
                    10.0,
                    constants::WHITE,
                ))
            }
        }
    }
}
