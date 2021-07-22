extern crate piston_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

//use piston_window::{Button, Key, ButtonState, WindowSettings, PistonWindow, RenderEvent, ButtonEvent};
use piston_window::*;

use std::time::Instant;

use osmos_clone::game::*;

const WINDOW_SIZE: u32 = 512;

fn main() {
    let mut window: PistonWindow = piston_window::WindowSettings::new("Osmos clone", (WINDOW_SIZE, WINDOW_SIZE))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let frames_per_second = 10;
    let updates_per_second = 10;
    let mut previous_now = Instant::now();
    let mut now = Instant::now();

    let mut game = Game::new();

    window.set_max_fps(frames_per_second);
    window.set_ups(updates_per_second);

    //while let Some(event) = events.next(&mut window) {
    while let Some(event) = window.next() {

        if let Some(_) = event.render_args() {
            previous_now = now;
            now = Instant::now();
            let time_elapsed = now - previous_now;
            println!("tick!! {}", time_elapsed.as_secs_f32().to_string());

            game.draw(&mut window, &event);
        }        
                
        if let Some(button_args) = event.button_args() {
            game.handle_button_args(button_args, &window);
        }
    }
}
