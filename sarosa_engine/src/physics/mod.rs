
use models::Player;
use cgmath::Vector2;

struct PhysicsEngine;


impl PhysicsEngine {

    pub fn move_player(&self, player: &mut Player, offset: &Vector2<f32>) {
        // TODO check possibility of move.
        player.position = player.position + *offset;
    }

    pub fn set_player_at(&self, player: &mut Player, position: &Vector2<f32>) {
        // TODO check the position / set the closest one.
        player.position = *position;
    }
}
