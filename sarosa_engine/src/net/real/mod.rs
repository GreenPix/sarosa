use std::mem;
use std::sync::atomic::AtomicUsize;
use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use sarosa_net::net::{
    connect,
    NetworkWriter,
    NetworkReader,
    NetworkSettings,
    NetworkError,
};
use sarosa_net::messages::Order;
use events::{
    UserEvent,
    UserEventType
};
use models::settings;
use sarosa_net::messages::EntityOrder;
use net::{
    RemoteServerHandle,
    ServerEvent
};

use self::reader::ServerEventReader;
use self::sender::UserEventSender;
mod reader;
mod sender;

pub struct RemoteServer {
    writer: Option<NetworkWriter>,
    reader: Option<NetworkReader>,
    this_player_id: Arc<AtomicUsize>,
}

impl RemoteServerHandle for RemoteServer {}

impl RemoteServer {

    pub fn new(settings: &settings::NetworkSettings) -> RemoteServer {
        let sets = NetworkSettings::new(settings).unwrap();
        let (reader, writer) = connect(&sets).unwrap();
        RemoteServer {
            writer: Some(writer),
            reader: Some(reader),
            this_player_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn start_writer_thread(&mut self, rx_user: Receiver<UserEvent>, tx_error_writer: Sender<()>) {

        let player_id = self.this_player_id.clone();
        if let Some(mut writer) = mem::replace(&mut self.writer, None) {
            thread::Builder::new()
                .name("Network - Writer".to_string())
                .spawn(move|| {

                let mut sender = UserEventSender::new(player_id);

                'run: loop {

                    // Receive user events:
                    let mut converter = sender.prepare_event_consumer();

                    while let Ok(ue) = rx_user.try_recv() {
                        if let UserEventType::Quit = ue.kind {
                            break 'run;
                        }
                        let order = if let UserEventType::Attack = ue.kind {
                            Order::Attack
                        } else {
                            converter.consume_event(ue)
                        };

                        let order_event = EntityOrder {
                            entity: converter.player_id as u64,
                            order: order
                        };
                        match writer.write(&order_event) {
                            Err(e) => {
                                debug!("io::Error {}", e);
                                break 'run;
                            },
                            _ => (),
                        }
                    }

                    // Make sure the data is sent.
                    match writer.flush() {
                        Err(e) => {
                            debug!("io::Error {}", e);
                            break 'run;
                        },
                        _ => (),
                    }

                    thread::sleep_ms(30u32);
                }

                // Tell main thread we're going to shutdown.
                let _ = tx_error_writer.send(());

            }).expect("Couldn't start thread");
        }
    }

    pub fn start_reader_thread(&mut self, tx_serv: Sender<ServerEvent>, rx_error_reader: Receiver<()>) {

        let player_id = self.this_player_id.clone();
        if let Some(mut reader) = mem::replace(&mut self.reader, None) {
            thread::Builder::new()
                .name("Network - Reader".to_string())
                .spawn(move|| {

                let mut converter = ServerEventReader::new(player_id);

                'run: loop {

                    if let Ok(_) = rx_error_reader.try_recv() {
                        break 'run;
                    }

                    // Lookup for remote events
                    'events: loop {
                        match reader.read() {
                            Ok(notification) => match converter.consume_event(notification) {
                                Some(server_event) => match tx_serv.send(server_event) {
                                    Err(_) => break 'run,
                                    _ => ()
                                },
                                None => (),
                            },
                            Err(network_error) => match network_error {
                                NetworkError::DisconnectedFromServer => break 'run,
                                _ => break 'events,
                            },
                        }
                    }

                    thread::sleep_ms(8u32);
                }

                let _ = tx_serv.send(ServerEvent::DisconnectedFromServer);

            }).expect("Couldn't start thread");
        }
    }
}
