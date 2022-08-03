use super::constants;
use nalgebra::Vector2;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;
use rand::prelude::*;
use std::collections::hash_map::HashMap;
use std::collections::hash_map::Keys;
use std::collections::hash_map::Values;
use std::collections::hash_map::ValuesMut;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Cell {
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub radius: f64,
    pub color: constants::Color,
}

impl Cell {
    pub fn new(position: Vector2<f64>, velocity: Vector2<f64>, radius: f64) -> Self {
        Cell {
            position,
            velocity,
            radius,
            color: constants::WHITE,
        }
    }

    pub fn render(&self, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
        ellipse(
            self.color,
            [
                self.position[0] - self.radius,
                self.position[1] - self.radius,
                self.radius * 2.0,
                self.radius * 2.0,
            ],
            transform,
            graphics,
        );
    }
}

pub struct CellCollection {
    cells: HashMap<i32, Cell>, //The key is an index that is guaranteed to increase with creation order - higher index for newer cells
    largest_index: i32,
}

impl CellCollection {
    pub fn new() -> CellCollection {
        CellCollection {
            cells: HashMap::new(),
            largest_index: 0,
        }
    }

    fn generate_index(&mut self) -> i32 {
        self.largest_index += 1;
        self.largest_index
    }

    pub fn add_cell(&mut self, cell: Cell) -> i32 {
        let index = self.generate_index();
        self.cells.insert(index, cell);
        index
    }

    pub fn delete_cell(&mut self, index: &i32) {
        self.cells.remove(&index);
    }

    pub fn get_indices(&self) -> Keys<i32, Cell> {
        self.cells.keys()
    }

    pub fn get_cell_mut(&mut self, index: i32) -> &mut Cell {
        self.cells.get_mut(&index).unwrap()
    }

    pub fn get_cells_mut(&mut self) -> ValuesMut<i32, Cell> {
        self.cells.values_mut()
    }

    pub fn get_cell(&self, index: i32) -> &Cell {
        self.cells.get(&index).unwrap()
    }

    pub fn get_cells(&self) -> Values<i32, Cell> {
        self.cells.values()
    }
}

pub struct DirectionMarker {
    pub position: Vector2<f64>,
    pub direction: Vector2<f64>,
    pub length: f64,
    pub color: constants::Color,
}

pub struct CellCollectionFactory {
    window_size: (f64, f64),
    pub cell_collection: Option<CellCollection>,
    pub player_index: Option<i32>,
}

impl CellCollectionFactory {
    pub fn new(window_size: (f64, f64)) -> Self {
        CellCollectionFactory {
            window_size,
            cell_collection: None,
            player_index: None,
        }
    }

    pub fn generate(&mut self) {
        //self.generate_custom();
        self.generate_parametric();
    }
    pub fn generate_custom(&mut self) {
        let mut cell_collection = CellCollection::new();

        let player_index = cell_collection.add_cell(Cell::new(
            Vector2::<f64>::new(300.0, 300.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
        ));

        cell_collection.add_cell(Cell::new(
            Vector2::<f64>::new(50.0, 50.0),
            Vector2::<f64>::new(0.0, 0.0),
            20.0,
        ));
        cell_collection.add_cell(Cell::new(
            Vector2::<f64>::new(240.0, 240.0),
            Vector2::<f64>::new(1.0, 0.0),
            20.0,
        ));

        self.cell_collection = Some(cell_collection);
        self.player_index = Some(player_index);
    }

    pub fn generate_parametric(&mut self) {
        let mut rng = rand::thread_rng();
        let mut cell_collection = CellCollection::new();

        let radius_limits = [20.0, 60.0];
        let player_relative_radius = 0.8;
        let speed_limits = [5.0, 20.0];
        let target_fill_ratio = 0.2;

        let player_radius =
            radius_limits[0] + player_relative_radius * (radius_limits[1] - radius_limits[0]);

        let player_index = cell_collection.add_cell(Cell::new(
            self.get_random_position_within_scene(player_radius, &mut rng),
            Vector2::<f64>::new(0.0, 0.0),
            player_radius,
        ));

        let mut total_cell_area = player_radius.powf(2.0) * PI;
        let total_scene_area = self.window_size.0 * self.window_size.1;

        while total_cell_area / total_scene_area < target_fill_ratio {
            let radius: f64 = rng.gen_range(radius_limits[0]..radius_limits[1]);
            let speed: f64 = rng.gen_range(speed_limits[0]..speed_limits[1]);
            cell_collection.add_cell(Cell::new(
                self.get_random_position_within_scene(radius, &mut rng),
                self.get_random_velocity(speed, &mut rng),
                radius,
            ));

            total_cell_area += radius.powf(2.0) * PI;
        }

        self.cell_collection = Some(cell_collection);
        self.player_index = Some(player_index);
    }

    fn get_random_position_within_scene(&self, radius: f64, rng: &mut ThreadRng) -> Vector2<f64> {
        let width = self.window_size.0 - 2.0 * radius;
        let height = self.window_size.1 - 2.0 * radius;
        let x: f64 = rng.gen_range(radius..radius + width);
        let y: f64 = rng.gen_range(radius..radius + height);
        Vector2::<f64>::new(x, y)
    }

    fn get_random_velocity(&self, magnitude: f64, rng: &mut ThreadRng) -> Vector2<f64> {
        let angle: f64 = rng.gen_range(0.0..2.0 * PI);
        let x = angle.cos() * magnitude;
        let y = angle.sin() * magnitude;
        Vector2::<f64>::new(x, y)
    }
}

impl DirectionMarker {
    pub fn new(length: f64) -> Self {
        DirectionMarker {
            position: Vector2::<f64>::new(0.0, 0.0),
            direction: Vector2::<f64>::new(0.0, 0.0),
            length: length,
            color: constants::WHITE,
        }
    }

    pub fn render(&self, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
        rectangle(
            self.color,
            [self.position[0], self.position[1], self.length, self.length],
            transform,
            graphics,
        );
    }
}
