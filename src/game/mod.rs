use events::{
    EventSystem,
    UserEventType,
};
use rendering::GameRenderer;
use rendering::scene::WorldScene;
use Window;
use Settings;

pub use self::loops::GameLoop;
mod loops;

pub enum GameRunState {
    Running,
    Stopped,
}

pub struct GameInstance {
    renderer: GameRenderer,
    world_scene: WorldScene,
}

impl GameInstance {

    pub fn new(window: &Window, _: Settings) -> GameInstance {
        GameInstance {
            renderer: GameRenderer::new(window),
            world_scene: WorldScene::default()
        }
    }

    fn event_update(&mut self, event_sys: &EventSystem) -> GameRunState {
        for &e in event_sys.iter() {
            match e.kind {
                UserEventType::Quit => return GameRunState::Stopped,
                _ => panic!("TODO"),
            }
        }
        GameRunState::Running
    }

    fn frame_update(&mut self, window: &mut Window) {
        // TODO
        self.renderer.do_random_stuff();
        self.renderer.render(&self.world_scene, window);
    }

    fn fixed_update(&mut self, _: u64) { //fixed_timestamp: u64) {
        // TODO
    }

}
