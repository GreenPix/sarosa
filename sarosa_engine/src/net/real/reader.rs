use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use num::traits::ToPrimitive;

use cgmath::Vector2;
use models::player::THIS_PLAYER;
use sarosa_net::messages::Location as Loc;
use sarosa_net::messages::Notification::*;
use sarosa_net::messages::Notification;
use net::ServerEvent;

pub struct ServerEventReader {
    this_player_id: Arc<AtomicUsize>,
    local_copy_player_id: Option<u64>,
}

impl ServerEventReader  {

    pub fn new(player_id: Arc<AtomicUsize>) -> ServerEventReader {

        ServerEventReader {
            this_player_id: player_id,
            local_copy_player_id: None
        }
    }

    pub fn consume_event(&mut self, server_event: Notification) -> Option<ServerEvent> {
        match server_event {
            ThisIsYou { entity: id } => {
                if self.local_copy_player_id.is_none() {
                    debug!("ThisIsYou({}) received", id);
                    self.local_copy_player_id = Some(id);
                    self.this_player_id.store(id as usize, Ordering::Relaxed);
                }
                None
            }
            Location { entity, location: Loc { x, y } } => {
                let xf = x.to_f32().unwrap_or(0f32) / 1000f32;
                let yf = y.to_f32().unwrap_or(0f32) / 1000f32;
                let speed = Vector2::new(0f32, 1f32);

                if let &Some(me) = &self.local_copy_player_id {
                    if me == entity {
                        Some(ServerEvent::Position {
                            pos: Vector2::new(xf, yf),
                            speed: speed,
                            id: THIS_PLAYER,
                        })
                    } else {
                        Some(ServerEvent::Position {
                            pos: Vector2::new(xf, yf),
                            speed: speed,
                            id: entity,
                        })
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}
