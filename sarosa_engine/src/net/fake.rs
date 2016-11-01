extern crate rand;

use std::time::Duration;
use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::iter;
use std::iter::Once;
use std::sync::Mutex;

use cgmath::{Zero, ElementWise};
use cgmath::Vector2;

use animation::TextureId;
use models::player::THIS_PLAYER;
use events::{
    UserEventType,
    UserEvent,
};
use events::UserEventType::*;
use events::UserEventState::*;

use super::ServerEvent;
use super::RemoteServerHandle;

struct FakeServerForReal {
    current_player_pos: Vector2<f32>,
    speed: Vector2<f32>,
    first_event: bool,
}

impl FakeServerForReal {

    fn event_update(&mut self, user_event: UserEvent) {
        let speed = match user_event {
            UserEvent { state: Start, kind: CmdUp }     => Vector2::new(0f32,  1f32),
            UserEvent { state: Start, kind: CmdDown }   => Vector2::new(0f32, -1f32),
            UserEvent { state: Start, kind: CmdLeft }   => Vector2::new(-1f32, 0f32),
            UserEvent { state: Start, kind: CmdRight }  => Vector2::new( 1f32, 0f32),
            _ => Vector2::zero(),
        };
        let factor = Vector2::new(0.5f32, 0.5f32);
        self.speed = &self.speed + &factor.mul_element_wise(speed);
    }

    fn event_iter(&mut self) -> Once<ServerEvent> {
        let approx_dt = Vector2::new(0.02, 0.02);
        self.current_player_pos = &self.current_player_pos + &approx_dt.mul_element_wise(self.speed);

        if self.first_event {
            self.first_event = false;
            iter::once(ServerEvent::NewPlayer {
                initial_pos: self.current_player_pos,
                id: THIS_PLAYER,
                tex_id: TextureId(rand::random::<u32>() % 3),
            })
        } else {
            iter::once(ServerEvent::Position {
                pos: self.current_player_pos,
                id: THIS_PLAYER,
                speed: self.speed,
            })
        }
    }
}

pub struct RemoteServer {
    data: Arc<Mutex<FakeServerForReal>>,
}

impl RemoteServerHandle for RemoteServer {}

impl RemoteServer {

    pub fn new() -> RemoteServer {
        RemoteServer {
            data: Arc::new(Mutex::new(
                FakeServerForReal {
                    current_player_pos: Vector2::zero(),
                    speed: Vector2::zero(),
                    first_event: true,
                }
            ))
        }
    }

    pub fn start_writer_thread(&mut self, rx_user: Receiver<UserEvent>, _: Sender<()>) {

        let arc_mutex_crazy_frog = self.data.clone();
        thread::Builder::new()
            .name("NetworkFake - Writer".to_string())
            .spawn(move|| {

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

                thread::sleep(Duration::from_millis(30));
            }
        }).expect("Couldn't start thread");
    }

    pub fn start_reader_thread(&mut self, tx_serv: Sender<ServerEvent>, _: Receiver<()>) {

        let arc_mutex_crazy_frog = self.data.clone();
        thread::Builder::new()
            .name("NetworkFake - Reader".to_string())
            .spawn(move|| {

            'run: loop {
                // Lookup for remote events
                {
                    let mut server = arc_mutex_crazy_frog.lock().unwrap();
                    for server_event in server.event_iter() {
                        match tx_serv.send(server_event) {
                            Err(_) => break 'run,
                            _ => (),
                        }
                    }
                }

                thread::sleep(Duration::from_millis(20));
            }
        }).expect("Couldn't start thread");
    }
}
