
use std::collections::HashMap;
use rendering::object;
use glium::Frame;

#[derive(Default)]
pub struct WorldScene {
    objects: HashMap<String, object::Object>,
}

impl WorldScene {


    pub fn render(&self, _: &Frame) {}
}
