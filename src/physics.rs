use super::scene;

pub fn integrate(scene: &mut scene::Scene, time_step: f64) {
    scene.cell_collection.get_player_mut().position =
        scene.cell_collection.get_player_mut().position
            + time_step * scene.cell_collection.get_player().velocity;

    for enemy in scene.cell_collection.get_enemies_mut().iter_mut() {
        enemy.position += time_step * enemy.velocity;
    }
}
