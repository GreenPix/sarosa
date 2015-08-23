use std::rc::Rc;
use std::ops::Deref;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::io;
use std::cell::{
    RefCell,
    Ref,
    RefMut
};
use glium::glutin::{
    VirtualKeyCode
};
use events::CommandKind;

#[derive(Debug, Clone)]
pub struct Settings {
    keyboard: Rc<RefCell<KeyboardSettings>>,
    window: Rc<RefCell<WindowSettings>>,
    network: Rc<RefCell<NetworkSettings>>,
}

#[derive(Debug)]
pub struct KeyboardSettings(HashMap<VirtualKeyCode, CommandKind>);

#[derive(Debug)]
pub struct WindowSettings {
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub struct NetworkSettings {
    address: String,
    offline_server: bool,
}

impl Settings {
    pub fn new(addr: String, offline_server: bool) -> Settings {
        Settings {
            keyboard: Rc::new(RefCell::new(KeyboardSettings::new())),
            window: Rc::new(RefCell::new(WindowSettings::new())),
            network: Rc::new(RefCell::new(NetworkSettings::new(addr, offline_server))),
        }
    }

    pub fn keyboard<'a>(&'a self) -> Ref<'a, KeyboardSettings> {
        self.keyboard.borrow()
    }

    pub fn network<'a>(&'a self) -> Ref<'a, NetworkSettings> {
        self.network.borrow()
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

impl NetworkSettings {

    pub fn new(addr: String, offline_server: bool) -> NetworkSettings {
        NetworkSettings {
            address: addr,
            offline_server: offline_server,
        }
    }

    pub fn offline_server(&self) -> bool {
        self.offline_server
    }

    pub fn addr(&self) -> &str {
        self.address.deref()
    }
}

impl ToSocketAddrs for NetworkSettings {
    type Iter = <&'static str as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.address.to_socket_addrs()
    }
}

impl KeyboardSettings {

    pub fn new() -> KeyboardSettings {
        let mut hm = HashMap::new();

        ///////////////////////////////////////////////////////////////
        // Default bindings for Keyboards Events
        //
        // TODO(Nemikolh): Read that config from a settings file.
        //
        hm.insert(VirtualKeyCode::Up, CommandKind::Up);
        hm.insert(VirtualKeyCode::Down, CommandKind::Down);
        hm.insert(VirtualKeyCode::Left, CommandKind::Left);
        hm.insert(VirtualKeyCode::Right, CommandKind::Right);
        hm.insert(VirtualKeyCode::I, CommandKind::ZoomIn);
        hm.insert(VirtualKeyCode::O, CommandKind::ZoomOut);
        //
        ///////////////////////////////////////////////////////////////
        KeyboardSettings(hm)
    }

    pub fn get(&self, key: Option<VirtualKeyCode>) -> Option<CommandKind> {
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
