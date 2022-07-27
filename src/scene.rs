use super::cell_interaction_utility;
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
    player_key: i32,
    keys_to_delete: Vec<i32>, //keys to delete at the end of the update
}

impl Scene {
    pub fn new(window_size: (f64, f64)) -> Self {
        let mut cell_collection = CellCollection::new();

        let player_key = cell_collection.add_cell(Cell::new(
            Vector2::<f64>::new(300.0, 300.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
            constants::RED,
        ));

        cell_collection.add_cell(Cell::new(
            Vector2::<f64>::new(50.0, 50.0),
            Vector2::<f64>::new(0.0, 0.0),
            20.0,
            constants::YELLOW,
        ));
        cell_collection.add_cell(Cell::new(
            Vector2::<f64>::new(240.0, 240.0),
            Vector2::<f64>::new(1.0, 0.0),
            20.0,
            constants::BLUE,
        ));

        let direction_marker = DirectionMarker::new(10.0);

        let keys_to_delete = Vec::new();

        Scene {
            window_size,
            cell_collection,
            direction_marker,
            player_key,
            keys_to_delete,
        }
    }

    pub fn get_player(&self) -> &Cell {
        return self.cell_collection.get_cell(self.player_key);
    }

    pub fn get_player_mut(&mut self) -> &mut Cell {
        return self.cell_collection.get_cell_mut(self.player_key);
    }

    pub fn update(&mut self, dt: f64, input_handler: &input_handler::InputHandler) {
        self.direction_marker.position =
            self.get_direction_marker_position(input_handler.mouse_position);

        for input_action in input_handler.input_actions.iter() {
            if let input_handler::InputAction::LeftMouseClick(mouse_coordinates) = input_action {
                self.handle_ejection(*mouse_coordinates);
            }
        }

        self.handle_wall_bounce();

        self.handle_cell_collisions();

        self.handle_object_deletion();

        physics::integrate(self, dt);
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
        let mut ejection_calculator =
            cell_interaction_utility::EjectionCalculator::new(self.get_player());
        ejection_calculator.calculate(mouse_position);

        let new_cell = ejection_calculator.ejected_particle.unwrap();
        let new_player_radius = ejection_calculator.new_player_radius;
        let new_player_velocity = ejection_calculator.new_player_velocity;

        self.cell_collection.add_cell(new_cell);
        self.get_player_mut().radius = new_player_radius;
        self.get_player_mut().velocity = new_player_velocity;
    }

    fn handle_wall_bounce(&mut self) {
        let mut new_positions = HashMap::new();
        let mut new_velocities = HashMap::new();
        for key in self.cell_collection.get_keys() {
            let cell = self.cell_collection.get_cell(*key);
            let mut wall_bounce_calculator =
                cell_interaction_utility::WallBounceCalculator::new(&cell, self.window_size);
            wall_bounce_calculator.calculate();

            if let Some(p) = wall_bounce_calculator.new_position {
                new_positions.insert(*key, p);
            }
            if let Some(v) = wall_bounce_calculator.new_velocity {
                new_velocities.insert(*key, v);
            }
        }

        for (key, position) in new_positions.iter() {
            self.cell_collection.get_cell_mut(*key).position = *position;
        }

        for (key, velocity) in new_velocities.iter() {
            self.cell_collection.get_cell_mut(*key).velocity = *velocity;
        }
    }

    fn handle_cell_collisions(&mut self) {
        let mut new_radii = HashMap::new();
        let mut new_velocities = HashMap::new();
        for pair in self.cell_collection.get_keys().combinations(2) {
            let key0 = *pair[0];
            let key1 = *pair[1];
            let cell0 = self.cell_collection.get_cell(key0);
            let cell1 = self.cell_collection.get_cell(key1);

            let mut collision_calculator =
                cell_interaction_utility::CollisionCalculator::new([cell0, cell1], [key0, key1]);
            collision_calculator.calculate();
            if collision_calculator.collision_type == CollisionType::NoCollision {
                continue;
            }

            for i in 0..2 {
                let key_i = *pair[i];
                if collision_calculator.should_delete_cell[i] {
                    self.keys_to_delete.push(key_i);
                } else {
                    new_radii.insert(key_i, collision_calculator.new_radii[i]);
                    new_velocities.insert(key_i, collision_calculator.new_velocities[i]);
                }
            }
        }

        for (key, radius) in new_radii.iter() {
            self.cell_collection.get_cell_mut(*key).radius = *radius;
        }

        for (key, velocity) in new_velocities.iter() {
            self.cell_collection.get_cell_mut(*key).velocity = *velocity;
        }
    }

    fn handle_object_deletion(&mut self) {
        if self.keys_to_delete.contains(&self.player_key) {
            panic!("PLAYER DELETED!");
        }

        for key in self.keys_to_delete.iter() {
            self.cell_collection.delete_cell(key);
        }
        self.keys_to_delete.clear();
    }
}
