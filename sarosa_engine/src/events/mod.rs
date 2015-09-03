use std::ops::Deref;
use std::ops::DerefMut;
use std::slice::Iter;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
//use glutin::

#[derive(Default)]
pub struct EventSystem {
    app_events: Vec<AppEvent>,
    command_events: Vec<CommandEvent>,
    seen_cmdevents: HashMap<CommandKind, CommandEvent>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppEvent {
    WinResized { width: u32, height: u32 },
    ZoomIn,
    ZoomOut,
    Quit,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CommandState {
    Start,
    Stop
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CommandEvent {
    pub state: CommandState,
    pub kind: CommandKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CommandKind {
    Up,
    Down,
    Left,
    Right,
}

impl EventSystem {

    pub fn iter_cmds<'a>(&'a self) -> Iter<'a, CommandEvent> {
        self.command_events.iter()
    }

    pub fn iter_apps<'a>(&'a self) -> Iter<'a, AppEvent> {
        self.app_events.iter()
    }

    pub fn clear(&mut self) {
        self.clear_cmds();
        self.clear_apps();
    }

    pub fn clear_cmds(&mut self) {
        self.command_events.clear();
    }

    pub fn clear_apps(&mut self) {
        self.app_events.clear();
    }
}


pub trait PushEvent {
    fn push_cmd(&mut self, e: CommandEvent);
    fn push_app(&mut self, e: AppEvent);
}

impl PushEvent for EventSystem {

    fn push_cmd(&mut self, e: CommandEvent) {
        match self.seen_events.entry(e.kind) {
            Occupied(mut old_e) => {
                if old_e.get().state != e.state {
                    self.command_events.push(e);
                    old_e.insert(e);
                }
            }
            Vacant(free) => {
                free.insert(e);
                self.command_events.push(e);
            }
        }
    }

    fn push_app(&mut self, e: AppEvent) {
        for app_event in self.app_events.iter_mut() {
            match &e {
                &AppEvent::WinResized { width, height } => {
                    if let AppEvent::WinResized { width: ref mut w, height: ref mut h } = app_event {
                        *w = width;
                        *h = height;
                        return;
                    }
                },
                &AppEvent::Quit => {
                    if let AppEvent::Quit = app_event {
                        return;
                    }
                }
            }
        }
        self.app_events.push(e);
    }
}
