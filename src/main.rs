extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent};

use graphics::character::CharacterCache;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};

use std::f64::consts;

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
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;

#[derive(Clone)]
struct Tile {
    colour: Colour,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { colour: WHITE }
    }

    pub fn wall() -> Self {
        Tile { colour: BLACK }
    }
}

#[derive(Clone)]
struct Object {
    x: i32,
    y: i32,
    character: char,
    colour: Colour,
}

impl Object {
    pub fn new(x: i32, y: i32, character: char, colour: Colour) -> Self {
        Object {
            x,
            y,
            character,
            colour,
        }
    }
}

type Map = Vec<Vec<Tile>>;

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
    map[WORLD_SIZE as usize / 2][WORLD_SIZE as usize / 2] = Tile::wall();
    map
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    let map = make_map();

    let mut events = Events::new(EventSettings::new());
    let mut player = Object::new(0, 0, '@', RED);
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            gl.draw(render_args.viewport(), |context, gl_graphics| {
                graphics::clear(BLACK, gl_graphics);

                let pos: [f64; 4] = [
                    (WINDOW_SIZE as f64)*0.15,
                    (WINDOW_SIZE as f64)*0.4,
                    (WINDOW_SIZE as f64)*0.45,
                    (WINDOW_SIZE as f64)*0.4,
                    ];
                let rect = graphics::Rectangle::new(GREEN);
                let rect2 = graphics::Rectangle::new(GREEN);
                rect.draw(
                    pos,
                    &context.draw_state,
                    context.transform,
                    gl_graphics,
                );

                let ellipse = graphics::Ellipse::new(WHITE).draw(
                        [(WINDOW_SIZE as f64)*0.15,
                        (WINDOW_SIZE as f64)*0.4,
                        (WINDOW_SIZE as f64)*0.45,
                        (WINDOW_SIZE as f64)*0.4,],
                        &context.draw_state,
                        context.transform,
                        gl_graphics,
                    );
                
                graphics::CircleArc::new(YELLOW, 1.0, 0.0, 1.99*consts::PI).draw(
                    [(WINDOW_SIZE as f64)*0.35,
                    (WINDOW_SIZE as f64)*0.55,
                    (WINDOW_SIZE as f64)*0.45,
                    (WINDOW_SIZE as f64)*0.4,],
                    &context.draw_state,
                    context.transform,
                    gl_graphics,);

                graphics::rectangle(GREEN,
                    [(WINDOW_SIZE as f64)*0.25,
                    (WINDOW_SIZE as f64)*0.45,
                    (WINDOW_SIZE as f64)*0.65,
                    (WINDOW_SIZE as f64)*0.45,],
                    context.transform,
                    gl_graphics,);
                graphics::rectangle(BLUE,
                    [(WINDOW_SIZE as f64)*0.25 + 1.0,
                    (WINDOW_SIZE as f64)*0.45 + 1.0,
                    (WINDOW_SIZE as f64)*0.65 - 2.0,
                    (WINDOW_SIZE as f64)*0.45 - 2.0,],
                    context.transform,
                    gl_graphics,);

                graphics::ellipse(RED,
                    [(WINDOW_SIZE as f64)*0.05,
                    (WINDOW_SIZE as f64)*0.35,
                    (WINDOW_SIZE as f64)*0.35,
                    (WINDOW_SIZE as f64)*0.35],
                    context.transform,
                    gl_graphics);
                graphics::ellipse(BLACK,
                    [(WINDOW_SIZE as f64)*0.05 + 1.0,
                    (WINDOW_SIZE as f64)*0.35 + 1.0,
                    (WINDOW_SIZE as f64)*0.35 - 2.0,
                    (WINDOW_SIZE as f64)*0.35 - 2.0],
                    context.transform,
                    gl_graphics);

                let xx = (WINDOW_SIZE as f64)*0.05;
                let yy = (WINDOW_SIZE as f64)*0.55;
                let radius = (WINDOW_SIZE as f64)*0.65;
                let thickness = radius/50.0;
                graphics::Ellipse::new_border(RED, thickness).draw(
                    [xx + thickness,
                    yy + thickness,
                    radius - thickness*2.0,
                    radius - thickness*2.0],
                    &context.draw_state,
                    context.transform,
                    gl_graphics,);

                graphics::Ellipse::new_border(GREEN, 1.0).draw(
                    [(WINDOW_SIZE as f64)*0.05,
                    (WINDOW_SIZE as f64)*0.55,
                    (WINDOW_SIZE as f64)*0.35,
                    (WINDOW_SIZE as f64)*0.35],
                    &context.draw_state,
                    context.transform,
                    gl_graphics,);

                let rr = ring::Ring::new((WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.35, (WINDOW_SIZE as f64)*0.25, YELLOW);
                rr.draw(&context.draw_state,
                        context.transform,
                        gl_graphics);

                use graphics::Transformed;
                let character = glyphs.character(32, player.character).unwrap();
                graphics::Image::new_color(player.colour).draw(
                    character.texture,
                    &context.draw_state,
                    context.transform.trans(player.x as f64, player.y as f64),
                    gl_graphics,
                );
            });
        }
        if let Some(k) = event.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => player.y -= PIXEL_SIZE as i32,
                    Button::Keyboard(Key::Down) => player.y += PIXEL_SIZE as i32,
                    Button::Keyboard(Key::Left) => player.x -= PIXEL_SIZE as i32,
                    Button::Keyboard(Key::Right) => player.x += PIXEL_SIZE as i32,
                    _ => (),
                }
            }
        }
    }
}
