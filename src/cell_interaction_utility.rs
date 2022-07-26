use super::constants;
use super::constants::CollisionType;
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

pub struct EjectionCalculator<'a> {
    player: &'a Cell,
    pub ejected_particle: Option<Cell>,
    pub new_player_velocity: Vector2<f64>,
}

impl<'a> EjectionCalculator<'a> {
    pub fn new(player: &'a Cell) -> Self {
        EjectionCalculator {
            player,
            ejected_particle: None,
            new_player_velocity: Vector2::<f64>::new(0.0, 0.0),
        }
    }

    pub fn calculate(&mut self, mouse_position: Vector2<f64>) {
        self.ejected_particle = Some(self.create_ejected_particle(mouse_position));

        self.new_player_velocity = self.get_new_player_velocity_after_ejection();
    }

    fn create_ejected_particle(&mut self, mouse_position: Vector2<f64>) -> Cell {
        let aim_direction = (mouse_position - self.player.position).normalize();

        let ejected_particle_position = self.player.position
            + aim_direction * (self.player.radius * 1.05 + constants::RADIUS_EJECTED_PARTICLE);

        let ejected_particle_velocity = constants::RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE
            * aim_direction
            + self.player.velocity;

        Cell::new(
            ejected_particle_position,
            ejected_particle_velocity,
            constants::RADIUS_EJECTED_PARTICLE,
            constants::WHITE,
        )
    }

    fn get_new_player_velocity_after_ejection(&mut self) -> Vector2<f64> {
        let area_player = PI * self.player.radius.powf(2.0);
        let area_ejected_particle = PI * constants::RADIUS_EJECTED_PARTICLE.powf(2.0);
        let new_velocity_player = area_player / (area_player - area_ejected_particle)
            * self.player.velocity
            - area_ejected_particle / (area_player - area_ejected_particle)
                * self.ejected_particle.as_ref().unwrap().velocity;
        new_velocity_player
    }
}

/// Calculates parameters revelant to the collision between two cells. Meant to be a short-lived utility struct
/// For the purposes of the internal calculations of this struct, the cells are reordered such that the largest is the first one
///  in any case, the order of the results are guaranteed to be with respect to the original order, so the user of the struct doesn't need to worry about this detail
pub struct CollisionCalculator<'a> {
    cells: [&'a Cell; 2],                    //The larger/smaller cell, in that order
    is_cell_order_reversed_internally: bool, //Whether the cell order was reversed in order to make sure the first cell is the larger one
    pub collision_type: CollisionType,
    pub new_radii: [f64; 2],
    pub new_velocities: [Vector2<f64>; 2],
    pub should_delete_cell: [bool; 2],
}

