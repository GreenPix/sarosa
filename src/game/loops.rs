extern crate clock_ticks;

use std::thread;
use super::GameInstance;
use super::GameRunState;
use events::EventSystem;
use Window;
use Server;

pub struct GameLoop;

impl GameLoop {

    pub fn new() -> GameLoop {
        GameLoop
    }

    pub fn run_loop(&mut self, window: &mut Window, instance: &mut GameInstance, server: &mut Server)
    {
        let mut accumulator = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();
        let mut event_sys = EventSystem::default();

        loop {
            window.poll_events(&mut event_sys);

            // Lookup all events:

            // First the server
            server.event_update(&event_sys);

            // Then the game instance
            match instance.event_update(&event_sys) {
                GameRunState::Stopped => break,
                GameRunState::Running => ()
            };

            // Mark all events as consumed
            event_sys.clear();

            instance.frame_update(window);

            let now = clock_ticks::precise_time_ns();
            accumulator += now - previous_clock;
            previous_clock = now;

            const FIXED_TIME_STAMP: u64 = 16666667;
            while accumulator >= FIXED_TIME_STAMP {
                accumulator -= FIXED_TIME_STAMP;

                instance.fixed_update(FIXED_TIME_STAMP);
            }

            thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
        }
    }
}
