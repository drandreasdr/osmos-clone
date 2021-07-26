use super::scene;

pub fn integrate(scene: &mut scene::Scene, time_step: f64) {
    scene.player.position += time_step * scene.player.velocity;

    scene.enemy.position += time_step * scene.enemy.velocity;
}
