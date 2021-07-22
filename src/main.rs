extern crate piston_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use piston::event_loop::{EventLoop, EventSettings, Events};
//use piston_window::{Button, Key, ButtonState, WindowSettings, PistonWindow, RenderEvent, ButtonEvent};
use piston_window::*;

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
    let mut window: PistonWindow = piston_window::WindowSettings::new("Osmos clone", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let frames_per_second = 10;
    let mut previous_now = Instant::now();
    let mut now = Instant::now();

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(frames_per_second);
    let mut player = ring::Ring::new(0.0, 0.0, 50.0, RED);
    let rr = ring::Ring::new((WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.25, YELLOW);

    //while let Some(event) = events.next(&mut window) {
    while let Some(event) = window.next() {
        previous_now = now;
        now = Instant::now();
        let time_elapsed = now - previous_now;
        println!("tick!! {}", time_elapsed.as_secs_f32().to_string());

        if let Some(_) = event.render_args() {
            //game.render(&mut window, &e);
            window.draw_2d(&event, |context, graphics, _| {
                clear(BLACK, graphics);
    
                previous_now = now;
                now = Instant::now();
                let time_elapsed = now - previous_now;
                println!("tick!! {}", time_elapsed.as_secs_f32().to_string());
    
                //update(time_elapsed)
                rr.draw(context.transform,
                        graphics);
                player.draw(context.transform,
                            graphics);
            });
        }

        
                
        if let Some(button_args) = event.button_args() {
            if button_args.state == ButtonState::Press {
                match button_args.button {
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
