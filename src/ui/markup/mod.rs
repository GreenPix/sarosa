// Dependencies
use xml::reader::EventReader;
use xml::reader::events::*;
use xml::attribute::OwnedAttribute;
use std::old_io::Buffer;

use std::collections::HashMap;

use ui::ErrorReporter;

// Re-export
pub use self::tags::Node;
pub use self::tags::NodeType;
pub use self::tags::{Template, View};
pub use self::tags::{
    ButtonData,
    LineInputData,
    ProgressBarData,
    TemplateData,
    RepeatData
};
pub use self::lib::Library;

mod lib;
mod tags;


// Tag list
const TEMPLATE_TAG: &'static str = "template";
const VIEW_TAG: &'static str = "view";
const GROUP_TAG: &'static str = "group";
const BUTTON_TAG: &'static str = "button";
const LINE_INPUT_TAG: &'static str = "line-input";
const PROGRESS_BAR_TAG: &'static str = "progress-bar";
const REPEAT_TAG: &'static str = "repeat";


/// Parse the given buffer.
///
/// # Example:
///
/// ```
/// let reader = BufferedReader::new(
///     "<view name=\"toto\">\
///     </view>\
/// ".as_bytes());
/// ui::markup::parse(ui::StdOutErrorReporter, reader);
/// ```
pub fn parse<E, B>(reporter: E, reader: B) -> Library<E>
    where E: ErrorReporter,
          B: Buffer
{
    let mut parser = Parser::new(reporter, reader);
    parser.parse()
}



/// Parser
struct Parser<E, B> {
    err: E,
    parser: EventReader<B>,
}

enum ErrorStatus {
    NotReported(&'static str),
    Reported,
}

enum ErrorType {
    Fatal,
    Warning,
}

type ParseError = (ErrorType, ErrorStatus);

impl<E, B> Parser<E, B>
    where E: ErrorReporter,
          B: Buffer
{

    pub fn new(reporter: E, reader: B) -> Parser<E, B> {
        Parser {
            err: reporter,
            parser: EventReader::new(reader)
        }
    }

    pub fn parse(&mut self) -> Library<E>
    {
        let mut views = HashMap::new();
        let mut templates = HashMap::new();

        'doc: loop {

            match self.parser.next() {
                XmlEvent::StartElement { name, attributes, .. } => {

                    let test_parse = self.parse_root_tag(
                        &mut views,
                        &mut templates,
                        &name.local_name,
                        &attributes
                    );

                    match test_parse {
                        Err((ErrorType::Fatal, _)) => break 'doc,
                        _ => ()
                    }
                }
                XmlEvent::Error(e) => {
                    self.err.log(format!("Error: {}", e));
                    break 'doc;
                }
                XmlEvent::EndDocument => break 'doc,
                XmlEvent::StartDocument { .. } => (),
                _ => unreachable!(),
            }
        }

        Library::new(self.err, views, templates)
    }

    fn parse_view(&mut self) -> Result<tags::View, ParseError>
    {
        let mut view = tags::View::new();

        match self.parse_loop(VIEW_TAG, &mut view) {
            Ok(()) => Ok(view),
            Err(error_reported) => Err(error_reported)
        }
    }

    fn parse_template_decl(&mut self) -> Result<tags::Template, ParseError>
    {
        let mut template = tags::Template::new();

        match self.parse_loop(TEMPLATE_TAG, &mut template) {
            Ok(()) => Ok(template),
            Err(error_reported) => Err(error_reported)
        }
    }

    fn parse_root_tag(&mut self,
                      views: &mut HashMap<String, tags::View>,
                      templates: &mut HashMap<String, tags::Template>,
                      name: &str,
                      attributes: &Vec<OwnedAttribute>) -> Result<(), ParseError>
    {
        match name {
            TEMPLATE_TAG => {
                let attr_name = lookup_name("name", attributes);

                match attr_name {
                    None => {
                        let (row, col) = self.parser.get_cursor();
                        self.err.log(
                            format!(
                                "Warning {}:{} : `template` has no name add an \
                                 attribute 'name=\"<a-name>\"'",
                            row, col)
                        );

                        match self.consume_children(name) {
                            Err(parse_err) => Err(parse_err),
                            Ok(()) => Ok(())
                        }
                    }
                    Some(template_name) => {
                        match self.parse_template_decl() {
                            Ok(template) => {

                                templates.insert(template_name, template);
                                Ok(())
                            }
                            Err(error_reported) => Err(error_reported)
                        }
                    }
                }
            }
            VIEW_TAG => {
                match self.parse_view() {
                    Ok(view) => {
                        let attr_name = lookup_name("name", attributes)
                            .unwrap_or(tags::MAIN_VIEW_NAME.to_string());
                        views.insert(attr_name, view);
                        Ok(())
                    }
                    Err(error_reported) => Err(error_reported)
                }
            }
            _ => {
                let (row, col) = self.parser.get_cursor();
                self.err.log(
                    format!(
                        "Warning {}:{} : `{}` can't be at root level, \
                        you can only have `template` or `view`"
                    , row+1, col+1, name));

                match self.consume_children(name) {
                    Err(parse_err) => Err(parse_err),
                    Ok(()) => Ok(())
                }
            }
        }
    }

