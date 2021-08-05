use super::scene;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;

pub fn render(scene: &scene::Scene, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
    ellipse(
        scene.player.color,
        [
            scene.player.position[0] - scene.player.radius,
            scene.player.position[1] - scene.player.radius,
            scene.player.radius * 2.0,
            scene.player.radius * 2.0,
        ],
        transform,
        graphics,
    );

    rectangle(
        scene.direction_marker.color,
        [
            scene.direction_marker.position[0],
            scene.direction_marker.position[1],
            scene.direction_marker.length,
            scene.direction_marker.length,
        ],
        transform,
        graphics,
    );

    for enemy in scene.enemies.iter() {
        ellipse(
            enemy.color,
            [
                enemy.position[0] - enemy.radius,
                enemy.position[1] - enemy.radius,
                enemy.radius * 2.0,
                enemy.radius * 2.0,
            ],
            transform,
            graphics,
        );
    }
}
