use super::scene;

pub fn integrate(scene: &mut scene::Scene, time_step: f64) {
    for cell in scene.cell_collection.get_cells_mut() {
        cell.position += time_step * cell.velocity;
    }
}
