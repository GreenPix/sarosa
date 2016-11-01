use cgmath::Vector2;
use cgmath::Zero;

use events::{
    EventSystem,
    UserEventType,
    UserEventState
};
use models::game::GameData;
use models::player::{
    Player,
    PlayerId
};
use animation::TextureId;
use animation::AnimationManager;
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
    anim_manager: AnimationManager,
    // ui_router: oil::Router,
}

pub struct GameDataRefMut<'a> {
    should_require_gpu_init: bool,
    game_data: &'a mut GameData,
    renderer: &'a mut GameRenderer,
    anim_manager: &'a AnimationManager,
}

impl<'a> GameDataRefMut<'a> {

    pub fn add_player(&mut self, id: PlayerId, initial_pos: Vector2<f32>, tex_id: TextureId) {
        debug!("Player id: {:?}", tex_id);
        let player = Player::new(initial_pos, Vector2::zero(), tex_id, self.anim_manager);
        let is_new = self.game_data.add_player(id, player);
        self.should_require_gpu_init |= is_new;
    }

    pub fn update_player(&mut self, id: PlayerId, pos: Vector2<f32>, speed: Vector2<f32>) {
        self.game_data.update_player(id, pos, speed);
        self.should_require_gpu_init = true;
    }

    pub fn remove_player(&mut self, id: PlayerId) {
        self.game_data.remove_player(id);
        self.should_require_gpu_init = true;
    }
}

impl<'a> Drop for GameDataRefMut<'a> {
    fn drop(&mut self) {
        if self.should_require_gpu_init {
            self.renderer.update_gpu_mem(self.game_data);
        }
    }
}

impl GameInstance {

    pub fn new(window: &Window, _: Settings) -> GameInstance {

        let anim_manager = AnimationManager::new();
        // TODO(Nemikolh):
        //
        //  There's a lot going on here behind the scene.
        //  Basically the GameData is going to load the data
        //  for THIS_PLAYER but it assumes that we have loaded
        //  within the GameRenderer the skin of the player
        //  at the texture id 0.
        //
        //  Probably in a near feature all those details will
        //  be given by a ResourceManager which in turns will
        //  use a local cache or something to know what is the
        //  correct skin id.
        //
        let game_data = GameData::new(TextureId(0), &anim_manager);
        let mut renderer = GameRenderer::new(window);
        renderer.initialize_gpu_mem(&game_data, window);

        GameInstance {
            renderer: renderer,
            world_scene: WorldScene::new(),
            game_data: game_data,
            anim_manager: anim_manager,
        }
    }

    pub fn proxy_add<'a>(&'a mut self) -> GameDataRefMut<'a> {
        GameDataRefMut {
            should_require_gpu_init: false,
            game_data: &mut self.game_data,
            renderer: &mut self.renderer,
            anim_manager: &self.anim_manager,
        }
    }

    fn event_update(&mut self, event_sys: &EventSystem) -> LoopState {
        for &e in event_sys.iter() {
            if e.state == UserEventState::Start {
                match e.kind {
                    UserEventType::Quit => return LoopState::Break,
                    UserEventType::ZoomIn => self.world_scene.camera().zoom_in(),
                    UserEventType::ZoomOut => self.world_scene.camera().zoom_out(),
                    _ => (),
                }
            }
        }
        LoopState::Continue
    }

    fn frame_update(&mut self, window: &mut Window) {
        self.world_scene.update_world(&self.game_data);
        //self.renderer.update_gpu_mem(&self.game_data);
        self.renderer.render(&self.world_scene, window);
    }

    fn fixed_update(&mut self, fixed_timestamp: u64) {
        self.game_data.fixed_update(&self.anim_manager, fixed_timestamp);
    }

}
