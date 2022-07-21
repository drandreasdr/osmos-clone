pub type Color = [f32; 4];

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const WHITE: Color = [1.0; 4];
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const TRANSPARENT: Color = [0.0, 0.0, 0.0, 0.0];

pub const RELATIVE_VELOCITY_MAGNITUDE_EJECTED_PARTICLE: f64 = 50.0;
pub const RADIUS_EJECTED_PARTICLE: f64 = 10.0;

#[derive(Eq, PartialEq)]
pub enum CollisionType {
    NoCollision,
    FullMerge,
    PartialMerge,
}
