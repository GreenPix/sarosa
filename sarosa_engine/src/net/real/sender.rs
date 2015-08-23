use std::cmp::max;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use sarosa_net::messages::Direction;
use events::{
    CommandKind,
    CommandState,
    CommandEvent,
};
use sarosa_net::messages::Order;

struct CommandStates {
    up: u8,
    down: u8,
    left: u8,
    right: u8,
}

impl CommandStates {
    pub fn new() -> CommandStates {
        CommandStates {
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        }
    }

    pub fn inject(&mut self, command: CommandKind, reset: bool) {
        let max_value = max(max(max(self.up, self.down), self.left), self.right);
        if reset {
            let old_value = match command {
                CommandKind::Up => {
                    let old_value = self.up;
                    self.up = 0;
                    old_value
                }
                CommandKind::Down => {
                    let old_value = self.down;
                    self.down = 0;
                    old_value
                }
                CommandKind::Left => {
                    let old_value = self.left;
                    self.left = 0;
                    old_value
                }
                CommandKind::Right => {
                    let old_value = self.right;
                    self.right = 0;
                    old_value
                }
            };

            if self.up    > old_value { self.up    = self.up    - 1 }
            if self.down  > old_value { self.down  = self.down  - 1 }
            if self.left  > old_value { self.left  = self.left  - 1 }
            if self.right > old_value { self.right = self.right - 1 }
        } else {
            match command {
                CommandKind::Up => self.up = max_value + 1,
                CommandKind::Down => self.down = max_value + 1,
                CommandKind::Left => self.left = max_value + 1,
                CommandKind::Right => self.right = max_value + 1,
            }
        }
    }

    pub fn next_direction(&self) -> Option<Direction> {
        let max_value = max(max(max(self.up, self.down), self.left), self.right);
        if max_value == 0 { None }
        else if self.up == max_value { Some(Direction::North) }
        else if self.down == max_value { Some(Direction::South) }
        else if self.left == max_value { Some(Direction::West) }
        else { Some(Direction::East) }
    }
}

pub struct CommandEventSender {
    commands_states: CommandStates,
    this_player_id: Arc<AtomicUsize>,
}

impl CommandEventSender {

    pub fn new(player_id: Arc<AtomicUsize>) -> CommandEventSender {
        CommandEventSender {
            this_player_id: player_id,
            commands_states: CommandStates::new(),
        }
    }

    pub fn prepare_event_consumer(&mut self) -> CommandEventConsumer {
        let id = self.this_player_id.load(Ordering::Relaxed);
        CommandEventConsumer {
            s: self,
            player_id: id,
        }
    }
}

pub struct CommandEventConsumer<'a> {
    s: &'a mut CommandEventSender,
    pub player_id: usize,
}

impl<'a> CommandEventConsumer<'a> {

    pub fn consume_event(&mut self, ue: CommandEvent) -> Order {
        let reset = ue.state != CommandState::Start;
        self.s.commands_states.inject(ue.kind, reset);
        Order::Walk(self.s.commands_states.next_direction())
    }
}
