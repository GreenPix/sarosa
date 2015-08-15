extern crate clock_ticks;

use std::thread;
use super::GameInstance;
use events::EventSystem;
use Window;
use Server;

pub mod deferred;

pub enum LoopState {
    Continue,
    Break,
}

pub struct GameLoop;

impl GameLoop {

    pub fn new() -> GameLoop {
        GameLoop
    }

    pub fn run_loop(&mut self, window: &mut Window, instance: &mut GameInstance, server: &mut Server)
    {
        debug!("Game has started");

        let mut lag = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();
        let mut event_sys = EventSystem::default();

        loop {
            // Lookup all events:
            window.poll_events(&mut event_sys);

            // Push then to the server
            server.event_update(&event_sys);

            // Then the game instance
            match instance.event_update(&event_sys) {
                LoopState::Break => break,
                LoopState::Continue => ()
            };

            // Mark all events as consumed
            event_sys.clear();

            // Remote update:
            match server.remote_update(instance) {
                Err(_) => break,
                _ => ()
            }

            // Frame update
            instance.frame_update(window);

            // Fixed update
            let now = clock_ticks::precise_time_ns();
            lag += now - previous_clock;
            previous_clock = now;

            const FIXED_TIME_STAMP: u64 = 16666667;
            // while lag >= FIXED_TIME_STAMP {
            //    lag -= FIXED_TIME_STAMP;
            // }
            let fixed_update = (lag / FIXED_TIME_STAMP) * FIXED_TIME_STAMP;
            instance.fixed_update(lag);
            lag -= fixed_update;

            thread::sleep_ms(((FIXED_TIME_STAMP - lag) / 1000000) as u32);
        }

        debug!("Game has finished");
    }
}
