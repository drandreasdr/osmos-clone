extern crate graphics;
extern crate opengl_graphics;

use piston_window::*;

pub struct Ring {
    pub x: f64,
    pub y: f64,
    radius: f64,
    thickness: f64,
    colour: Colour
}

type Colour = [f32; 4];

impl Ring {
    pub fn new(x: f64,
               y: f64,
               radius: f64,
               colour: Colour) -> Self {
        let thickness = radius/10.0;
        Ring {
            x,
            y,
            radius,
            thickness,
            colour
        }
    }

    pub fn draw(&self,
                transform: graphics::math::Matrix2d,
                graphics: &mut G2d) {
        ellipse(self.colour, [self.x, self.y, self.radius, self.radius],
            transform,
            graphics);
    }
}
