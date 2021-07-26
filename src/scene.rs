use super::constants;
use super::enemy::*;
use super::input_handler;
use super::physics;
use super::player::*;
use nalgebra::Vector2;

pub struct Scene {
    pub player: Player,
    pub enemy: Enemy,
}

impl Scene {
    pub fn new() -> Self {
        let player = Player::new(
            Vector2::<f64>::new(0.0, 0.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
            constants::RED,
        );

        let enemy = Enemy::new(
            Vector2::<f64>::new(50.0, 50.0),
            Vector2::<f64>::new(0.0, 0.0),
            20.0,
            constants::YELLOW,
        );

        Scene { player, enemy }
    }

    pub fn update(&mut self, dt: f64, input_handler: &input_handler::InputHandler) {
        physics::integrate(self, dt);
    }
}