    fn parse_tag(&mut self,
                 name: &str,
                 attributes: &Vec<OwnedAttribute>)
                 -> Result<Option<tags::Node>, ParseError>
    {
        let ignore_child = name == TEMPLATE_TAG;

        let node_type = match name {
            TEMPLATE_TAG     => tags::parse_template(attributes),
            GROUP_TAG        => Ok(tags::NodeType::Group),
            BUTTON_TAG       => tags::parse_button(attributes),
            LINE_INPUT_TAG   => tags::parse_linput(attributes),
            PROGRESS_BAR_TAG => tags::parse_pbar(attributes),
            REPEAT_TAG       => tags::parse_repeat(attributes),
            _ => {
                let (row, col) = self.parser.get_cursor();
                self.err.log(
                    format!("Warning {}:{} : Unknown tag `{}`", row+1, col+1, name)
                );
                Err((ErrorType::Warning, ErrorStatus::Reported))
            }
        };

        match node_type {
            Err(parse_error) => {
                match self.report_error_if_needed(parse_error) {
                    (ErrorType::Warning, _) => {
                        match self.consume_children(name) {
                            Err(parse_err) =>
                                Err(self.report_error_if_needed(parse_err)),
                            Ok(()) =>
                                Ok(None)
                        }
                    },
                    reported_parse_error => Err(reported_parse_error)
                }
            }
            Ok(nt) => {
                let classes = lookup_name("class", attributes);
                let mut node = tags::Node::new(classes, nt);

                if ignore_child {

                    // Consume children
                    match self.consume_children(name) {
                        Ok(()) => Ok(Some(node)),
                        Err(reported_error) => Err(reported_error),
                    }

                } else {

                    // Parse children
                    match self.parse_loop(name, &mut node) {
                        Ok(()) => Ok(Some(node)),
                        Err(reported_error) => Err(reported_error),
                    }
                }
            }
        }
    }

    fn report_error_if_needed(&mut self,
                              parse_error: ParseError) -> ParseError
    {
        let (row, col) = self.parser.get_cursor();
        match parse_error {
            (ErrorType::Fatal, ErrorStatus::NotReported(msg)) => {
                self.err.log(
                    format!("Error {}:{} : {}", row+1, col+1, msg)
                );
                (ErrorType::Fatal, ErrorStatus::Reported)
            }
            (ErrorType::Warning, ErrorStatus::NotReported(msg)) => {
                self.err.log(
                    format!("Warning {}:{} : {}", row+1, col+1, msg)
                );
                (ErrorType::Warning, ErrorStatus::Reported)
            }
            _ => parse_error
        }
    }

    // This function may only return Ok(()) or
    // Err((ErrorType::Fatal, ErrorStatus::Reported)).
    fn consume_children(&mut self, tag: &str) -> Result<(), ParseError>
    {
        let mut depth = 1i32;
        loop {
            match self.parser.next() {
                XmlEvent::StartElement { name, .. } => {

                    depth += 1;

                    let (row, col) = self.parser.get_cursor();
                    self.err.log(
                        format!("Warning {}:{}, `{}` has been ignored",
                                row+1, col+1, name)
                    );
                }
                XmlEvent::EndElement { name } => {

                    depth -= 1;
                    if name.local_name == tag && depth == 0 {
                        return Ok(());
                    }
                }
                XmlEvent::Error( e ) => {

                    self.err.log(format!("Error: {}", e));
                    return Err((ErrorType::Fatal, ErrorStatus::Reported));
                }
                _ => ()
            }
        }
    }

