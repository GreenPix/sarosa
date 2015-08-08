use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::mem;
use std::ops::Deref;
use cgmath::Vector2;

use events::EventSystem;
use game::GameInstance;
use models::player::PlayerId;
use models::player::Player;
use events::UserEvent;
use Settings;

#[cfg(feature = "fake_server")]
use self::fake::RemoteServer;
#[cfg(not(feature = "fake_server"))]
use self::real::RemoteServer;

#[cfg(not(feature = "fake_server"))]
mod real;
#[cfg(feature = "fake_server")]
mod fake;


pub struct Server {
    tx: Sender<UserEvent>,
    rx: Receiver<ServerEvent>,
    tx_error: Sender<()>,
    rx_error: Receiver<()>,
    remote_server: Option<RemoteServer>,
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
        let (tx_error, _) = channel();
        let (_, rx_error) = channel();
        Server {
            tx: tx,
            rx: rx,
            rx_error: rx_error,
            tx_error: tx_error,
            remote_server: None,
            settings: settings,
        }
    }

    pub fn disconnect(&mut self) {
        self.tx_error.send(());
    }

    pub fn connect(&mut self) {

        let mut remote_server = RemoteServer::new(self.settings.network().deref());

        // Main channels for communication
        let (tx_user, rx_user): (Sender<UserEvent>, Receiver<UserEvent>) = channel();
        let (tx_serv, rx_serv): (Sender<ServerEvent>, Receiver<ServerEvent>) = channel();

        // Channels for errors
        let (tx_error_reader, rx_error_reader): (Sender<()>, Receiver<()>) = channel();
        let (tx_error_writer, rx_error_writer): (Sender<()>, Receiver<()>) = channel();

        remote_server.start_writer_thread(rx_user, tx_error_writer);
        remote_server.start_reader_thread(tx_serv, rx_error_reader);

        self.remote_server = Some(remote_server);
        self.rx = rx_serv;
        self.rx_error = rx_error_writer;
        self.tx_error = tx_error_reader;
        let _ = mem::replace(&mut self.tx, tx_user);
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

        if let Ok(_) = self.rx_error.try_recv() {
            return Err(ServerError::Disconnected);
        }

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
}

impl Drop for Server {
    fn drop(&mut self) {
        self.disconnect();
    }
}
