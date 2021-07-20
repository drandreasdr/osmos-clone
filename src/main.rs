extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent};

use opengl_graphics::{GlGraphics, OpenGL};

use std::time::Instant;

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

    let frames_per_second = 10;
    let time_per_frame = 1.0/frames_per_second as f32;
    let mut previous_now = Instant::now();
    let mut now = Instant::now();

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(frames_per_second);
    let mut player = ring::Ring::new(0.0, 0.0, 50.0, RED);
    let rr = ring::Ring::new((WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.25, YELLOW);

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |context, gl_graphics| {
                
                previous_now = now;
                now = Instant::now();
                let time_elapsed = now - previous_now;
                println!("tick!! {}", time_elapsed.as_secs_f32().to_string());

                //update(time_elapsed)
                graphics::clear(BLACK, gl_graphics);
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
