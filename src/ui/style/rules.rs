
use ui::deps;
use ui::asset;
//use color::alpha::Rgba;

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selector: String,
    pub declarations: Vec<Declaration>,
}

pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Length(f32, Unit),
//    ColorValue(Rgba<u8>)
    Font(asset::FontData),
    Image(asset::ImageData),
    KeywordAuto,
}

#[derive(Debug)]
pub enum Unit {
    Px,
}

impl Stylesheet {

    #[inline]
    pub fn new() -> Stylesheet {
        Stylesheet {
            rules: Vec::new()
        }
    }
}
