
use ui::libs::LibPath;
//use color::alpha::Rgba;

pub struct Stylesheet {
    rules: Vec<Rule>,
}

pub struct Rule {
    selector: String,
    declarations: Vec<Declaration>,
}

pub struct Declaration {
    name: String,
    value: Value,
}

pub enum Value {
    Length(f32, Unit),
//    ColorValue(Rgba<u8>)
    LibPathValue(LibPath),
    KeywordAuto,
}

pub enum Unit {
    Px,
}
