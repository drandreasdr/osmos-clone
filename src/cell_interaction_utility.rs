use super::constants;
use super::constants::CollisionType;
use super::constants::Direction;
use super::entities::Cell;
use nalgebra::Vector2;
use std::f64::consts::PI;

pub struct WallBounceCalculator<'a> {
    cell: &'a Cell,
    window_size: (f64, f64),
    pub new_position: Option<Vector2<f64>>,
    pub new_velocity: Option<Vector2<f64>>,
}

impl<'a> WallBounceCalculator<'a> {
    pub fn new(cell: &'a Cell, window_size: (f64, f64)) -> Self {
        WallBounceCalculator {
            cell,
            window_size,
            new_position: None,
            new_velocity: None,
        }
    }

    pub fn calculate(&mut self) {
        self.new_position = self.get_position_after_wall_bounce();
        self.new_velocity = self.get_velocity_after_wall_bounce();
    }

    fn get_directions_where_cell_overlaps_edge(&self) -> Vec<Direction> {
        let mut result = Vec::new();

        let position_x = self.cell.position[0];
        let position_y = self.cell.position[1];
        let window_size_x = self.window_size.0;
        let window_size_y = self.window_size.1;
        let radius = self.cell.radius;

        if position_x - radius < 0.0 {
            result.push(Direction::Left);
        }
        if position_x + radius > window_size_x {
            result.push(Direction::Right);
        }
        if position_y - radius < 0.0 {
            result.push(Direction::Up);
        }
        if position_y + radius > window_size_y {
            result.push(Direction::Down);
        }

        result
    }

    fn get_position_after_wall_bounce(&self) -> Option<Vector2<f64>> {
        let directions_where_cell_overlaps_edge = self.get_directions_where_cell_overlaps_edge();
        let radius = self.cell.radius;

        let mut new_position_x = None;
        let mut new_position_y = None;
        if directions_where_cell_overlaps_edge.contains(&Direction::Left) {
            new_position_x = Some(radius);
        }
        if directions_where_cell_overlaps_edge.contains(&Direction::Right) {
            new_position_x = Some(self.window_size.0 - radius);
        }
        if directions_where_cell_overlaps_edge.contains(&Direction::Up) {
            new_position_y = Some(radius);
        }
        if directions_where_cell_overlaps_edge.contains(&Direction::Down) {
            new_position_y = Some(self.window_size.1 - radius);
        }

        if new_position_x.is_none() && new_position_y.is_none() {
            return None;
        }

        let mut result = self.cell.position;
        if let Some(p) = new_position_x {
            result[0] = p;
        }
        if let Some(p) = new_position_y {
            result[1] = p;
        }
        Some(result)
    }

    fn get_velocity_after_wall_bounce(&self) -> Option<Vector2<f64>> {
        let directions_where_cell_overlaps_edge = self.get_directions_where_cell_overlaps_edge();

        let mut velocity_factor_x = None;
        let mut velocity_factor_y = None;
        if directions_where_cell_overlaps_edge.contains(&Direction::Left) {
            velocity_factor_x = Some(1.0);
        }
        if directions_where_cell_overlaps_edge.contains(&Direction::Right) {
            velocity_factor_x = Some(-1.0);
        }
        if directions_where_cell_overlaps_edge.contains(&Direction::Up) {
            velocity_factor_y = Some(1.0);
        }
        if directions_where_cell_overlaps_edge.contains(&Direction::Down) {
            velocity_factor_y = Some(-1.0);
        }

        if velocity_factor_x.is_none() && velocity_factor_y.is_none() {
            return None;
        }

        let mut result = self.cell.velocity;
        if let Some(f) = velocity_factor_x {
            result[0] = result[0].abs() * f;
        }
        if let Some(f) = velocity_factor_y {
            result[1] = result[1].abs() * f;
        }
        Some(result)
    }
}

pub struct EjectionCalculator<'a> {
    player: &'a Cell,
    pub ejected_particle: Option<Cell>,
    pub new_player_radius: f64,
    pub new_player_velocity: Vector2<f64>,
}

impl<'a> EjectionCalculator<'a> {
    pub fn new(player: &'a Cell) -> Self {
        EjectionCalculator {
            player,
            ejected_particle: None,
            new_player_radius: 0.0,
            new_player_velocity: Vector2::<f64>::new(0.0, 0.0),
        }
    }

    pub fn calculate(&mut self, mouse_position: Vector2<f64>) {
        self.ejected_particle = Some(self.create_ejected_particle(mouse_position));

        self.new_player_radius = self.get_new_player_radius_after_ejection();

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

    fn get_new_player_radius_after_ejection(&mut self) -> f64 {
        (self.player.radius.powf(2.0) - constants::RADIUS_EJECTED_PARTICLE.powf(2.0)).sqrt()
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

/// Calculates parameters revelant to the collision between two cells. Meant to be a short-lived utility struct.
/// The rule is that the larger cell dominates (consumes) the smaller. If they are equal in size, then the older cell consumes the newer.
/// Internally in this struct, the cells are reordered such that the dominant one is the first one,
///  but the order of the output is guaranteed to be with respect to the original cell order,
///  so the user of the struct can consume the output without caring about this detail
pub struct CollisionCalculator<'a> {
    cells: [&'a Cell; 2],                    //The first cell is the dominant one
    is_cell_order_reversed_internally: bool, //Whether the cell order was reversed in order to make sure the first cell is the dominant one
    pub collision_type: CollisionType,
    pub new_radii: [f64; 2],
    pub new_velocities: [Vector2<f64>; 2],
    pub should_delete_cell: [bool; 2],
}

impl<'a> CollisionCalculator<'a> {
    pub fn new(cells_in: [&'a Cell; 2], cell_keys: [i32; 2]) -> Self {
        let mut cells = cells_in;
        let mut is_cell_order_reversed_internally = false;
        if cells[0].radius < cells[1].radius {
            is_cell_order_reversed_internally = true;
        } else if cells[0].radius == cells[1].radius && cell_keys[0] > cell_keys[1] {
            is_cell_order_reversed_internally = true;
        }
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
        let velocity1 = self.cells[1].velocity;
        let distance = (position0 - position1).norm();
        let area0 = PI * radius0.powf(2.0);
        let area1 = PI * radius1.powf(2.0);

        match self.collision_type {
            CollisionType::FullMerge => [
                area0 / (area0 + area1) * velocity0 + area1 / (area0 + area1) * velocity1,
                velocity1,
            ],
            CollisionType::PartialMerge => {
                let area_transferred = PI / 2.0
                    * (-radius0.powf(2.0)
                        + radius1.powf(2.0)
                        + distance
                            * (2.0 * radius0.powf(2.0) + 2.0 * radius1.powf(2.0)
                                - distance.powf(2.0))
                            .sqrt());
                [
                    area0 / (area0 + area_transferred) * velocity0
                        + area_transferred / (area0 + area_transferred) * velocity1,
                    velocity1,
                ]
            }
            CollisionType::NoCollision => [velocity0, velocity1],
        }
    }
    pub fn get_should_delete_cell_after_collision(&self) -> [bool; 2] {
        let mut result = [false, false];

        if self.collision_type == CollisionType::FullMerge {
            result[1] = true;
        }

        result
    }
}
