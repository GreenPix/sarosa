use cgmath::Vector2;
use cgmath::ApproxEq;
use cgmath::EuclideanVector;
use super::FRAMES_PER_TEXTURE;
use super::SubTextureId;
use super::OldAnimator;
use super::AnimationManager;

#[derive(Debug, Copy, Clone)]
pub struct TextureId(pub u32);

#[derive(Debug, Copy, Clone)]
pub struct AbsoluteTextureId(pub u32);

pub struct PlayerAnimator {
    current_animator: OldAnimator,
    tex_id: TextureId,
    idle_frame: Option<SubTextureId>,
}

impl PlayerAnimator {

    pub fn new(tex_id: TextureId, anim_manager: &AnimationManager) -> PlayerAnimator {
        PlayerAnimator {
            current_animator: anim_manager.down_animator.clone(),
            tex_id: tex_id,
            idle_frame: Some(anim_manager.down_idle_frame()),
        }
    }

    pub fn absolute_tex_id(&self) -> AbsoluteTextureId {
        match self.idle_frame {
            Some(frame) => {
                absolute_tex_id(self.tex_id, frame)
            }
            None => {
                let frame = self.current_animator.current_frame();
                absolute_tex_id(self.tex_id, frame)
            }
        }
    }

    pub fn update(
        &mut self,
        anim_manager: &AnimationManager,
        time_elapsed: u64,
        speed: &Vector2<f32>)
    {
        if speed.length2().approx_eq(&0f32) {
            let idle_frame = anim_manager.get_idle_frame(&self.current_animator);
            self.idle_frame = Some(idle_frame);
        } else {
            self.idle_frame = None;
            anim_manager.update_animator(&mut self.current_animator, &speed);
            self.current_animator.next_frame(time_elapsed, speed.length());
        }
    }
}

fn absolute_tex_id(tex_id: TextureId, frame: SubTextureId) -> AbsoluteTextureId {
    let TextureId(tex_id) = tex_id;
    let SubTextureId(frame) = frame;
    AbsoluteTextureId(tex_id * FRAMES_PER_TEXTURE + frame as u32)
}
