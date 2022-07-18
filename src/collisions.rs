use super::constants;
use super::entities::*;
use super::scene;
use itertools::Itertools;
use nalgebra::Vector2;
use std::f64::consts::PI;

const RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE: f64 = 50.0;
const RADIUS_EJECTED_PARTICLE: f64 = 10.0;

pub fn handle_ejection(scene: &mut scene::Scene, target_coordinates: Vector2<f64>) {
    let enemies = scene.cell_collection.get_enemies();

    let aim_direction =
        (target_coordinates - scene.cell_collection.get_player().position).normalize();

    let new_enemy_start_position = scene.cell_collection.get_player().position
        + aim_direction * (scene.cell_collection.get_player().radius + RADIUS_EJECTED_PARTICLE);

    let velocity_ejected_particle = RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE * aim_direction
        + scene.cell_collection.get_player().velocity;

    let area_player =
        PI * scene.cell_collection.get_player().radius * scene.cell_collection.get_player().radius;
    let area_ejected_particle = PI * RADIUS_EJECTED_PARTICLE * RADIUS_EJECTED_PARTICLE;

    let new_velocity_player = area_player / (area_player - area_ejected_particle)
        * scene.cell_collection.get_player().velocity
        - area_ejected_particle / (area_player - area_ejected_particle) * velocity_ejected_particle;

    let apa = Cell::new(
        CellType::NonPlayer,
        new_enemy_start_position,
        velocity_ejected_particle,
        RADIUS_EJECTED_PARTICLE,
        constants::WHITE,
    );
    scene.cell_collection.add_cell(apa);
    scene.cell_collection.get_player_mut().velocity = new_velocity_player;
}

pub fn handle_collisions(scene: &mut scene::Scene) {
    let player = scene.cell_collection.get_player_mut();

    //Handle wall bounce:
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

    let velocity_factors = get_velocity_factors(player.position, player.radius, scene.window_size);
    player.velocity[0] *= velocity_factors[0];
    player.velocity[1] *= velocity_factors[1];

    for enemy in scene.cell_collection.get_enemies_mut().iter_mut() {
        let velocity_factors =
            get_velocity_factors(enemy.position, enemy.radius, scene.window_size);
        enemy.velocity[0] *= velocity_factors[0];
        enemy.velocity[1] *= velocity_factors[1];
    }

    //Handle merge:
    println!("---");
    for pair in scene.cell_collection.cells.iter().combinations(2) {
        let cell1 = pair[0];
        let cell2 = pair[1];
        println!("{:?}, {:?}", cell1.position, cell2.position);
        if cell1.overlapsWith(&cell2) {
            println!("{:?} overlaps with {:?}", cell1, cell2);
        }
    }
    //Check all against all cells for overlaps
    //For each overlap:
    // find the new radii such that the cells are exactly tangent to each other, and the corresponding transferred area
    // find the new velocities
}
