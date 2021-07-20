extern crate graphics;
extern crate opengl_graphics;

pub struct Ring {
    xPosition: f64,
    yPosition: f64,
    radius: f64,
    thickness: f64,
    ellipse: graphics::Ellipse
}

impl Ring {
    pub fn new(xPosition: f64,
               yPosition: f64,
               radius: f64,
               color: [f32; 4]) -> Self {
        let thickness = radius/10.0;
        let ellipse = graphics::Ellipse::new_border(color, thickness);
        Ring {
            xPosition,
            yPosition,
            radius,
            thickness,
            ellipse
        }
    }

    pub fn draw(&self,
                draw_state: &graphics::DrawState,
                transform: graphics::math::Matrix2d,
                graphics: &mut opengl_graphics::GlGraphics) {
        self.ellipse.draw([self.xPosition + self.thickness,
                           self.yPosition + self.thickness,
                           self.radius - self.thickness*2.0,
                           self.radius - self.thickness*2.0],
                           draw_state,
                           transform,
                           graphics);
    }
}
