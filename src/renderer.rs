use super::scene;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;

pub fn render(scene: &scene::Scene, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
    let player = scene.cell_collection.get_player();
    let enemies = scene.cell_collection.get_enemies();

    ellipse(
        player.color,
        [
            player.position[0] - player.radius,
            player.position[1] - player.radius,
            player.radius * 2.0,
            player.radius * 2.0,
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
