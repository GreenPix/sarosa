use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use cgmath::Vector2;
use animation::TextureId;
use models::player::THIS_PLAYER;
use sarosa_net::messages::Vec2d;
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
            NewEntity { entity, position: Vec2d { x, y }, skin, pv } => {
                debug!("New  player: {}", entity);
                if let &Some(me) = &self.local_copy_player_id {
                    if me == entity {
                        Some(ServerEvent::NewPlayer {
                            initial_pos: Vector2::new(x, y),
                            id: THIS_PLAYER,
                            tex_id: TextureId((skin % 3) as u32),
                        })
                    } else {
                        Some(ServerEvent::NewPlayer {
                            initial_pos: Vector2::new(x, y),
                            id: entity,
                            tex_id: TextureId((skin % 3) as u32),
                        })
                    }
                } else {
                    None
                }
            }
            EntityHasQuit { entity } => {
                debug!("Player has quit: {}", entity);
                Some(ServerEvent::PlayerHasQuit(entity))
            }
            Say { entity, message } => {
                debug!("Player {} says: {}", entity, message);
                None
            }
            Position { entity, position, speed, pv } => {
                let xf = position.x;
                let yf = position.y;
                //Vector2::new(0f32, 0.001f32);
                let speed = Vector2::new(speed.x, speed.y);

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
