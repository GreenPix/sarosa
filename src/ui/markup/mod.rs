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


/// Parser
pub struct Parser<E, B> {
    err: E,
    parser: EventReader<B>,
}



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

    pub fn parse(&mut self) -> Library
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
                        Err(()) => break 'doc,
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

        Library::new(views, templates)
    }

    fn parse_view(&mut self) -> Result<tags::View, ()>
    {
        let mut view = tags::View::new();

        match self.parse_loop(VIEW_TAG, &mut view) {
            Ok(()) => Ok(view),
            Err(()) => Err(())
        }
    }

    fn parse_template_decl(&mut self) -> Result<tags::Template, ()>
    {
        let mut template = tags::Template::new();

        match self.parse_loop(TEMPLATE_TAG, &mut template) {
            Ok(()) => Ok(template),
            Err(()) => Err(())
        }
    }

    fn parse_root_tag(&mut self,
                      views: &mut HashMap<String, tags::View>,
                      templates: &mut HashMap<String, tags::Template>,
                      name: &str,
                      attributes: &Vec<OwnedAttribute>) -> Result<(), ()>
    {
        match name {
            TEMPLATE_TAG => {
                let attr_name = lookup_name("name", attributes);

                match attr_name {
                    None => {
                        self.err.log(
                            "Template has no name add a name\
                             attribute 'name=\"<a-name>\"'".to_string()
                        );
                        Ok(())
                    }
                    Some(template_name) => {
                        match self.parse_template_decl() {
                            Ok(template) => {

                                templates.insert(template_name, template);
                                Ok(())
                            }
                            Err(()) => Err(())
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
                    Err(()) => Err(())
                }
            }
            _ => {
                let (row, col) = self.parser.get_cursor();
                self.err.log(
                    format!(
                        "Error {}:{} : Tag `{}` can't be at root level, \
                        you can only have `template` or `view`"
                    , row, col, name));
                Err(())
            }
        }
    }

    fn parse_tag(&mut self,
                 name: &str,
                 attributes: &Vec<OwnedAttribute>) -> Result<Option<tags::Node>, ()>
    {
        let node_type = match name {
            TEMPLATE_TAG     => tags::parse_template(attributes),
            GROUP_TAG        => Some(tags::NodeType::Group),
            BUTTON_TAG       => tags::parse_button(attributes),
            LINE_INPUT_TAG   => tags::parse_linput(attributes),
            PROGRESS_BAR_TAG => tags::parse_pbar(attributes),
            REPEAT_TAG       => tags::parse_repeat(attributes),
            _ => {
                let (row, col) = self.parser.get_cursor();
                self.err.log(
                    format!("Warning {}:{} : Unkown tag `{}`", row, col, name)
                );
                None
            }
        };

        match node_type {
            None => {
                match self.consume_children(name) {
                    Err(()) => Err(()),
                    Ok(()) => Ok(None)
                }
            }
            Some(nt) => {
                let classes = lookup_name("class", attributes);
                let mut node = tags::Node::new(classes, nt);

                // Propagate error if needed
                match self.parse_loop(name, &mut node) {
                    Err(()) => Err(()),
                    Ok(()) => Ok(Some(node))
                }
            }
        }
    }


    fn consume_children(&mut self, tag: &str) -> Result<(), ()>
    {
        let mut depth = 1i32;
        loop {
            match self.parser.next() {
                XmlEvent::StartElement { .. } => {

                    depth += 1;
                }
                XmlEvent::EndElement { name } => {

                    depth -= 1;
                    if name.local_name == tag && depth == 0 {
                        return Ok(());
                    }
                }
                XmlEvent::Error( e ) => {

                    self.err.log(format!("Error: {}", e));
                    return Err(());
                }
                _ => ()
            }
        }
    }

    fn parse_loop<T>(&mut self,
                     tag: &str,
                     parent: &mut T)
                     -> Result<(), ()>
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
                        // Error has been reported: stop parsing.
                        Err(()) => return Err(()),
                        // We're fine continue parsing.
                        Ok(node) => {
                            parent.add(node);
                        }
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
                    return Err(());
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
