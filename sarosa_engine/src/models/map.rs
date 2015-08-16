
use animation::TextureId;

pub struct GameMap {
    tex_id: TextureId,
    width: u32,
    height: u32,
}


impl GameMap {

    pub fn new() -> GameMap {
        GameMap {
            tex_id: TextureId(0),
            width: 300,
            height: 200,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tex_id(&self) -> TextureId {
        self.tex_id
    }
}
