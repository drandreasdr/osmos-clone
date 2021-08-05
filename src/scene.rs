use super::collisions;
use super::constants;
use super::entities::*;
use super::input_handler;
use super::physics;
use nalgebra::Vector2;
extern crate graphics;
extern crate opengl_graphics;
use piston_window::*;

pub struct Scene {
    pub player: Player,
    pub direction_marker: DirectionMarker,
    pub enemies: Vec<Enemy>,
}

impl Scene {
    pub fn new() -> Self {
        let player = Player::new(
            Vector2::<f64>::new(300.0, 300.0),
            Vector2::<f64>::new(0.0, 0.0),
            50.0,
            constants::RED,
        );

        let direction_marker = DirectionMarker::new(10.0);

        let enemies = vec![
            Enemy::new(
                Vector2::<f64>::new(50.0, 50.0),
                Vector2::<f64>::new(0.0, 0.0),
                20.0,
                constants::YELLOW,
            ),
            Enemy::new(
                Vector2::<f64>::new(50.0, 150.0),
                Vector2::<f64>::new(1.0, 0.0),
                20.0,
                constants::BLUE,
            ),
        ];

        Scene {
            player,
            direction_marker,
            enemies,
        }
    }

    pub fn update(&mut self, dt: f64, input_handler: &input_handler::InputHandler) {
        physics::integrate(self, dt);

        let vector_toward_mouse_cursor =
            (input_handler.mouse_position - self.player.position).normalize();
        let direction_marker_radius = 10.0;
        self.direction_marker.position =
            self.player.position + vector_toward_mouse_cursor * self.player.radius;
        let new_enemy_start_position = self.player.position
            + vector_toward_mouse_cursor * (self.player.radius + direction_marker_radius);

        //let new_enemy_velocity =

        for input_action in input_handler.input_actions.iter() {
            if let input_handler::InputAction::LeftMouseClick(mouse_coordinates) = input_action {
                //Ejection event:
                let (velocity_ejected_particle, new_velocity_player) =
                    collisions::calculate_ejection(&self.player, *mouse_coordinates);
                self.enemies.push(Enemy::new(
                    new_enemy_start_position,
                    velocity_ejected_particle,
                    collisions::RADIUS_EJECTED_PARTICLE,
                    constants::WHITE,
                ));
                self.player.velocity = new_velocity_player;
            }
        }
    }

    pub fn render(&self, transform: graphics::math::Matrix2d, graphics: &mut G2d) {
        self.player.render(transform, graphics);
        self.direction_marker.render(transform, graphics);
        for enemy in self.enemies.iter() {
            enemy.render(transform, graphics);
        }
    }
}
