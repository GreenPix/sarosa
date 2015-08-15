use animation::PlayerAnimator;
use animation::AnimationManager;
use animation::TextureId;
use cgmath::Vector2;

pub struct Player {
    pub position: Vector2<f32>,
    pub speed: Vector2<f32>,
    pub animator: PlayerAnimator,
}

pub type PlayerId = u64;

pub const THIS_PLAYER: PlayerId = 0;


impl Player {

    pub fn new(
        pos: Vector2<f32>,
        speed: Vector2<f32>,
        tex_id: TextureId,
        anim_manager: &AnimationManager) -> Player
    {
        Player {
            position: pos,
            speed: speed,
            animator: PlayerAnimator::new(tex_id, anim_manager)
        }
    }
}