impl<'a> CollisionCalculator<'a> {
    pub fn new(cells_in: [&'a Cell; 2]) -> Self {
        let mut cells = cells_in;
        let is_cell_order_reversed_internally = cells[0].radius < cells[1].radius;
        if is_cell_order_reversed_internally {
            cells.reverse();
        }

        CollisionCalculator {
            cells,
            is_cell_order_reversed_internally,
            collision_type: CollisionType::NoCollision,
            new_radii: [0.0; 2],
            new_velocities: [Vector2::<f64>::new(0.0, 0.0); 2],
            should_delete_cell: [false; 2],
        }
    }

    pub fn calculate(&mut self) {
        self.collision_type = self.get_collision_type();

        if self.collision_type == CollisionType::NoCollision {
            return;
        }

        self.new_radii = self.get_radii_after_collision();
        self.new_velocities = self.get_velocities_after_collision();
        self.should_delete_cell = self.get_should_delete_cell_after_collision();

        if self.is_cell_order_reversed_internally {
            //We need to reverse the order of the results in order to respect the original cell order
            self.new_radii.reverse();
            self.new_velocities.reverse();
            self.should_delete_cell.reverse();
        }
    }

    fn get_collision_type(&self) -> CollisionType {
        let radius0 = self.cells[0].radius;
        let radius1 = self.cells[1].radius;
        let distance = (self.cells[0].position - self.cells[1].position).norm();
        if distance >= radius0 + radius1 {
            return CollisionType::NoCollision;
        }
        if radius0 == radius1 {
            return CollisionType::Bounce;
        }
        if distance.powf(2.0) <= radius0.powf(2.0) + radius1.powf(2.0) {
            return CollisionType::FullMerge;
        }
        return CollisionType::PartialMerge;
    }

    fn get_radii_after_collision(&self) -> [f64; 2] {
        let radius0 = self.cells[0].radius;
        let radius1 = self.cells[1].radius;
        let position0 = self.cells[0].position;
        let position1 = self.cells[1].position;
        let distance = (position0 - position1).norm();

        let mut new_radius0 = radius0;
        let mut new_radius1 = radius1;

        match self.collision_type {
            CollisionType::FullMerge => {
                new_radius0 = (radius0.powf(2.0) + radius1.powf(2.0)).sqrt();
            }
            CollisionType::PartialMerge => {
                let root_term =
                    (2.0 * radius0.powf(2.0) + 2.0 * radius1.powf(2.0) - distance.powf(2.0)).sqrt();
                new_radius0 = 0.5 * (distance + root_term);
                new_radius1 = 0.5 * (distance - root_term);
            }
            CollisionType::Bounce => {}
            CollisionType::NoCollision => {}
        };

        [new_radius0, new_radius1]
    }

    pub fn get_velocities_after_collision(&self) -> [Vector2<f64>; 2] {
        let radius0 = self.cells[0].radius;
        let radius1 = self.cells[1].radius;
        let position0 = self.cells[0].position;
        let position1 = self.cells[1].position;
        let velocity0 = self.cells[0].velocity;
        let velocity1 = self.cells[0].velocity;
        let distance = (position0 - position1).norm();
        let area0 = PI * radius0.powf(2.0);
        let area1 = PI * radius1.powf(2.0);

        let mut new_velocity0 = Vector2::<f64>::new(0.0, 0.0);
        let mut new_velocity1 = Vector2::<f64>::new(0.0, 0.0);
        match self.collision_type {
            CollisionType::FullMerge => {
                new_velocity0 =
                    area0 / (area0 + area1) * velocity0 + area1 / (area0 + area1) * velocity1;
                new_velocity1 = velocity1;
            }
            CollisionType::PartialMerge => {
                let area_transferred = PI / 2.0
                    * (-radius0.powf(2.0)
                        + radius1.powf(2.0)
                        + distance
                            * (2.0 * radius0.powf(2.0) + 2.0 * radius1.powf(2.0)
                                - distance.powf(2.0))
                            .sqrt());
                new_velocity0 = area0 / (area0 + area_transferred) * velocity0
                    + area_transferred / (area0 + area_transferred) * velocity1;
                new_velocity1 = velocity1;
            }
            CollisionType::Bounce => {
                let r_0_to_1 = position1 - position0;
                let e_perp = r_0_to_1 / r_0_to_1.norm(); //perpendicular
                let e_para = Vector2::<f64>::new(e_perp[1], -e_perp[0]); //parallel
                let new_velocity0_perp = velocity1.dot(&e_perp);
                let new_velocity1_perp = velocity0.dot(&e_perp);
                let new_velocity0_para = velocity0.dot(&e_para);
                let new_velocity1_para = velocity1.dot(&e_para);

                new_velocity0 = new_velocity0_perp * e_perp + new_velocity0_para * e_para;
                new_velocity1 = new_velocity1_perp * e_perp + new_velocity1_para * e_para;
            }
            CollisionType::NoCollision => {}
        };

        [new_velocity0, new_velocity1]
    }
    pub fn get_should_delete_cell_after_collision(&self) -> [bool; 2] {
        let mut result = [false, false];

        if self.collision_type == CollisionType::FullMerge {
            result[1] = true;
        }

        result
    }
}