    fn parse_loop<T>(&mut self,
                     tag: &str,
                     parent: &mut T)
                     -> Result<(), ParseError>
        where T: tags::HasNode
    {
        loop {
            match self.parser.next() {
                XmlEvent::StartElement { name, attributes, .. } => {

                    let test_parse_child = self.parse_tag(
                        &name.local_name,
                        &attributes
                    );

                    match test_parse_child {
                        // We're fine continue parsing.
                        Ok(node) => {
                            parent.add(node);
                        },
                        // Error has been reported: stop parsing.
                        Err(reported_error) => return Err(reported_error),
                    }
                }
                XmlEvent::EndElement { name } => {

                    // TODO: remove at some point.
                    assert_eq!(name.local_name, tag);
                    return Ok(());
                }
                XmlEvent::Characters( text ) => {

                    parent.add(
                        Some(tags::Node::new(
                            None,
                            tags::NodeType::Text(text)
                        ))
                    );
                }
                XmlEvent::Error( e ) => {

                    self.err.log(format!("Error: {}", e));
                    return Err((ErrorType::Fatal, ErrorStatus::Reported));
                }
                XmlEvent::EndDocument => unreachable!(),
                _ => ()
            }
        }
    }

}

// ======================================== //
//                  HELPERS                 //
// ======================================== //

fn lookup_name<'a>(name: &'a str,
                   attributes: &Vec<OwnedAttribute>)
                   -> Option<String>
{
    attributes.iter()
        .find(|ref attribute| attribute.name.local_name == name)
        .map(|ref attribute| attribute.value.clone())
}


#[cfg(test)]
mod tests {

    use std::old_io::BufferedReader;
    use ui::EmptyErrorReporter;

    #[test]
    fn reject_invalid_root_tags() {
        let reader = BufferedReader::new("<test></test>".as_bytes());
        let mut parser = super::Parser::new(EmptyErrorReporter, reader);

        let res = parser.parse();
        assert_eq!(res.views.len(), 0);
        assert_eq!(res.templates.len(), 0);
    }

    #[test]
    fn ignore_unknown_tags() {
        let reader = BufferedReader::new(
            "<view>\
                <toto />\
                <h1>Test</h1>\
             </view>
            ".as_bytes());
        let mut parser = super::Parser::new(EmptyErrorReporter, reader);

        let res = parser.parse();

        assert_eq!(res.views.len(), 1);
        assert_eq!(res.views.values().next().unwrap().children.len(), 0);
        assert_eq!(res.templates.len(), 0);
    }

    #[test]
    fn reject_unnamed_template() {
        let reader = BufferedReader::new(
            "<template>\
                <toto />\
             </template>
            ".as_bytes());
        let mut parser = super::Parser::new(EmptyErrorReporter, reader);

        let res = parser.parse();

        assert_eq!(res.views.len(), 0);
        assert_eq!(res.templates.len(), 0);
    }

    #[test]
    fn ignore_ill_formed_repeat_1() {
        let reader = BufferedReader::new(
            "<view>\
                <repeat template-name=\"test\"/>\
             </view>
            ".as_bytes());
        let mut parser = super::Parser::new(EmptyErrorReporter, reader);

        let res = parser.parse();

        assert_eq!(res.views.len(), 1);
        assert_eq!(res.views.values().next().unwrap().children.len(), 0);
        assert_eq!(res.templates.len(), 0);
    }

    #[test]
    fn ignore_ill_formed_repeat_2() {
        let reader = BufferedReader::new(
            "<view>\
                <repeat iter=\"{test}\"/>\
             </view>
            ".as_bytes());
        let mut parser = super::Parser::new(EmptyErrorReporter, reader);

        let res = parser.parse();

        assert_eq!(res.views.len(), 1);
        assert_eq!(res.views.values().next().unwrap().children.len(), 0);
        assert_eq!(res.templates.len(), 0);
    }

    #[test]
    fn accept_well_formed_repeat() {
        let reader = BufferedReader::new(
            "<view>\
                <repeat iter=\"{arf}\" template-name=\"test\"/>\
             </view>
            ".as_bytes());
        let mut parser = super::Parser::new(EmptyErrorReporter, reader);

        let res = parser.parse();

        assert_eq!(res.views.len(), 1);
        assert_eq!(res.views.values().next().unwrap().children.len(), 1);
        assert_eq!(res.templates.len(), 0);
    }
}
