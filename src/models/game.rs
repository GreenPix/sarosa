use std::collections::HashMap;
use models::player::Player;
use models::player::PlayerId;
use models::map::GameMap;
use std::slice::Iter;

pub struct GameData {
    players: Vec<Player>,
    player_id_to_index: HashMap<PlayerId, usize>,
    map: GameMap,
}

impl GameData {

    pub fn new() -> GameData {
        GameData {
            players: Vec::new(),
            player_id_to_index: HashMap::new(),
            map: GameMap::new(),
        }
    }

    /// This function allow to add a new player
    /// to the game. If the player was already there,
    /// then the player data is updated and `false` is returned
    /// If the player is new, `true` will be returned.
    pub fn add_player(&mut self, player: Player, id: PlayerId) -> bool {
        use std::collections::hash_map::Entry::*;

        match self.player_id_to_index.entry(id) {
            Occupied(e) => {
                unsafe {
                    *self.players.get_unchecked_mut(*(e.get())) = player;
                    false
                }
            },
            Vacant(e) => {
                let index = self.players.len();
                self.players.push(player);
                e.insert(index);
                true
            }
        }
    }

    pub fn players_len(&self) -> usize {
        self.players.len()
    }

    pub fn iter_players(&self) -> Iter<Player> {
        self.players.iter()
    }

    pub fn get_map(&self) -> &GameMap {
        &self.map
    }
}
