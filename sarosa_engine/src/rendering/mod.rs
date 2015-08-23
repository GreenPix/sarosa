
use glium::glutin;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{
    Event,
    ElementState,
};
use events::{
    PushEvent,
    CommandEvent,
    CommandKind,
    AppEvent,
};
use profiler::Profiler;
use events::CommandState::{
    Start,
    Stop
};
use Settings;

pub use self::renderer::GameRenderer;

pub mod scene;

// Private modules
mod renderer;
mod camera;

/// Window sarosa object.
pub struct Window {
    display: GlutinFacade,
    settings: Settings,
}

impl Window {

    pub fn new<T: ToString>(settings: Settings, window_title: T) -> Window {
        let width = settings.window().width();
        let height = settings.window().height();

        let display = glutin::WindowBuilder::new()
            .with_visibility(true)
            .with_title(window_title.to_string())
            .with_dimensions(width, height)
            .build_glium()
            .unwrap();


        Window {
            display: display,
            settings: settings,
        }
    }

    pub fn set_title(&self, title: &str) {
        if let Some(win) = self.display.get_window() {
            win.set_title(title);
        }
    }

    pub fn poll_events(&mut self, event_sys: &mut PushEvent) {
        let keyboard = self.settings.keyboard();
        for event in self.display.poll_events() {
            let e = match event {
                Event::Closed => Some(CommandEvent{
                    state: Start,
                    kind: CommandKind::Quit
                }),
                Event::Resized(width, height) => {
                    event_sys.push_app(AppEvent::WinResized { width: width, height: height });
                    None
                }
                //KeyboardInput(ElementState, u8, Option<VirtualKeyCode>)
                Event::KeyboardInput(state, _, key) => {
                    let s = match state {
                        ElementState::Pressed => Start,
                        ElementState::Released => Stop,
                    };
                    if let Some(kind) = keyboard.get(key) {
                        Some(CommandEvent {
                            state: s,
                            kind: kind,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(e) = e {
                event_sys.push(e);
            }
        }
    }
}
