use std::ops::Deref;
use std::ops::DerefMut;
//use glutin::

#[derive(Default)]
pub struct EventSystem {
    queue: Vec<UserEvent>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserEventState {
    Start,
    Stop
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UserEvent {
    pub state: UserEventState,
    pub kind: UserEventType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserEventType {
    Quit,
    CmdUp,
    CmdDown,
    CmdLeft,
    CmdRight,
}


pub trait PushEvent {
    fn push(&mut self, e: UserEvent);
}

impl PushEvent for EventSystem {
    fn push(&mut self, e: UserEvent) {
        self.queue.push(e);
    }
}

impl Deref for EventSystem {
    type Target = Vec<UserEvent>;

    fn deref(&self) -> &Vec<UserEvent> {
        &self.queue
    }
}

impl DerefMut for EventSystem {
    fn deref_mut(&mut self) -> &mut Vec<UserEvent> {
        &mut self.queue
    }
}
