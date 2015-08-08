use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::mem;
use cgmath::Vector2;

use events::EventSystem;
use game::GameInstance;
use models::player::PlayerId;
use models::player::Player;
use events::UserEvent;
use events::UserEventState::*;
use events::UserEventType;
use Settings;

#[cfg(feature = "fake_server")]
use self::fake::RemoteServer;
#[cfg(not(feature = "fake_server"))]
use self::real::RemoteServer;

#[cfg(feature = "fake_server")]
mod fake;
#[cfg(not(feature = "fake_server"))]
mod real;

pub struct Server {
    thread_handle: Option<JoinHandle<()>>,
    tx: Sender<UserEvent>,
    rx: Receiver<ServerEvent>,
    settings: Settings,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ServerError {
    Disconnected,
    GameIsOver,
}

enum ServerEvent {
    DisconnectedFromServer,
    NewPlayer(Vector2<f32>, PlayerId),
    PlayerHasQuit(PlayerId),
    Position(Vector2<f32>, PlayerId),
}

impl Server {

    pub fn new(settings: Settings) -> Server {
        let (tx, _) = channel();
        let (_, rx) = channel();
        Server {
            thread_handle: None,
            tx: tx,
            rx: rx,
            settings: settings,
        }
    }

    pub fn disconnect(&mut self) {
        let join_handle = mem::replace(&mut self.thread_handle, None);
        Server::disconnect_handle(join_handle, &mut self.tx);
    }

    pub fn connect(&mut self, remote: String) {
        let (tx_user, rx_user): (Sender<UserEvent>, Receiver<UserEvent>) = channel();
        let (tx_serv, rx_serv): (Sender<ServerEvent>, Receiver<ServerEvent>) = channel();
        let thread_handle = thread::spawn(move|| {
            let mut server = RemoteServer::new(remote);

            'run: loop {
                // Receive user events:
                while let Ok(ue) = rx_user.try_recv() {
                    match ue.kind {
                        UserEventType::Quit => {
                            server.disconnect();
                            break 'run;
                        }
                        _ => server.event_update(ue),
                    }
                }

                // Lookup for remote events
                for server_event in server.event_iter() {
                    tx_serv.send(server_event).unwrap();
                }

                thread::sleep_ms(30u32);
            }
        });

        self.rx = rx_serv;

        let old_handle = mem::replace(&mut self.thread_handle, Some(thread_handle));
        let mut old_tx = mem::replace(&mut self.tx, tx_user);
        Server::disconnect_handle(old_handle, &mut old_tx);
    }

    pub fn event_update(&mut self, event_sys: &EventSystem) {
        for &e in event_sys.iter() {
            let _ = self.tx.send(e);
        }
    }

    pub fn remote_update(&mut self, game_instance: &mut GameInstance) -> Result<(), ServerError> {
        use self::ServerEvent::NewPlayer;
        use self::ServerEvent::Position;
        use self::ServerEvent::DisconnectedFromServer;

        let mut game_data = game_instance.game_data();

        while let Ok(server_event) = self.rx.try_recv() {
            match server_event {
                NewPlayer(pos, id) => game_data.add_player(Player { position: pos }, id),
                Position(pos, id)  => game_data.add_player(Player { position: pos }, id),
                DisconnectedFromServer => return Err(ServerError::Disconnected),
                _ => (),
            }
        }

        Ok(())
    }

    fn disconnect_handle(join_handle: Option<JoinHandle<()>>, tx: &mut Sender<UserEvent>) {
        if let Some(jh) = join_handle {
            // Send first a message to the thread to terminate
            tx.send(UserEvent { state: Start, kind: UserEventType::Quit}).unwrap();
            // Wait for the other thread termination
            jh.join().unwrap();
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.disconnect();
    }
}
