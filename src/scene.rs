use super::cell_interactions;
use super::constants;
use super::constants::CollisionType;
use super::entities::*;
use super::input_handler;
use super::physics;
use itertools::Itertools;
use nalgebra::Vector2;
use std::collections::hash_map::HashMap;
extern crate graphics;
extern crate opengl_graphics;

use piston_window::*;

pub struct Scene {
    pub window_size: (f64, f64),
    pub cell_collection: CellCollection,
    pub direction_marker: DirectionMarker,
    player_index: i32,
    indices_to_delete: Vec<i32>, //indices of cells to delete at the end of the update
}

impl Scene {
    pub fn new(
        window_size: (f64, f64),
        cell_collection: CellCollection,
        player_index: i32,
    ) -> Self {
        let direction_marker = DirectionMarker::new(10.0);

        let indices_to_delete = Vec::new();

        Scene {
            window_size,
            cell_collection,
            direction_marker,
            player_index,
            indices_to_delete,
        }
    }

    pub fn get_player(&self) -> &Cell {
        return self.cell_collection.get_cell(self.player_index);
    }

    pub fn get_player_mut(&mut self) -> &mut Cell {
        return self.cell_collection.get_cell_mut(self.player_index);
    }

    pub fn update(
        &mut self,
        dt: f64,
        game_speed: &constants::GameSpeed,
        input_handler: &input_handler::InputHandler,
    ) {
        //Explanation for the order of function calls:
        // Advance the time step and handle ejection before handling cell collisions, since the latter resolves overlaps between cells
        // Handle cell collisions before wall bounce, since the latter corrects the position of cells that end up out of bounds

        self.direction_marker.position =
            self.get_direction_marker_position(input_handler.mouse_position);

        let dt_factor = match game_speed {
            constants::GameSpeed::SLOW => 0.5,
            constants::GameSpeed::NORMAL => 1.0,
            constants::GameSpeed::FAST => 5.0,
        };

        let dt_scaled = dt * dt_factor;

        physics::integrate(self, dt_scaled);

        for input_action in input_handler.input_actions.iter() {
            if let input_handler::InputAction::LeftMouseClick(mouse_coordinates) = input_action {
                self.handle_ejection(*mouse_coordinates);
            }
        }

        self.handle_cell_collisions();

        self.handle_wall_bounce();

        self.update_colors();

        //self.check_win_loss_condition();

        self.handle_object_deletion();
    }

    pub fn render(&self, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
        self.direction_marker.render(transform, graphics);
        for cell in self.cell_collection.get_cells() {
            cell.render(transform, graphics);
        }
    }

    fn get_direction_marker_position(&self, mouse_position: Vector2<f64>) -> Vector2<f64> {
        let player = self.get_player();
        let aim_direction = (mouse_position - player.position).normalize();
        player.position + aim_direction * player.radius
    }

    fn handle_ejection(&mut self, mouse_position: Vector2<f64>) {
        let mut ejection_calculator = cell_interactions::EjectionCalculator::new(self.get_player());
        ejection_calculator.calculate(mouse_position);

        let new_cell = ejection_calculator.ejected_particle.unwrap();
        let new_player_radius = ejection_calculator.new_player_radius;
        let new_player_velocity = ejection_calculator.new_player_velocity;

        self.cell_collection.add_cell(new_cell);
        self.get_player_mut().radius = new_player_radius;
        self.get_player_mut().velocity = new_player_velocity;

        if new_player_radius == 0.0 {
            self.indices_to_delete.push(self.player_index);
        }
    }

    fn handle_wall_bounce(&mut self) {
        let mut new_positions = HashMap::new();
        let mut new_velocities = HashMap::new();
        for index in self.cell_collection.get_indices() {
            let cell = self.cell_collection.get_cell(*index);
            let mut wall_bounce_calculator =
                cell_interactions::WallBounceCalculator::new(&cell, self.window_size);
            wall_bounce_calculator.calculate();

            if let Some(p) = wall_bounce_calculator.new_position {
                new_positions.insert(*index, p);
            }
            if let Some(v) = wall_bounce_calculator.new_velocity {
                new_velocities.insert(*index, v);
            }
        }

        for (index, position) in new_positions.iter() {
            self.cell_collection.get_cell_mut(*index).position = *position;
        }

        for (index, velocity) in new_velocities.iter() {
            self.cell_collection.get_cell_mut(*index).velocity = *velocity;
        }
    }

    fn handle_cell_collisions(&mut self) {
        let mut new_radii = HashMap::new();
        let mut new_velocities = HashMap::new();
        for pair in self.cell_collection.get_indices().combinations(2) {
            let index0 = *pair[0];
            let index1 = *pair[1];
            let cell0 = self.cell_collection.get_cell(index0);
            let cell1 = self.cell_collection.get_cell(index1);

            let mut collision_calculator =
                cell_interactions::CollisionCalculator::new([cell0, cell1], [index0, index1]);
            collision_calculator.calculate();
            if collision_calculator.collision_type == CollisionType::NoCollision {
                continue;
            }

            for i in 0..2 {
                let index_i = *pair[i];
                if collision_calculator.should_delete_cell[i] {
                    self.indices_to_delete.push(index_i);
                } else {
                    new_radii.insert(index_i, collision_calculator.new_radii[i]);
                    new_velocities.insert(index_i, collision_calculator.new_velocities[i]);
                }
            }
        }

        for (index, radius) in new_radii.iter() {
            self.cell_collection.get_cell_mut(*index).radius = *radius;
        }

        for (index, velocity) in new_velocities.iter() {
            self.cell_collection.get_cell_mut(*index).velocity = *velocity;
        }
    }

    fn update_colors(&mut self) {
        let new_colors =
            cell_interactions::calculate_cell_colors(&self.cell_collection, self.player_index);
        for (index, color) in new_colors.iter() {
            self.cell_collection.get_cell_mut(*index).color = *color;
        }
    }

    fn handle_object_deletion(&mut self) {
        if self.indices_to_delete.contains(&self.player_index) {
            panic!("PLAYER DELETED!");
        }

        for index in self.indices_to_delete.iter() {
            self.cell_collection.delete_cell(index);
        }
        self.indices_to_delete.clear();
    }
}
