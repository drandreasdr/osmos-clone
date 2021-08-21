use super::collisions;
use super::constants;
use super::entities::*;
use super::input_handler;
use super::physics;
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
        cell_collection.add_cell(Cell::new(CellType::Player,
            Vector2::<f64>::new(300.0, 300.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
            constants::RED,
        ));

        cell_collection.add_cell(Cell::new(CellType::NonPlayer,
            Vector2::<f64>::new(50.0, 50.0),
                Vector2::<f64>::new(0.0, 0.0),
                20.0,
                constants::YELLOW));
                cell_collection.add_cell(Cell::new(CellType::NonPlayer,
                    Vector2::<f64>::new(50.0, 150.0),
                Vector2::<f64>::new(1.0, 0.0),
                20.0,
                constants::BLUE,));

        let direction_marker = DirectionMarker::new(10.0);

        Scene {
            window_size,
            cell_collection,
            direction_marker,
        }
    }

    pub fn update(&mut self, dt: f64, input_handler: &input_handler::InputHandler) {
        let mut player = self.cell_collection.get_player();
        let aim_direction = (input_handler.mouse_position - player.position).normalize();
        self.direction_marker.position = player.position + aim_direction * player.radius;

        physics::integrate(self, dt);

        collisions::handle_collisions(self);

        for input_action in input_handler.input_actions.iter() {
            if let input_handler::InputAction::LeftMouseClick(mouse_coordinates) = input_action {
                collisions::handle_ejection(self, *mouse_coordinates);
            }
        }
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
}
