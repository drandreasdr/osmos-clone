use super::scene;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;

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

    for enemy in scene.enemies.iter() {
        ellipse(
            enemy.color,
            [
                enemy.position[0],
                enemy.position[1],
                enemy.radius,
                enemy.radius,
            ],
            transform,
            graphics,
        );
    }
}
