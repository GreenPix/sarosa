use glium::Surface;
use glium::draw_parameters::DrawParameters;
use glium::draw_parameters::BlendingFunction::Addition;
use glium::draw_parameters::LinearBlendingFactor::{
    SourceAlpha,
    OneMinusSourceAlpha,
};

use Window;
use models::game::GameData;
use rendering::scene::WorldScene;
use self::players::PlayersRenderer;

mod shaders;
mod players;

pub struct GameRenderer {
    players_renderer: PlayersRenderer,
}

impl GameRenderer {

    pub fn new(window: &Window) -> GameRenderer {

        GameRenderer {
            players_renderer: PlayersRenderer::new(window),
        }
    }

    pub fn initialize_gpu_mem(&mut self, game_data: &GameData,  window: &Window) {

        self.map_renderer.initialize_gpu_mem(game_data, window);
    }

    pub fn update_gpu_mem(&mut self, game_data: &GameData) {

        self.players_renderer.update_gpu_mem(game_data);
    }

    pub fn render(&self, world_scene: &WorldScene, window: &mut Window) {

        // Compute the projection matrix:
        let transform = (*window.projection()) * (*world_scene.transform());

        // Preparing the frame
        let mut target = window.display.draw();

        let draw_parameters = DrawParameters {
            blending_function: Some(
                Addition { source: SourceAlpha, destination: OneMinusSourceAlpha }
            ),
            .. Default::default()
        };

        // Clear the screen
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Draw the map
        self.map_renderer.render(&mut target, &transform, &draw_parameters);
        // Draw the players
        self.players_renderer.render(&mut target, &transform, &draw_parameters);

        // Swap buffers
        target.finish().unwrap();
    }
}
