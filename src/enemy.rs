use super::constants;
use nalgebra::Vector2;

pub struct Enemy {
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub radius: f64,
    pub color: constants::Color,
}

impl Enemy {
    pub fn new(
        position: Vector2<f64>,
        velocity: Vector2<f64>,
        radius: f64,
        color: constants::Color,
    ) -> Self {
        Enemy {
            position,
            velocity,
            radius,
            color,
        }
    }
}
