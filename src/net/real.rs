use std::mem;
use cgmath::Vector2;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use num::traits::ToPrimitive;

use models::player::THIS_PLAYER;
use events::{
    UserEventType,
    UserEvent,
};
use sarosa_net::net::{
    connect,
    NetworkWriter,
    NetworkReader,
    NetworkSettings,
};
use sarosa_net::messages::Notification::*;
use sarosa_net::messages::Order;
use sarosa_net::messages::TargettedOrder;
use sarosa_net::messages::Direction;
use super::ServerEvent;

pub struct RemoteServer {
    writer: Option<NetworkWriter>,
    reader: Option<NetworkReader>,
    this_player_id: Arc<AtomicUsize>,
}

impl RemoteServer {

    pub fn new(settings: &NetworkSettings) -> RemoteServer {
        let (reader, writer) = connect(settings).unwrap();
        RemoteServer {
            writer: Some(writer),
            reader: Some(reader),
            this_player_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn start_writer_thread(&mut self, rx_user: Receiver<UserEvent>, tx_error_writer: Sender<()>) {

        let player_id = self.this_player_id.clone();
        if let Some(mut writer) = mem::replace(&mut self.writer, None) {
            thread::spawn(move|| {

                'run: loop {
                    // Receive user events:
                    let id = player_id.load(Ordering::Relaxed);

                    // TODO:

                    while let Ok(ue) = rx_user.try_recv() {
                        let direction = match ue.kind {
                            UserEventType::Quit => break 'run,
                            UserEventType::CmdUp => Direction::North,
                            UserEventType::CmdDown => Direction::South,
                            UserEventType::CmdLeft => Direction::West,
                            UserEventType::CmdRight => Direction::East,
                        };
                        match writer.write(&TargettedOrder {
                            target: id as u64,
                            order: Order::Walk(Some(direction)),
                        }) {
                            Err(e) => {
                                debug!("io::Error {}", e);
                                // Tell main thread we're going to shutdown.
                                break 'run;
                            },
                            _ => (),
                        }
                    }

                    // Make sure the data is sent.
                    match writer.flush() {
                        Err(e) => {
                            debug!("io::Error {}", e);
                            // Tell main thread we're going to shutdown.
                            let _ = tx_error_writer.send(());
                            break 'run;
                        },
                        _ => (),
                    }

                    thread::sleep_ms(30u32);
                }
            });
        }
    }

    pub fn start_reader_thread(&mut self, tx_serv: Sender<ServerEvent>, rx_error_reader: Receiver<()>) {

        let player_id = self.this_player_id.clone();
        if let Some(mut reader) = mem::replace(&mut self.reader, None) {
            thread::spawn(move|| {

                let mut this_player_id = None;

                'run: loop {

                    if let Ok(_) = rx_error_reader.try_recv() {
                        break 'run;
                    }

                    // Lookup for remote events
                    match reader.read() {
                        Ok(server_event) => {
                            // TODO move that logic somewhere else
                            match server_event {
                                ThisIsYou(id) => {
                                    if this_player_id.is_none() {
                                        this_player_id = Some(id);
                                        player_id.store(10, Ordering::Relaxed);
                                    }
                                }
                                Location {x, y, id} => {
                                    let xf = x.to_f32().unwrap_or(0f32) / 1000f32;
                                    let yf = y.to_f32().unwrap_or(0f32) / 1000f32;
                                    if let &Some(me) = &this_player_id {
                                        let server_event = if me == id {
                                            ServerEvent::Position(Vector2::new(xf, yf), THIS_PLAYER)
                                        } else {
                                            ServerEvent::Position(Vector2::new(xf, yf), id)
                                        };
                                        match tx_serv.send(server_event) {
                                            Err(_) => break 'run,
                                            _ => (),
                                        }
                                    }
                                },
                                _ => (),
                            }
                        }
                        Err(_) => {
                            break 'run;
                        }
                    }

                    thread::sleep_ms(10u32);
                }
            });
        }
    }
}
