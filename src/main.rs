extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;

use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent};

use opengl_graphics::{GlGraphics, OpenGL};

mod ring;

type Colour = [f32; 4];

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const YELLOW: Colour = [1.0, 1.0, 0.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const TRANSPARENT: Colour = [0.0, 0.0, 0.0, 0.0];

const WINDOW_SIZE: i32 = 512;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    let mut player = ring::Ring::new(0.0, 0.0, 50.0, RED);
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |context, gl_graphics| {
                graphics::clear(BLACK, gl_graphics);

                let rr = ring::Ring::new((WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.25, YELLOW);
                rr.draw(&context.draw_state,
                        context.transform,
                        gl_graphics);

                player.draw(&context.draw_state,
                            context.transform,
                            gl_graphics);
            });
        }
        if let Some(k) = event.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => player.y -= 10.0,
                    Button::Keyboard(Key::Down) => player.y += 10.0,
                    Button::Keyboard(Key::Left) => player.x -= 10.0,
                    Button::Keyboard(Key::Right) => player.x += 10.0,
                    _ => (),
                }
            }
        }
    }
}
