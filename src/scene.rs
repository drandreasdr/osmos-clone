use super::collisions;
use super::constants;
use super::entities::*;
use super::input_handler;
use super::physics;
use nalgebra::Vector2;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;

pub struct Scene {
    pub window_size: (f64, f64),
    pub player: Player,
    pub direction_marker: DirectionMarker,
    pub enemies: Vec<Enemy>,
}

impl Scene {
    pub fn new(window_size: (f64, f64)) -> Self {
        let player = Player::new(
            Vector2::<f64>::new(300.0, 300.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
            constants::RED,
        );

        let direction_marker = DirectionMarker::new(10.0);

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
            window_size,
            player,
            direction_marker,
            enemies,
        }
    }

    pub fn update(&mut self, dt: f64, input_handler: &input_handler::InputHandler) {
        let aim_direction = (input_handler.mouse_position - self.player.position).normalize();
        self.direction_marker.position = self.player.position + aim_direction * self.player.radius;

        physics::integrate(self, dt);

        collisions::handle_collisions(self);

        for input_action in input_handler.input_actions.iter() {
            if let input_handler::InputAction::LeftMouseClick(mouse_coordinates) = input_action {
                collisions::handle_ejection(self, *mouse_coordinates);
            }
        }
    }

    pub fn render(&self, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
        self.player.render(transform, graphics);
        self.direction_marker.render(transform, graphics);
        for enemy in self.enemies.iter() {
            enemy.render(transform, graphics);
        }
    }
}
