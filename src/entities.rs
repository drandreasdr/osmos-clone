use super::constants;
use nalgebra::Vector2;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;
use std::collections::hash_map::HashMap;
use std::collections::hash_map::Keys;
use std::collections::hash_map::Values;
use std::collections::hash_map::ValuesMut;

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
    pub cell_collection: Option<CellCollection>,
    pub player_index: Option<i32>,
}

impl CellCollectionFactory {
    pub fn new() -> Self {
        CellCollectionFactory {
            cell_collection: None,
            player_index: None,
        }
    }

    pub fn generate(&mut self) {
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
