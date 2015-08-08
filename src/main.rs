extern crate sarosa_engine as sarosa;
extern crate env_logger;


fn main() {

    env_logger::init().unwrap();

    // Initialization
    let settings = sarosa::Settings::new("localhost:6666");
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
