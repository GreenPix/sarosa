use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
//use glutin::

#[derive(Default)]
pub struct EventSystem {
    app_events: Vec<AppEvent>,
    command_events: Vec<CommandEvent>,
    seen_events: HashMap<CommandKind, CommandEvent>,
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
    pub fn iter_cmds(&self) -> {
        self.queue.filter_map()
    }

    pub fn clear(&mut self) {
        self.clear_cmds();
        self.clear_();
    }

    pub fn clear_cmds(&mut self) {
        self.command_events.clear();
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
}
