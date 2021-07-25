use super::constants;
use super::enemy::*;
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
            Vector2::<f64>::new(100.0, 10.0),
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
}
