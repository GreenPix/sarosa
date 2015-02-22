
mod parser;

// Dependencies
use std::collections::HashMap;
use ui::report::ErrorReporter;
use ui::style;
use ui::asset;

/// Convenient function to parse a style.
pub fn parse<E, B>(reporter: E, reader: B) -> StyleDefinitions
    where E: ErrorReporter,
          B: Buffer
{
    let mut parser = parser::Parser::new(reporter, reader);
    parser.parse()
}

pub struct StyleDefinitions {
    pub defs: HashMap<String, Value>,
}

impl StyleDefinitions {
    pub fn new() -> StyleDefinitions {
        StyleDefinitions {
            defs: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    /// Number [0-9]+
    Number(f32),
    /// String ".+"
    Quote(String),
    /// Font(path, width, height)
    Font(String, f32, f32),
    /// Image(path)
    Image(String),
    // Add other construtor here...
}

impl Value {
    pub fn convert_to_style_value(&self) -> style::Value {
        // TODO: FIXME We don't do anything interesting here.
        match *self {
            Value::Number(v) => style::Value::Length(v, style::Unit::Px),
            Value::Quote(..) => style::Value::KeywordAuto,
            Value::Font(..) => style::Value::Font(asset::FontData),
            Value::Image(..) => style::Value::Image(asset::ImageData)
        }
    }
}
