extern crate clock_ticks;

use std::thread;

use events::EventSystem;
use profiler::Profiler;
use super::GameInstance;
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

        let mut p = Profiler::new();
        let mut lag = 0;
        let mut previous_clock = clock_ticks::precise_time_ns();
        let mut event_sys = EventSystem::default();

        loop {
            p.start_frame();

            // Lookup all events:
            p.enter("Poll events");
            window.poll_events(&mut event_sys);
            p.leave();

            // Push them to the server
            p.enter("Push event to server");
            server.event_update(&event_sys);
            p.leave();

            // Then the game instance
            p.enter("Local event update");
            match instance.event_update(&event_sys) {
                LoopState::Break => break,
                LoopState::Continue => ()
            };
            p.leave();

            // Mark all events as consumed
            event_sys.clear();

            // Remote update:
            p.enter("Server event update");
            match server.remote_update(instance) {
                Err(_) => break,
                _ => ()
            }
            p.leave();

            // Frame update
            p.enter("Frame update");
            instance.frame_update(window);
            p.leave();

            // Fixed update
            p.enter("Fixed update");
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
            p.leave();


            thread::sleep_ms(((FIXED_TIME_STAMP - lag) / 1000000) as u32);
            p.end_frame();
        }

        p.print_summary();

        debug!("Game has finished");
    }
}
