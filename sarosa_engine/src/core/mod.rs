use events::{
    EventSystem,
    UserEventType,
};
use models::game::GameData;
use models::player::{
    Player,
    PlayerId
};
use rendering::GameRenderer;
use rendering::scene::WorldScene;
use Window;
use Settings;

pub use self::loops::GameLoop;
pub use self::loops::LoopState;
pub mod loops;

pub struct GameInstance {
    renderer: GameRenderer,
    world_scene: WorldScene,
    game_data: GameData,
    // ui_router: oil::Router,
}

pub struct GameDataRefMut<'a> {
    should_require_gpu_init: bool,
    game_data: &'a mut GameData,
    renderer: &'a mut GameRenderer,
}

impl<'a> GameDataRefMut<'a> {

    pub fn add_player(&mut self, player: Player, id: PlayerId) {
        let is_new = self.game_data.add_player(player, id);
        self.should_require_gpu_init |= is_new;
    }
}

impl<'a> Drop for GameDataRefMut<'a> {
    fn drop(&mut self) {
        if self.should_require_gpu_init {
            self.renderer.initialize_gpu_mem(self.game_data);
        }
    }
}

impl GameInstance {

    pub fn new(window: &Window, _: Settings) -> GameInstance {
        GameInstance {
            renderer: GameRenderer::new(window),
            world_scene: WorldScene::default(),
            game_data: GameData::new(),
        }
    }

    pub fn game_data<'a>(&'a mut self) -> GameDataRefMut<'a> {
        GameDataRefMut {
            should_require_gpu_init: false,
            game_data: &mut self.game_data,
            renderer: &mut self.renderer,
        }
    }

    fn event_update(&mut self, event_sys: &EventSystem) -> LoopState {
        for &e in event_sys.iter() {
            match e.kind {
                UserEventType::Quit => return LoopState::Break,
                _ => (),
            }
        }
        LoopState::Continue
    }

    fn frame_update(&mut self, window: &mut Window) {
        self.renderer.update_gpu_mem(&self.game_data);
        self.renderer.render(&self.world_scene, window);
    }

    fn fixed_update(&mut self, fixed_timestamp: u64) {
        self.renderer.fixed_update(fixed_timestamp);
    }

}
