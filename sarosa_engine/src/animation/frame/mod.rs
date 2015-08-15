

use self::old::FrameAnimator;
use animation::NB_FRAMES;

pub use self::old::FrameId;

// This module contains the old format of the assets
// we currently have. It contains code to interpret
// the convention encoded into the images used for animations.
mod old;

#[derive(Clone)]
pub struct TimeBasedAnimator<F: Copy> {
    frame_animator: FrameAnimator<F>,
    animation_total_time: u64,
    accumulated_time: u64,
}


impl<F: Copy + Eq> TimeBasedAnimator<F> {

    pub fn new(
        frames: &[F; NB_FRAMES as usize],
        init_frame: FrameId,
        end_frame: FrameId,
        animation_total_time: u64
        ) -> TimeBasedAnimator<F>
    {
        let mut frame_animator = FrameAnimator::new(frames, init_frame);
        frame_animator.set_end_frame(end_frame);
        TimeBasedAnimator {
            frame_animator: frame_animator,
            animation_total_time: animation_total_time,
            accumulated_time: 0,
        }
    }

    #[inline]
    pub fn use_same_frames_as(&self, other: &TimeBasedAnimator<F>) -> bool {
        self.frame_animator.use_same_frames_as(&other.frame_animator)
    }

    #[inline]
    pub fn current_frame(&self) -> F {
        self.frame_animator.current_frame()
    }

    #[inline]
    pub fn next_frame(&mut self, time_elapsed: u64) -> F {
        if self.frame_animator.is_finished() {
            self.frame_animator.reset();
        }
        let frame = self.frame_animator.current_frame();

        self.accumulated_time += time_elapsed;
        let frame_time = self.animation_total_time / (NB_FRAMES as u64);

        while self.accumulated_time > frame_time {
            self.frame_animator.next_frame();
            self.accumulated_time = self.accumulated_time.saturating_sub(frame_time);
        }

        frame
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn next_frame() {}
}
