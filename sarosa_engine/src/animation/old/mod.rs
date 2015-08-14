
pub type FrameId = u8;

//u8 = 0b1000_0000;
const BACKWARD_DIR: FrameId = 0b1000_0000;
const NB_FRAMES: FrameId = 3;

pub struct FrameAnimator<F> {
    frames: [F; NB_FRAMES as usize],
    last_frame: FrameId,
    init_frame: FrameId,
    end_frame: FrameId,
}

impl<F: Copy> FrameAnimator<F> {

    #[inline]
    pub fn new(frames: &[F; NB_FRAMES as usize], init_frame: FrameId) -> FrameAnimator<F> {
        assert!(init_frame < NB_FRAMES);
        FrameAnimator {
            frames: *frames,
            last_frame: init_frame,
            init_frame: init_frame,
            end_frame: init_frame,
        }
    }

    #[inline]
    pub fn set_end_frame(&mut self, end_frame: FrameId) {
        assert!(self.end_frame < NB_FRAMES);
        assert!(self.end_frame > self.init_frame + 1 || self.end_frame == self.init_frame);
        self.end_frame = end_frame;
    }

    #[inline]
    pub fn next_frame(&mut self) -> F {
        unsafe {
            let index = self.next_index();
            *self.frames.get_unchecked(index as usize)
        }
    }

    pub fn is_finished(&self) -> bool {
        self.end_frame == self.last_frame + 1
    }

    fn next_index(&mut self) -> FrameId {
        if self.last_frame + 1 == self.end_frame {
            self.end_frame
        } else if self.last_frame & BACKWARD_DIR == BACKWARD_DIR {
            // We decrement the last frame
            self.last_frame = (self.last_frame ^ BACKWARD_DIR).saturating_sub(1);
            if self.last_frame == 0 {
                0
            } else {
                self.last_frame |= BACKWARD_DIR;
                self.last_frame ^ BACKWARD_DIR
            }
        } else {
            self.last_frame = self.last_frame + 1;
            if self.last_frame == NB_FRAMES {
                self.last_frame |= BACKWARD_DIR;
                self.last_frame ^ BACKWARD_DIR
            } else {
                self.last_frame
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    pub type Test = FrameAnimator<u32>;

    #[test]
    #[should_panic]
    fn new_should_reject_greater_value_than_nb_frames() {
        let frame_player = Test::new(&[1, 2, 3], 3);
    }

    #[test]
    #[should_panic]
    fn set_end_frame_should_reject_smaller_value_than_init() {
        let frame_player = Test::new(&[1, 2, 3], 2);
        frame_player.set_end_frame(0);
    }

    #[test]
    #[should_panic]
    fn set_end_frame_should_reject_init_frame_plus_1() {
        let frame_player = Test::new(&[1, 2, 3], 0);
        frame_player.set_end_frame(1);
    }

    #[test]
    fn next_frame_should_go_from_init_to_end() {
        let frame_player = Test::new(&[1, 2, 3], 0);
        frame_player.set_end_frame(2);
        assert_eq!(frame_player.next_frame(), 1);
        assert_eq!(frame_player.next_frame(), 2);
        assert_eq!(frame_player.next_frame(), 3);
        assert_eq!(frame_player.next_frame(), 3);
        assert_eq!(frame_player.is_finished(), 3);
    }

    #[test]
    fn next_frame_should_loop_when_init_eq_end() {
        let frame_player = Test::new(&[1, 2, 3], 1);
        assert_eq!(frame_player.next_frame(), 2);
        assert_eq!(frame_player.next_frame(), 3);
        assert_eq!(frame_player.next_frame(), 2);
        assert_eq!(frame_player.next_frame(), 1);
        assert_eq!(frame_player.next_frame(), 2);
        assert_eq!(frame_player.next_frame(), 2);
        assert_eq!(frame_player.is_finished(), 2);
    }
}
