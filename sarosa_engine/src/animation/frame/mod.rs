

use self::old::FrameId;
use self::old::FrameAnimator;
use self::old::NB_FRAMES;

// This module contains the old format of the assets
// we currently have. It contains code to interpret
// the convention encoded into the images used for animations.
mod old;

pub struct TimeBasedAnimator<F> {
    frame_animator: FrameAnimator<F>,
    animation_total_time: u64,
    accumulated_time: u64,
}


impl<F: Copy> TimeBasedAnimator<F> {

    pub fn new(
        frames: &[F; NB_FRAMES as usize],
        init_frame: FrameId,
        animation_total_time: u64
        ) -> TimeBasedAnimator<F>
    {
        let frame_animator = FrameAnimator::new(frames, init_frame);
        TimeBasedAnimator {
            frame_animator: frame_animator,
            animation_total_time: animation_total_time,
            accumulated_time: 0,
        }
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
