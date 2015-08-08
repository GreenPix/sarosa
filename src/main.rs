extern crate sarosa;

fn main() {

    let settings = sarosa::Settings::new();
    let mut server = sarosa::Server::new(settings.clone());
    let mut win = sarosa::Window::new(settings.clone(), "Sarosa - Renaissance Project");
    let mut instance = sarosa::GameInstance::new(&win, settings.clone());
    let mut game = sarosa::GameLoop::new();
    game.run_loop(&mut win, &mut instance, &mut server);
}
