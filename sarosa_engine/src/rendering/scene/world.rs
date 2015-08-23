use rendering::camera::Camera;
use rendering::scene::WorldScene;
use models::game::GameData;


impl WorldScene {

    pub fn new(window: &Window, game_data: &GameData) -> WorldScene {
        WorldScene {
            camera: Camera::new(),
            map: Map::new(window, game_data),
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
