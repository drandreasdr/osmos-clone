use super::constants;
use super::entities::Cell;
use nalgebra::Vector2;
use std::f64::consts::PI;

pub fn get_velocity_factors_for_wall_bounce(
    position: Vector2<f64>,
    radius: f64,
    window_size: (f64, f64),
) -> Vector2<f64> {
    let mut result = Vector2::<f64>::new(1.0, 1.0);
    if position[0] - radius < 0.0 || position[0] + radius > window_size.0 {
        result[0] *= -1.0;
    }
    if position[1] - radius < 0.0 || position[1] + radius > window_size.1 {
        result[1] *= -1.0;
    }

    result
}

pub fn create_ejected_particle(
    mouse_position: Vector2<f64>,
    player_position: Vector2<f64>,
    player_velocity: Vector2<f64>,
    player_radius: f64,
) -> Cell {
    let aim_direction = (mouse_position - player_position).normalize();

    let ejected_particle_position =
        player_position + aim_direction * (player_radius + constants::RADIUS_EJECTED_PARTICLE);

    let ejected_particle_velocity =
        constants::RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE * aim_direction + player_velocity;

    Cell::new(
        ejected_particle_position,
        ejected_particle_velocity,
        constants::RADIUS_EJECTED_PARTICLE,
        constants::WHITE,
    )
}

pub fn get_new_player_velocity_after_ejection(
    player_current_velocity: Vector2<f64>,
    player_radius: f64,
    ejected_particle_velocity: Vector2<f64>,
) -> Vector2<f64> {
    let area_player = PI * player_radius * player_radius;
    let area_ejected_particle =
        PI * constants::RADIUS_EJECTED_PARTICLE * constants::RADIUS_EJECTED_PARTICLE;

    let new_velocity_player = area_player / (area_player - area_ejected_particle)
        * player_current_velocity
        - area_ejected_particle / (area_player - area_ejected_particle) * ejected_particle_velocity;

    new_velocity_player
}
