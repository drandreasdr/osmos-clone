extern crate graphics;
extern crate opengl_graphics;

pub struct Ring {
    x_position: f64,
    y_position: f64,
    radius: f64,
    thickness: f64,
    colour: Colour
}

type Colour = [f32; 4];

impl Ring {
    pub fn new(x_position: f64,
               y_position: f64,
               radius: f64,
               colour: Colour) -> Self {
        let thickness = radius/10.0;
        Ring {
            x_position,
            y_position,
            radius,
            thickness,
            colour
        }
    }

    pub fn draw(&self,
                draw_state: &graphics::DrawState,
                transform: graphics::math::Matrix2d,
                graphics: &mut opengl_graphics::GlGraphics) {
        graphics::Ellipse::new_border(self.colour, self.thickness).draw([self.x_position + self.thickness,
                           self.y_position + self.thickness,
                           self.radius - self.thickness*2.0,
                           self.radius - self.thickness*2.0],
                           draw_state,
                           transform,
                           graphics);
    }
}
