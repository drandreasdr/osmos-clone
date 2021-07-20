extern crate graphics;
extern crate opengl_graphics;

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
                draw_state: &graphics::DrawState,
                transform: graphics::math::Matrix2d,
                graphics: &mut opengl_graphics::GlGraphics) {
        graphics::Ellipse::new_border(self.colour, self.thickness).draw([self.x + self.thickness,
                           self.y + self.thickness,
                           self.radius - self.thickness*2.0,
                           self.radius - self.thickness*2.0],
                           draw_state,
                           transform,
                           graphics);
    }
}
