
pub use self::rules::Stylesheet;
pub use self::rules::Rule;
pub use self::rules::Declaration;
pub use self::rules::Value;
pub use self::rules::Unit;
pub use self::error::Error;

mod rules;
mod error;
mod parser;

use ui::report::ErrorReporter;


/// Convenient function to parse a style.
pub fn parse<E, B>(reporter: E, reader: B) -> Stylesheet
    where E: ErrorReporter,
          B: Buffer
{
    let mut parser = parser::Parser::new(reporter, reader);
    parser.parse()
}
