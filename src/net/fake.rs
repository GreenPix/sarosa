use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::iter;
use std::iter::Once;
use std::sync::Mutex;

use num::traits::Zero;
use cgmath::Vector2;
use models::player::THIS_PLAYER;
use events::{
    UserEventType,
    UserEvent,
};
use events::UserEventType::*;
use events::UserEventState::*;
use sarosa_net::net::{
    NetworkSettings,
};
use super::ServerEvent;

struct FakeServerForReal {
    current_player_pos: Vector2<f32>,
    direction: Vector2<f32>,
}

impl FakeServerForReal {

    fn event_update(&mut self, user_event: UserEvent) {
        let direction = match user_event {
            UserEvent { state: Start, kind: CmdUp }     => Vector2::new(0f32,  1f32),
            UserEvent { state: Start, kind: CmdDown }   => Vector2::new(0f32, -1f32),
            UserEvent { state: Start, kind: CmdLeft }   => Vector2::new(-1f32, 0f32),
            UserEvent { state: Start, kind: CmdRight }  => Vector2::new( 1f32, 0f32),
            _ => Vector2::zero(),
        };
        self.direction = self.direction + direction;
    }

    fn event_iter(&mut self) -> Once<ServerEvent> {
        let factor = Vector2::new(0.01f32, 0.01f32);
        self.current_player_pos = self.current_player_pos + factor * self.direction;
        iter::once(ServerEvent::Position(self.current_player_pos, THIS_PLAYER))
    }
}

pub struct RemoteServer {
    data: Arc<Mutex<FakeServerForReal>>,
}

impl RemoteServer {

    pub fn new(_: &NetworkSettings) -> RemoteServer {
        RemoteServer {
            data: Arc::new(Mutex::new(
                FakeServerForReal {
                    current_player_pos: Vector2::zero(),
                    direction: Vector2::zero(),
                }
            ))
        }
    }

    pub fn start_writer_thread(&mut self, rx_user: Receiver<UserEvent>, _: Sender<()>) {

        let arc_mutex_crazy_frog = self.data.clone();
        thread::spawn(move|| {

            'run: loop {

                // Receive user events:
                {
                    let mut server = arc_mutex_crazy_frog.lock().unwrap();
                    while let Ok(ue) = rx_user.try_recv() {
                        match ue.kind {
                            UserEventType::Quit => {
                                break 'run;
                            }
                            _ => {
                                server.event_update(ue);
                            },
                        }
                    }
                }

                thread::sleep_ms(30u32);
            }
        });
    }

    pub fn start_reader_thread(&mut self, tx_serv: Sender<ServerEvent>, _: Receiver<()>) {

        let arc_mutex_crazy_frog = self.data.clone();
        thread::spawn(move|| {

            'run: loop {
                // Lookup for remote events
                {
                    let mut server = arc_mutex_crazy_frog.lock().unwrap();
                    for server_event in server.event_iter() {
                        tx_serv.send(server_event).unwrap();
                    }
                }

                thread::sleep_ms(20u32);
            }
        });
    }
}


//
//
// pub fn disconnect(&mut self) {
//     let join_handle = mem::replace(&mut self.thread_handle, None);
//     Server::disconnect_handle(join_handle, &mut self.tx);
// }
//
//
// fn disconnect_handle(join_handle: Option<JoinHandle<()>>, tx: &mut Sender<UserEvent>) {
//     if let Some(jh) = join_handle {
//         // Send first a message to the thread to terminate
//         tx.send(UserEvent { state: Start, kind: UserEventType::Quit}).unwrap();
//         // Wait for the other thread termination
//         jh.join().unwrap();
//     }
// }
//
//
//
// let (tx_user, rx_user): (Sender<UserEvent>, Receiver<UserEvent>) = channel();
// let (tx_serv, rx_serv): (Sender<ServerEvent>, Receiver<ServerEvent>) = channel();
//
// let thread_writer = thread::spawn(move|| {
//     let mut server = RemoteServer::new(remote);
//
//     'run: loop {
//         // Receive user events:
//         while let Ok(ue) = rx_user.try_recv() {
//             match ue.kind {
//                 UserEventType::Quit => {
//                     server.disconnect();
//                     break 'run;
//                 }
//                 _ => server.event_update(ue),
//             }
//         }
//
//         // Lookup for remote events
//         for server_event in server.event_iter() {
//             tx_serv.send(server_event).unwrap();
//         }
//
//         thread::sleep_ms(30u32);
//     }
// });
//
// self.rx = rx_serv;
// let mut old_tx = mem::replace(&mut self.tx, tx_user);
//
// let old_handle = mem::replace(&mut self.thread_handle, Some(thread_handle));
// Server::disconnect_handle(old_handle, &mut old_tx);
//
//
//
//
//
//
//
// impl RemoteServer {
//
//     pub fn new(_: String) -> RemoteServer {
//         RemoteServer {
//             current_player_pos: Vector2::zero(),
//             direction: Vector2::zero(),
//         }
//     }
//
//     pub fn disconnect(&self) {}
//
//     pub fn event_update(&mut self, user_event: UserEvent) {
//         let direction = match user_event {
//             UserEvent { state: Start, kind: CmdUp }     => Vector2::new(0f32,  1f32),
//             UserEvent { state: Start, kind: CmdDown }   => Vector2::new(0f32, -1f32),
//             UserEvent { state: Start, kind: CmdLeft }   => Vector2::new(-1f32, 0f32),
//             UserEvent { state: Start, kind: CmdRight }  => Vector2::new( 1f32, 0f32),
//             _ => Vector2::zero(),
//         };
//         self.direction = self.direction + direction;
//     }
//
//     pub fn event_iter(&mut self) -> Once<ServerEvent> {
//         let factor = Vector2::new(0.01f32, 0.01f32);
//         self.current_player_pos = self.current_player_pos + factor * self.direction;
//         iter::once(ServerEvent::Position(self.current_player_pos, THIS_PLAYER))
//     }
// }
