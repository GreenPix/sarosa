
pub use self::rules::Stylesheet;
pub use self::rules::Rule;

mod rules;


/// Convenient function to parse a style.
pub fn parse<E, B>(reporter: E, reader: B) -> Stylesheet
    where E: ErrorReporter,
          B: Buffer
{
    let mut parser = Parser::new(reporter, reader);
    parser.parse()
}

/// Parser
struct Parser<E, B> {
    err: E,
    buffer: B,
}

impl<E, B> Parser<E, B>
    where E: ErrorReporter,
          B: Buffer
{

    pub fn new(reporter: E, reader: B) -> Parser<E, B> {
        Parser {
            err: reporter,
            buffer: reader,
        }
    }

    pub fn parse(&mut self) -> Stylesheet {
        // TODO..
    }
}
