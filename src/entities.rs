use super::constants;
use nalgebra::Vector2;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;

#[derive(Debug)]
pub enum CellType {
    Player,
    NonPlayer,
}

#[derive(Debug)]
pub struct Cell {
    pub cell_type: CellType,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub radius: f64,
    pub color: constants::Color,
}

impl Cell {
    pub fn new(
        cell_type: CellType,
        position: Vector2<f64>,
        velocity: Vector2<f64>,
        radius: f64,
        color: constants::Color,
    ) -> Self {
        Cell {
            cell_type,
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
    pub cells: Vec<Cell>,
}

impl CellCollection {
    pub fn new() -> CellCollection {
        CellCollection { cells: Vec::new() }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn get_player_mut(&mut self) -> &mut Cell {
        return &mut self.cells[0];
    }

    pub fn get_player(&self) -> &Cell {
        return &self.cells[0];
    }

    pub fn get_enemies_mut(&mut self) -> &mut [Cell] {
        return &mut self.cells[1..];
    }

    pub fn get_enemies(&self) -> &[Cell] {
        return &self.cells[1..];
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
