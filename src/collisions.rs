use super::constants;
use super::entities::*;
use super::scene;
use nalgebra::Vector2;
use std::f64::consts::PI;

const RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE: f64 = 50.0;
const RADIUS_EJECTED_PARTICLE: f64 = 10.0;

pub fn handle_ejection(scene: &mut scene::Scene, target_coordinates: Vector2<f64>) {
    let aim_direction = (target_coordinates - scene.player.position).normalize();

    let new_enemy_start_position =
        scene.player.position + aim_direction * (scene.player.radius + RADIUS_EJECTED_PARTICLE);

    let velocity_ejected_particle =
        RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE * aim_direction + scene.player.velocity;

    let area_player = PI * scene.player.radius * scene.player.radius;
    let area_ejected_particle = PI * RADIUS_EJECTED_PARTICLE * RADIUS_EJECTED_PARTICLE;

    let new_velocity_player = area_player / (area_player - area_ejected_particle)
        * scene.player.velocity
        - area_ejected_particle / (area_player - area_ejected_particle) * velocity_ejected_particle;

    scene.enemies.push(Enemy::new(
        new_enemy_start_position,
        velocity_ejected_particle,
        RADIUS_EJECTED_PARTICLE,
        constants::WHITE,
    ));
    scene.player.velocity = new_velocity_player;
}

pub fn handle_collisions(scene: &mut scene::Scene) {
    let get_velocity_factors =
        |position: Vector2<f64>, radius: f64, window_size: (f64, f64)| -> Vector2<f64> {
            let mut result = Vector2::<f64>::new(1.0, 1.0);
            if position[0] - radius < 0.0 || position[0] + radius > window_size.0 {
                result[0] *= -1.0;
            }
            if position[1] - radius < 0.0 || position[1] + radius > window_size.1 {
                result[1] *= -1.0;
            }

            result
        };

    let velocity_factors = get_velocity_factors(
        scene.player.position,
        scene.player.radius,
        scene.window_size,
    );
    scene.player.velocity[0] *= velocity_factors[0];
    scene.player.velocity[1] *= velocity_factors[1];

    for enemy in scene.enemies.iter_mut() {
        let velocity_factors =
            get_velocity_factors(enemy.position, enemy.radius, scene.window_size);
        enemy.velocity[0] *= velocity_factors[0];
        enemy.velocity[1] *= velocity_factors[1];
    }
}
