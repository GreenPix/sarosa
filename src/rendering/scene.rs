
use std::collections::HashMap;
use rendering::object;

#[derive(Default)]
pub struct WorldScene {
    objects: HashMap<String, object::Object>,
}
