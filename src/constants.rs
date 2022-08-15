pub type Color = [f32; 4];

pub const WINDOW_SIZE: u32 = 800;

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const WHITE: Color = [1.0; 4];
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const TRANSPARENT: Color = [0.0, 0.0, 0.0, 0.0];
pub const COLOR_OF_PLAYER: Color = WHITE;
pub const COLOR_OF_CELL_DOMINATED_BY_PLAYER: Color = GREEN;
pub const COLOR_OF_CELL_DOMINATING_PLAYER: Color = RED;

pub const RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE: f64 = 50.0;
pub const RADIUS_EJECTED_PARTICLE: f64 = 10.0;

#[derive(Eq, PartialEq)]
pub enum CollisionType {
    NoCollision,
    FullMerge,
    PartialMerge,
}

#[derive(Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub enum GameSpeed {
    SLOW,
    NORMAL,
    FAST,
}

#[derive(Copy, Clone)]
pub enum GameAction {
    SetGameSpeed(GameSpeed),
    EjectCell([f64; 2]),
}
