
use cgmath::Matrix4;

use models::game::GameData;
use rendering::camera::Camera;

pub struct WorldScene {
    camera: Camera,
}

impl WorldScene {

    pub fn new() -> WorldScene {
        WorldScene {
            camera: Camera::new()
        }
    }

    pub fn camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn update_world(&mut self, game_data: &GameData) {
        if let Some(player) = game_data.this_player() {
            self.camera.track(&player.position);
        }
    }

    pub fn transform(&self) -> &Matrix4<f32> {
        self.camera.as_uniform()
    }
}
