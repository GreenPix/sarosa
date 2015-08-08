use std::rc::Rc;
use std::collections::HashMap;
use std::cell::{
    RefCell,
    Ref,
    RefMut
};
use glium::glutin::{
    VirtualKeyCode
};
use events::UserEventType;

#[derive(Debug, Clone)]
pub struct Settings {
    keyboard: Rc<RefCell<KeyboardSettings>>,
    window: Rc<RefCell<WindowSettings>>,
}

#[derive(Debug)]
pub struct KeyboardSettings(HashMap<VirtualKeyCode, UserEventType>);

#[derive(Debug)]
pub struct WindowSettings {
    width: u32,
    height: u32,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            keyboard: Rc::new(RefCell::new(KeyboardSettings::new())),
            window: Rc::new(RefCell::new(WindowSettings::new())),
        }
    }

    pub fn keyboard<'a>(&'a self) -> Ref<'a, KeyboardSettings> {
        self.keyboard.borrow()
    }

    pub fn window<'a>(&'a self) -> Ref<'a, WindowSettings> {
        self.window.borrow()
    }

    pub fn keyboard_mut<'a>(&'a mut self) -> RefMut<'a, KeyboardSettings> {
        self.keyboard.borrow_mut()
    }

    pub fn window_mut<'a>(&'a mut self) -> RefMut<'a, WindowSettings> {
        self.window.borrow_mut()
    }

    pub fn all_mut<'a>(&'a mut self) -> (RefMut<'a, WindowSettings>, RefMut<'a, KeyboardSettings>) {
        (self.window.borrow_mut(), self.keyboard.borrow_mut())
    }
}

impl KeyboardSettings {

    pub fn new() -> KeyboardSettings {
        let mut hm = HashMap::new();
        hm.insert(VirtualKeyCode::Up, UserEventType::CmdUp);
        hm.insert(VirtualKeyCode::Down, UserEventType::CmdDown);
        hm.insert(VirtualKeyCode::Left, UserEventType::CmdLeft);
        hm.insert(VirtualKeyCode::Right, UserEventType::CmdRight);
        KeyboardSettings(hm)
    }

    pub fn get(&self, key: Option<VirtualKeyCode>) -> Option<UserEventType> {
        if let Some(k) = key {
            self.0.get(&k).map(|e| *e)
        } else {
            None
        }
    }
}

impl WindowSettings {

    pub fn new() -> WindowSettings {
        WindowSettings {
            width: 800,
            height: 600,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}
