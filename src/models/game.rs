
use models::player::Player;
use models::map::Map;
use std::slice::Iter;

pub struct Game {
    players: Vec<Player>,
    map: Map,
}

impl Game {

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn set_map(&mut self, map: Map) {
        self.map = map;
    }

    fn iter_players(&self) -> Iter<Player> {
        self.players.iter()
    }

    fn get_map(&self) -> &Map {
        &self.map
    }
}
