// Dependencies
use cgmath::Vector2;
use self::frame::TimeBasedAnimator;
use self::frame::FrameId;

// Re-exports
pub use self::player::PlayerAnimator;
pub use self::player::TextureId;
pub use self::player::AbsoluteTextureId;

// Sub modules
mod frame;
mod player;


// Old format for animation:
// Should not be exported, it is private to the sub modules.
const FRAMES_PER_TEXTURE :u32 = 9 * 4;
const NB_FRAMES: FrameId = 3;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct SubTextureId(u8);
type OldAnimator = TimeBasedAnimator<SubTextureId>;


pub struct AnimationManager {
    up_animator: OldAnimator,
    down_animator: OldAnimator,
    left_animator: OldAnimator,
    right_animator: OldAnimator,
}


macro_rules! anim_dir {
    (
        [$($frame:expr),*],
        $init_frame:expr,
        $animation_total_time:expr) =>
    {
        TimeBasedAnimator::new(
            &[
                $(SubTextureId($frame),)*
            ],
            $init_frame,
            $init_frame,
            $animation_total_time)
    }
}

impl AnimationManager {

    pub fn new() -> AnimationManager {
        // anim_total_time = (0.33 * 0.2 / 0.3225) * 10e9 ns
        // 2046511628
        let up    = anim_dir!([ 0,  1,  2], 1, 0.25 * 0.2);
        let right = anim_dir!([ 9, 10, 11], 1, 0.33 * 0.2);
        let down  = anim_dir!([18, 19, 20], 1, 0.25 * 0.2);
        let left  = anim_dir!([27, 28, 29], 1, 0.33 * 0.2);
        AnimationManager {
            up_animator: up,
            down_animator: down,
            left_animator: left,
            right_animator: right,
        }
    }

    #[inline]
    fn up_idle_frame(&self) -> SubTextureId     { SubTextureId(1) }
    #[inline]
    fn right_idle_frame(&self) -> SubTextureId  { SubTextureId(10) }
    #[inline]
    fn down_idle_frame(&self) -> SubTextureId   { SubTextureId(19) }
    #[inline]
    fn left_idle_frame(&self) -> SubTextureId   { SubTextureId(28) }

    // Return the idle frame for the given animator
    fn get_idle_frame(&self, animator: &OldAnimator) -> SubTextureId {
        if animator.use_same_frames_as(&self.up_animator) {
            self.up_idle_frame()
        } else if animator.use_same_frames_as(&self.right_animator) {
            self.right_idle_frame()
        } else if animator.use_same_frames_as(&self.left_animator) {
            self.left_idle_frame()
        } else {
            self.down_idle_frame()
        }
    }

    // Update the given animator if it does match the given direction.
    // Otherwise do nothing.
    fn update_animator(&self, animator: &mut OldAnimator, direction: &Vector2<f32>) {
        if direction.y.abs() > direction.x.abs() {
            if direction.y > 0f32 {
                let test = animator.use_same_frames_as(&self.up_animator);
                if !test {
                    trace!("Animator changed to up");
                    *animator = self.up_animator.clone();
                }
            } else {
                let test = animator.use_same_frames_as(&self.down_animator);
                if !test {
                    trace!("Animator changed to down");
                    *animator = self.down_animator.clone();
                }
            }
        } else {

            if direction.x < 0f32 {
                let test = animator.use_same_frames_as(&self.left_animator);
                if !test {
                    trace!("Animator changed to left");
                    *animator = self.left_animator.clone();
                }
            } else {
                let test = animator.use_same_frames_as(&self.right_animator);
                if !test {
                    trace!("Animator changed to right");
                    *animator = self.right_animator.clone();
                }
            }
        }
    }
}
