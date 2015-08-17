use std::slice::Iter;
use cgmath::Vector2;
use num::traits::Zero;

use models::player::Player;
use models::player::THIS_PLAYER;
use models::player::PlayerId;
use models::map::GameMap;
use animation::TextureId;
use animation::AnimationManager;

pub struct GameData {
    players: Vec<Player>,
    players_id: Vec<PlayerId>,
    map: GameMap,
}

impl GameData {

    pub fn new(this_player_tex_id: TextureId, anim_manager: &AnimationManager)
        -> GameData
    {
        assert_eq!(THIS_PLAYER, 0);

        let mut players = Vec::with_capacity(20);
        players.push(Player::new(
                Vector2::zero(),
                Vector2::zero(),
                this_player_tex_id,
                anim_manager
        ));

        let mut players_id = Vec::with_capacity(20);
        players_id.push(THIS_PLAYER);

        GameData {
            players: Vec::new(),
            players_id: Vec::new(),
            map: GameMap::new(),
        }
    }

    /// This function allow to add a new player
    /// to the game. If the player was already there,
    /// then the player data is updated and `false` is returned
    /// If the player is new, `true` will be returned.
    pub fn add_player(&mut self, id: PlayerId, player: Player) -> bool {

        let index = match self.players_id.binary_search(&id) {
            Ok(index) => {
                unsafe {
                    *self.players.get_unchecked_mut(index) = player
                }
                return false;
            },
            Err(index) => index,
        };

        self.players.insert(index, player);
        self.players_id.insert(index, id);
        true
    }

    pub fn remove_player(&mut self, id: PlayerId) {

        let index = match self.players_id.binary_search(&id) {
            Ok(index) => index,
            Err(_) => {
                warn!("Received `delete player` for unknown player id: {}", id);
                return;
            }
        };

        self.players.remove(index);
        self.players_id.remove(index);
    }

    pub fn update_player(&mut self, id: PlayerId, pos: Vector2<f32>, speed: Vector2<f32>) {

        match self.players_id.binary_search(&id) {
            Ok(index) => {
                unsafe {
                    let ref mut player = self.players.get_unchecked_mut(index);
                    player.position = pos;
                    player.speed = speed;
                }
            },
            Err(_) => warn!("Received `update player` for unknown player id: {}", id),
        }
    }

    pub fn this_player(&self) -> Option<&Player> {
        self.players.get(THIS_PLAYER as usize)
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
