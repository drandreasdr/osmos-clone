use super::cell_interaction_utility;
use super::constants;
use super::entities::*;
use super::input_handler;
use super::physics;
use itertools::Itertools;
use nalgebra::Vector2;
extern crate graphics;
extern crate opengl_graphics;

use piston_window::*;

pub struct Scene {
    pub window_size: (f64, f64),
    pub cell_collection: CellCollection,
    pub direction_marker: DirectionMarker,
}

impl Scene {
    pub fn new(window_size: (f64, f64)) -> Self {
        let mut cell_collection = CellCollection::new();
        cell_collection.add_cell(Cell::new(
            CellType::Player,
            Vector2::<f64>::new(300.0, 300.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
            constants::RED,
        ));

        cell_collection.add_cell(Cell::new(
            CellType::NonPlayer,
            Vector2::<f64>::new(50.0, 50.0),
            Vector2::<f64>::new(0.0, 0.0),
            20.0,
            constants::YELLOW,
        ));
        cell_collection.add_cell(Cell::new(
            CellType::NonPlayer,
            Vector2::<f64>::new(240.0, 240.0),
            Vector2::<f64>::new(1.0, 0.0),
            20.0,
            constants::BLUE,
        ));

        let direction_marker = DirectionMarker::new(10.0);

        Scene {
            window_size,
            cell_collection,
            direction_marker,
        }
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

        physics::integrate(self, dt);
    }

    pub fn render(&self, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
        let player = self.cell_collection.get_player();
        let enemies = self.cell_collection.get_enemies();

        player.render(transform, graphics);
        self.direction_marker.render(transform, graphics);
        for enemy in enemies.iter() {
            enemy.render(transform, graphics);
        }
    }

    fn get_direction_marker_position(&self, mouse_position: Vector2<f64>) -> Vector2<f64> {
        let player = self.cell_collection.get_player();
        let aim_direction = (mouse_position - player.position).normalize();
        player.position + aim_direction * player.radius
    }

    fn handle_ejection(&mut self, mouse_position: Vector2<f64>) {
        let player = self.cell_collection.get_player();

        let new_cell = cell_interaction_utility::create_ejected_particle(
            mouse_position,
            player.position,
            player.velocity,
            player.radius,
        );

        let new_player_velocity = cell_interaction_utility::get_new_player_velocity_after_ejection(
            player.velocity,
            player.radius,
            new_cell.velocity,
        );

        self.cell_collection.add_cell(new_cell);
        self.cell_collection.get_player_mut().velocity = new_player_velocity;
    }

    fn handle_wall_bounce(&mut self) {
        let player = self.cell_collection.get_player_mut();

        let velocity_factors = cell_interaction_utility::get_velocity_factors_for_wall_bounce(
            player.position,
            player.radius,
            self.window_size,
        );
        player.velocity[0] *= velocity_factors[0];
        player.velocity[1] *= velocity_factors[1];

        for enemy in self.cell_collection.get_enemies_mut().iter_mut() {
            let velocity_factors = cell_interaction_utility::get_velocity_factors_for_wall_bounce(
                enemy.position,
                enemy.radius,
                self.window_size,
            );
            enemy.velocity[0] *= velocity_factors[0];
            enemy.velocity[1] *= velocity_factors[1];
        }
    }

    fn handle_cell_collisions(&self) {
        //Handle merge:
        println!("---");
        for pair in self.cell_collection.cells.iter().combinations(2) {
            let cell1 = pair[0];
            let cell2 = pair[1];
            println!("{:?}, {:?}", cell1.position, cell2.position);
            if cell1.overlaps_with(&cell2) {
                println!("{:?} overlaps with {:?}", cell1, cell2);
            }
        }
        //Check all against all cells for overlaps
        //For each overlap:
        // find the new radii such that the cells are exactly tangent to each other, and the corresponding transferred area
        // find the new velocities
    }
}
