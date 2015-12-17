use glium::Surface;
use glium::draw_parameters::DrawParameters;
use glium::draw_parameters::Blend;

use Window;
use models::game::GameData;
use rendering::scene::WorldScene;
use self::map::MapRenderer;
use self::players::PlayersRenderer;

mod shaders;
mod map;
mod players;

pub struct GameRenderer {
    players_renderer: PlayersRenderer,
    map_renderer: MapRenderer,
}

impl GameRenderer {

    pub fn new(window: &Window) -> GameRenderer {

        GameRenderer {
            players_renderer: PlayersRenderer::new(window),
            map_renderer: MapRenderer::new(window),
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
        let transform = window.projection() * world_scene.transform();

        // Preparing the frame
        let mut target = window.display.draw();

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        // Clear the screen
        target.clear_color(0.0, 0.0, 0.0, 1.0); //target.clear_color(0.11, 0.31, 0.11, 1.0);

        // Draw the map
        self.map_renderer.render(&mut target, &transform, &draw_parameters);
        // Draw the players
        self.players_renderer.render(&mut target, &transform, &draw_parameters);

        // Swap buffers
        target.finish().unwrap();
    }
}
