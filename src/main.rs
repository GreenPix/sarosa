extern crate sarosa_engine as sarosa;
extern crate env_logger;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::ops::Deref;

// Version support
include!(concat!(env!("OUT_DIR"), "/sarosa_version.rs"));

static USAGE: &'static str = "
Sarosa client.

Usage:
  sarosa [--host <host> --port <port>]
  sarosa --offline
  sarosa (-h | --help)
  sarosa --version

Options:
  -h --help         Show this screen.
  --version         Show version.
  --offline         Run a self-hosted offline server.
  --port <port>     Server port     [default: 7777].
  --host <host>     Server Hostname [default: localhost].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_offline: bool,
    flag_host: String,
    flag_port: u16,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{}", sarosa_version());
        return;
    }

    env_logger::init().unwrap();

    let mut address = args.flag_host;
    address.push_str(":"); address.push_str(args.flag_port.to_string().deref());

    // Initialization
    let settings = sarosa::Settings::new(address, args.flag_offline);
    let mut server = sarosa::Server::new(settings.clone());
    let mut win = sarosa::Window::new(settings.clone(), "Sarosa - Renaissance Project");
    let mut instance = sarosa::GameInstance::new(&win, settings.clone());
    let mut game = sarosa::GameLoop::new();

    // Try to connect to the server
    println!("Connecting to server...");
    server.connect();

    // Run the game.
    game.run_loop(&mut win, &mut instance, &mut server);
}
