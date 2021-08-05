use super::entities::*;
use nalgebra::Vector2;
use std::f64::consts::PI;

pub const RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE: f64 = 50.0;
pub const RADIUS_EJECTED_PARTICLE: f64 = 10.0;

pub fn calculate_ejection(
    player: &Player,
    target_coordinates: Vector2<f64>,
) -> (Vector2<f64>, Vector2<f64>) {
    let aim_direction = (target_coordinates - player.position).normalize();
    let velocity_ejected_particle =
        RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE * aim_direction + player.velocity;

    let area_player = PI * player.radius * player.radius;
    let area_ejected_particle = PI * RADIUS_EJECTED_PARTICLE * RADIUS_EJECTED_PARTICLE;

    let new_velocity_player = area_player / (area_player - area_ejected_particle) * player.velocity
        - area_ejected_particle / (area_player - area_ejected_particle) * velocity_ejected_particle;

    println!("---");
    println!("{:?}", area_player);
    println!("{:?}", area_ejected_particle);
    println!("{:?}", player.velocity);
    println!("{:?}", aim_direction);
    println!("{:?}", velocity_ejected_particle);
    println!("{:?}", new_velocity_player);

    //Calculate new body velocity vector
    (velocity_ejected_particle, new_velocity_player)
}
