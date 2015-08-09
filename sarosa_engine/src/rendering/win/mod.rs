use cgmath::Vector2;
use glium::glutin;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{
    Event,
    ElementState,
};
use num::traits::Zero;
use events::{
    PushEvent,
    UserEvent,
    UserEventType
};
use events::UserEventState::{
    Start,
    Stop
};
use Settings;

pub use self::renderer::GameRenderer;

mod renderer;


pub struct Camera {
    position: Vector2<f32>,
}


impl Camera {

    pub fn new() -> Camera {
        Camera {
            position: Vector2::zero()
        }
    }

}

pub struct Window {
    display: GlutinFacade,
    settings: Settings,
}

impl Window {

    pub fn new<T: ToString>(settings: Settings, window_title: T) -> Window {
        let display = glutin::WindowBuilder::new()
            .with_visibility(true)
            .with_title(window_title.to_string())
            .with_dimensions(settings.window().width(), settings.window().height())
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
                Event::Closed => Some(UserEvent{
                    state: Start,
                    kind: UserEventType::Quit
                }),
                //KeyboardInput(ElementState, u8, Option<VirtualKeyCode>)
                Event::KeyboardInput(state, _, key) => {
                    let s = match state {
                        ElementState::Pressed => Start,
                        ElementState::Released => Stop,
                    };
                    if let Some(kind) = keyboard.get(key) {
                        Some(UserEvent {
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
