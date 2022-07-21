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

pub struct CollisionCalculator<'a> {
    cell0: &'a Cell,
    cell1: &'a Cell,
    pub collision_type: constants::CollisionType,
    pub new_radii: [f64; 2],
    pub new_velocities: [Vector2<f64>; 2],
    pub should_delete_cell: [bool; 2],
}

impl<'a> CollisionCalculator<'a> {
    pub fn new(cell0: &'a Cell, cell1: &'a Cell) -> Self {
        CollisionCalculator {
            cell0,
            cell1,
            collision_type: constants::CollisionType::NoCollision,
            new_radii: [0.0; 2],
            new_velocities: [Vector2::<f64>::new(0.0, 0.0); 2],
            should_delete_cell: [false; 2],
        }
    }

    pub fn calculate(&mut self) {
        self.collision_type = self.get_collision_type();

        if self.collision_type == constants::CollisionType::NoCollision {
            return;
        }

        self.new_radii = self.get_radii_after_collision();
        self.new_velocities = self.get_velocities_after_collision();
        self.should_delete_cell = self.get_should_delete_cell_after_collision();
    }

    fn get_collision_type(&self) -> constants::CollisionType {
        let radius0 = self.cell0.radius;
        let radius1 = self.cell1.radius;
        let distance = (self.cell0.position - self.cell1.position).norm();
        if radius0 + radius1 <= distance {
            return constants::CollisionType::NoCollision;
        }
        if radius0.powf(2.0) + radius1.powf(2.0) <= distance.powf(2.0) {
            return constants::CollisionType::FullMerge;
        }
        return constants::CollisionType::PartialMerge;
    }

    fn get_radii_after_collision(&self) -> [f64; 2] {
        let radius0 = self.cell0.radius;
        let radius1 = self.cell1.radius;
        let distance = (self.cell0.position - self.cell1.position).norm();

        let root_term =
            (2.0 * radius0.powf(2.0) + 2.0 * radius1.powf(2.0) - distance.powf(2.0)).sqrt();
        let new_radius0 = 0.5 * (distance + root_term);
        let new_radius1 = 0.5 * (distance - root_term);
        [new_radius0, new_radius1]
    }

    pub fn get_velocities_after_collision(&self) -> [Vector2<f64>; 2] {
        [Vector2::<f64>::new(1.0, 1.0); 2]
    }
    pub fn get_should_delete_cell_after_collision(&self) -> [bool; 2] {
        [true; 2]
    }
}
