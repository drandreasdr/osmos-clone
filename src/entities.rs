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
    pub fn new(
        position: Vector2<f64>,
        velocity: Vector2<f64>,
        radius: f64,
        color: constants::Color,
    ) -> Self {
        Cell {
            position,
            velocity,
            radius,
            color,
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

    pub fn overlaps_with(&self, other: &Cell) -> bool {
        return (self.position - other.position).norm() < self.radius + other.radius;
    }
}

pub struct CellCollection {
    cells: HashMap<i64, Cell>,
    largest_key: i64,
}

impl CellCollection {
    pub fn new() -> CellCollection {
        CellCollection {
            cells: HashMap::new(),
            largest_key: 0,
        }
    }

    fn generate_key(&mut self) -> i64 {
        self.largest_key += 1;
        return self.largest_key;
    }

    pub fn add_cell(&mut self, cell: Cell) -> i64 {
        let key = self.generate_key();
        self.cells.insert(key, cell);
        return key;
    }

    pub fn get_keys(&self) -> Keys<i64, Cell> {
        return self.cells.keys();
    }

    pub fn get_cell_mut(&mut self, key: i64) -> &mut Cell {
        return self.cells.get_mut(&key).unwrap();
    }

    pub fn get_cells_mut(&mut self) -> ValuesMut<i64, Cell> {
        return self.cells.values_mut();
    }

    pub fn get_cell(&self, key: i64) -> &Cell {
        return self.cells.get(&key).unwrap();
    }

    pub fn get_cells(&self) -> Values<i64, Cell> {
        return self.cells.values();
    }
}

pub struct DirectionMarker {
    pub position: Vector2<f64>,
    pub direction: Vector2<f64>,
    pub length: f64,
    pub color: constants::Color,
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
