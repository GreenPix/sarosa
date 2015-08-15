use cgmath::{self, Matrix4};
use glium::glutin;
use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{
    Event,
    ElementState,
};
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

pub mod scene;

// Private modules
mod renderer;
mod camera;

/// Window sarosa object.
pub struct Window {
    display: GlutinFacade,
    settings: Settings,
    projection: Matrix4<f32>,
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
            projection: Window::ortho(width, height)
        }
    }

    fn ortho(width: u32, height: u32) -> Matrix4<f32> {
        let w = width as f32;
        let h = height as f32;
        let m = cgmath::ortho(- w / 2.0, w / 2.0, - h / 2.0, h / 2.0, -1.0, 1.0);
        m
    }

    fn projection(&self) -> &Matrix4<f32> {
        &self.projection
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
                Event::Resized(width, height) => {
                    self.projection  = Window::ortho(width, height);
                    None
                }
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
