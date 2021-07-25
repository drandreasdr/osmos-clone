use super::scene;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;

pub struct Renderer {}

impl Renderer {
    pub fn render(scene: &scene::Scene, transform: graphics::math::Matrix2d, graphics: &mut G2d) {

        ellipse(
            scene.player.color,
            [
                scene.player.position[0],
                scene.player.position[1],
                scene.player.radius,
                scene.player.radius,
            ],
            transform,
            graphics,
        );

        ellipse(
            scene.enemy.color,
            [
                scene.enemy.position[0],
                scene.enemy.position[1],
                scene.enemy.radius,
                scene.enemy.radius,
            ],
            transform,
            graphics,
        );
    }
}
