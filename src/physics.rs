use super::scene;

pub fn integrate(scene: &mut scene::Scene, time_step: f64) {
    for enemy in scene.cell_collection.get_cells_mut() {
        enemy.position += time_step * enemy.velocity;
    }
}
