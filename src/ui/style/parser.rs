
use std::num::FromStrRadix;
use ui::report::ErrorReporter;
use ui::libs::LibPath;

use super::Error;
use super::Value;
use super::Rule;
use super::Stylesheet;
use super::Unit;
use super::Declaration;

/// Parser
pub struct Parser<E, B> {
    err: E,
    buffer: B,
    row: usize,
    col: usize,
    tmp_char: Option<char>,
}

impl<E, B> Parser<E, B>
    where E: ErrorReporter,
          B: Buffer
{

    pub fn new(reporter: E, reader: B) -> Parser<E, B> {
        Parser {
            row: 0,
            col: 0,
            err: reporter,
            buffer: reader,
            tmp_char: None,
        }
    }

    pub fn parse(&mut self) -> Stylesheet {

        // Create stylesheet
        let mut stylesheet = Stylesheet::new();

        // Read token from the buffer.
        'rule: loop {
            self.consume_whitespace();

            // Is there anything to read ?
            match self.look_next_char() {
                None => break 'rule,
                _ => ()
            }

            match self.parse_rule() {
                Ok(rule) => {
                    stylesheet.rules.push(rule);
                }
                Err(err) => {
                    self.err.log(format!("Error {}", err));
                    break 'rule;
                }
            }
        }

        stylesheet
    }

    fn parse_rule(&mut self) -> Result<Rule, Error> {

        let selector = try!(self.parse_selector());
        let mut declarations = Vec::new();

        try!(self.consume_whitespace());
        match self.consume_char() {
            Some('{') => (),
            _ => return Err(self.error("Rule must start with a `{`"))
        }


        // Loop for declaration.
        'decl: loop {
            try!(self.consume_whitespace());

            match self.look_next_char() {
                Some('}') => break 'decl,
                Some(c) => {
                    let decl =  try!(self.parse_declaration());
                    declarations.push(decl);
                }
                None => return Err(self.error("Selector must end with a `}`"))
            }
        }

        // Consume '}'
        self.consume_char().unwrap();

        Ok(Rule {
            selector: selector,
            declarations: declarations
        })
    }

    fn parse_selector(&mut self) -> Result<String, Error> {

        try!(self.consume_whitespace());
        match self.consume_char() {
            Some('.') => (),
            _ => return Err(self.error("Selector must start with a `.`"))
        }
        self.consume_identifier()
    }

    fn parse_declaration(&mut self) -> Result<Declaration, Error> {

        try!(self.consume_whitespace());

        let name = try!(self.consume_identifier());

        try!(self.consume_whitespace());
        match self.consume_char() {
            Some(':') => (),
            _ => return Err(self.error("Invalid identifier expected `:`"))
        }

        let value = try!(self.parse_value());

        try!(self.consume_whitespace());
        match self.consume_char() {
            Some(';') => (),
            _ => return Err(self.error("Declaration should end with `;`"))
        }

        Ok(Declaration {
            name: name,
            value: value
        })
    }

    fn parse_value(&mut self) -> Result<Value, Error> {

        try!(self.consume_whitespace());
        match self.look_next_char() {
            Some(c) => match c {
                '$' => {
                    self.consume_char();
                    let path = try!(self.consume_path());
                    Ok(Value::LibPathValue(LibPath(path)))
                },
                'a' => {
                    let auto = try!(self.consume_identifier());
                    if auto == "auto" {
                        Ok(Value::KeywordAuto)
                    } else {
                        Err(self.error("Did you mean `auto`?"))
                    }
                }
                '0'...'9' => {
                    let val = try!(self.consume_number());
                    let unit = try!(self.consume_unit());
                    Ok(Value::Length(val, unit))
                }
                _ => Err(self.error("Unknown value."))
            },
            None => Err(self.error("Unexpected end of input. Expected Value."))
        }
    }

    fn consume_identifier(&mut self) -> Result<String, Error> {
        self.consume_while(valid_identifier_char)
    }

    fn consume_number(&mut self) -> Result<f32, Error> {
        let num: String = try!(self.consume_while(CharExt::is_numeric));
        FromStrRadix::from_str_radix(&num, 10)
            .map_err(|err| {
                Error::new(self.row, self.col, format!("Incorrect float value: {}", err))
            })
    }

    fn consume_unit(&mut self) -> Result<Unit, Error> {
        try!(self.consume_identifier());
        Ok(Unit::Px)
    }

    fn consume_path(&mut self) -> Result<String, Error> {
        self.consume_while(valid_path_char)
    }

    fn consume_whitespace(&mut self) -> Result<(), Error> {
        try!(self.consume_while(CharExt::is_whitespace));
        Ok(())
    }

    /// Consume characters until `test` returns false.
    /// This function return Err() only if the end of the stream
    /// is encountered.
    fn consume_while<F>(&mut self, test: F) -> Result<String, Error>
        where F: Fn(char) -> bool
    {
        let mut result = String::new();
        loop {
            match self.look_next_char() {
                Some(c) => {
                    if !test(c) {
                        return Ok(result)
                    }
                    self.consume_char();
                    result.push(c);
                }
                None => return Err(self.error("Unexpected end of stream"))
            }
        }
    }

    fn consume_char(&mut self) -> Option<char> {
        if self.tmp_char.is_none() {
            match self.buffer.read_char().ok() {
                Some(c) => {
                    if c == '\n' {
                        self.row += 1;
                        self.col = 0;
                    } else {
                        self.col += 1;
                    }
                    self.tmp_char = Some(c);
                    Some(c)
                }
                None => None
            }
        } else {
            let c = self.tmp_char.unwrap();
            self.tmp_char = None;
            Some(c)
        }
    }

    fn look_next_char(&mut self) -> Option<char> {
        if self.tmp_char.is_none() {
            self.consume_char()
        } else {
            self.tmp_char
        }
    }

    fn error(&self, msg: &str) -> Error {
        Error::new(self.row, self.col, msg.to_string())
    }
}

// ======================================== //
//                  HELPERS                 //
// ======================================== //

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' => true,
        _ => false,
    }
}

fn valid_path_char(c: char) -> bool {
    match c {
        '.' => true,
        _ => valid_identifier_char(c)
    }
}
