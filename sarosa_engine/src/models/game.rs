use std::collections::HashMap;
use std::slice::Iter;
use cgmath::Vector2;

use models::player::Player;
use models::player::THIS_PLAYER;
use models::player::PlayerId;
use models::map::GameMap;
use animation::AnimationManager;

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
    pub fn add_player(&mut self, id: PlayerId, player: Player) -> bool {
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

    pub fn remove_player(&mut self, id: PlayerId) {
        use std::collections::hash_map::Entry::*;

        // Swap last player in list to the player removed.
        let last_el_place = self.players.len().saturating_sub(1) as u64;
        match self.player_id_to_index.entry(id) {
            Occupied(e) => {
                self.players.swap_remove(*(e.get()));
            }
            Vacant(_) => {
                warn!("Received `delete player` for unknown player id: {}", id);
                return;
            }
        }
        // Remove the last element, which must be here
        self.player_id_to_index.remove(&last_el_place);
    }

    pub fn update_player(&mut self, id: PlayerId, pos: Vector2<f32>, speed: Vector2<f32>) {
        use std::collections::hash_map::Entry::*;

        match self.player_id_to_index.entry(id) {
            Occupied(e) => {
                unsafe {
                    let ref mut player = self.players.get_unchecked_mut(*(e.get()));
                    player.position = pos;
                    player.speed = speed;
                }
            },
            Vacant(_) => warn!("Received `update player` for unknown player id: {}", id),
        }
    }

    pub fn this_player(&self) -> Option<&Player> {
        self.player_id_to_index.get(&THIS_PLAYER).map(|&index| {
            unsafe { self.players.get_unchecked(index) }
        })
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

    pub fn fixed_update(&mut self, anim_manager: &AnimationManager, time_elapsed: u64) {
        for player in self.players.iter_mut() {
            player.animator.update(anim_manager, time_elapsed, &player.speed);
        }
    }
}
