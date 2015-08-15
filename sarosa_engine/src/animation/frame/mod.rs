
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
    distance_made: f32,
    total_distance: f32,
}


impl<F: Copy + Eq> TimeBasedAnimator<F> {

    pub fn new(
        frames: &[F; NB_FRAMES as usize],
        init_frame: FrameId,
        end_frame: FrameId,
        total_distance: f32,
        ) -> TimeBasedAnimator<F>
    {
        let mut frame_animator = FrameAnimator::new(frames, init_frame);
        frame_animator.set_end_frame(end_frame);
        TimeBasedAnimator {
            frame_animator: frame_animator,
            total_distance: total_distance,
            distance_made: 0.0,
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
    pub fn next_frame(&mut self, time_elapsed: u64, instant_speed: f32) -> F {
        if self.frame_animator.is_finished() {
            self.frame_animator.reset();
        }
        let frame = self.frame_animator.current_frame();

        self.distance_made += (time_elapsed as f64 / 1e9) as f32 * instant_speed;
        let frame_dist = self.total_distance / (NB_FRAMES as f32);

        while self.distance_made > frame_dist {
            self.frame_animator.next_frame();
            self.distance_made -= frame_dist;
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
