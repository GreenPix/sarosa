
use models::Tile;
use std::collections::HashMap;

mod object;
mod pipeline;

pub struct GameRenderer {
    objects: HashMap<String, object::Object>,
}

impl GameRenderer {

    fn new() -> GameRenderer {
        GameRenderer {
            objects: HashMap::new()
        }
    }
}
