use std::iter::Once;
use std::iter;
use events::UserEvent;
use models::player::THIS_PLAYER;
use events::UserEventType::*;
use events::UserEventState::*;
use cgmath::{
    Vector2
};
use num::traits::Zero;
use super::ServerEvent;

pub struct RemoteServer {
    current_player_pos: Vector2<f32>,
    direction: Vector2<f32>,
}

impl RemoteServer {

    pub fn new(_: String) -> RemoteServer {
        RemoteServer {
            current_player_pos: Vector2::zero(),
            direction: Vector2::zero(),
        }
    }

    pub fn disconnect(&self) {}

    pub fn event_update(&mut self, user_event: UserEvent) {
        let direction = match user_event {
            UserEvent { state: Start, kind: CmdUp }     => Vector2::new(0f32,  1f32),
            UserEvent { state: Start, kind: CmdDown }   => Vector2::new(0f32, -1f32),
            UserEvent { state: Start, kind: CmdLeft }   => Vector2::new(-1f32, 0f32),
            UserEvent { state: Start, kind: CmdRight }  => Vector2::new( 1f32, 0f32),
            _ => Vector2::zero(),
        };
        self.direction = self.direction + direction;
    }

    pub fn event_iter(&mut self) -> Once<ServerEvent> {
        let factor = Vector2::new(0.0001f32, 0.0001f32);
        self.current_player_pos = self.current_player_pos + factor * self.direction;
        iter::once(ServerEvent::Position(self.current_player_pos, THIS_PLAYER))
    }
}
