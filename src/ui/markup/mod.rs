
pub mod tags;

use xml::reader::EventReader;
use xml::reader::events::*;
use xml::attribute::OwnedAttribute;
use std::old_io::Buffer;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Str;

use ui::ErrorReporter;



// Tag list
const TEMPLATE_TAG: &'static str = "template";
const VIEW_TAG: &'static str = "view";
const GROUP_TAG: &'static str = "group";
const BUTTON_TAG: &'static str = "button";
const LINE_INPUT_TAG: &'static str = "line-input";
const PROGRESS_BAR_TAG: &'static str = "progress-bar";
const REPEAT_TAG: &'static str = "repeat";

/// Parser
pub struct Parser<E> {
    views: HashMap<String, tags::View>,
    templates: HashMap<String, tags::Template>,
    err: E,
}


impl<E> Parser<E> where E: ErrorReporter {

    pub fn new(reporter: E) -> Parser<E> {
        Parser {
            views: HashMap::new(),
            templates: HashMap::new(),
            err: reporter,
        }
    }

    pub fn parse<B>(&self, reader: B)
        where B: Buffer
    {
        let mut parser = EventReader::new(reader);

        'doc: loop {

            match parser.next() {
                XmlEvent::StartElement { name, .. } => {
                    println!("{}", name.local_name);
                    match name.local_name.as_slice() {
                        TEMPLATE_TAG => self.parse_template_decl(&mut parser),
                        VIEW_TAG => self.parse_view(&mut parser),
                        _ => {
                            self.err.log(
                                format!("
                                    Error: Tag {} can't be at root level,
                                    you can only have 'template' or 'view'
                                ", name));
                            break 'doc;
                        }
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
    }

    fn parse_view<B>(&self, parser: &mut EventReader<B>)
        where B: Buffer
    {
        let mut view = tags::View::new();

        self.parse_loop(VIEW_TAG, parser, &mut view);

        println!("{:?}", view);
    }

    fn parse_template_decl<B>(&self, parser: &mut EventReader<B>)
        where B: Buffer
    {
        let mut template = tags::Template::new();

        self.parse_loop(TEMPLATE_TAG, parser, &mut template);

        println!("{:?}", template);
    }

    fn parse_loop<B, T>(&self,
                        tag: &'static str,
                        parser: &mut EventReader<B>,
                        parent: &mut T)
        where B: Buffer,
              T: tags::HasNode
    {
        let mut depth = 1i32;
        'out: loop {
            match parser.next() {
                XmlEvent::StartElement { name, attributes, .. } => {

                    depth += 1;
                    parent.add(
                        self.parse_tag(
                            name.local_name.as_slice(),
                            &attributes
                        )
                    );
                }
                XmlEvent::EndElement { name } => {

                    depth -= 1;
                    if (name.local_name.as_slice() == tag && depth == 0) {
                        break 'out;
                    }
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
                    break 'out;
                }
                _ => ()
            }
        }
    }

    fn parse_tag(&self,
                 name: &str,
                 attributes: &Vec<OwnedAttribute>) -> Option<tags::Node>
    {
        let nodeType = match name {
            TEMPLATE_TAG     => tags::NodeType::Template(parse_template(attributes)),
            GROUP_TAG        => tags::NodeType::Group,
            BUTTON_TAG       => tags::NodeType::Button(parse_button(attributes)),
            LINE_INPUT_TAG   => tags::NodeType::LineInput(parse_linput(attributes)),
            PROGRESS_BAR_TAG => tags::NodeType::ProgressBar(parse_pbar(attributes)),
            REPEAT_TAG       => tags::NodeType::Repeat(parse_repeat(attributes)),
            _ => {
                self.err.log(
                    format!("Unkown tag: {}", name)
                );
                tags::NodeType::None
            }
        };

        let classes = lookup_name("class", attributes);

        if (nodeType == tags::NodeType::None) {
            None
        } else {
            Some(tags::Node::new(classes, nodeType))
        }
    }
}

// ======================================== //
//                  HELPERS                 //
// ======================================== //

fn parse_template(attributes: &Vec<OwnedAttribute>) -> tags::TemplateData {
    tags::TemplateData {
        path: lookup_name("path", attributes)
    }
}

fn parse_button(attributes: &Vec<OwnedAttribute>) -> tags::ButtonData {
    tags::ButtonData {
        gotoview: lookup_name("goto-view", attributes),
        action: lookup_name("action", attributes),
        key: lookup_name("key", attributes),
    }
}

fn parse_linput(attributes: &Vec<OwnedAttribute>) -> tags::LineInputData {
    tags::LineInputData {
        value: lookup_name("value", attributes),
        key: lookup_name("key", attributes),
    }
}

fn parse_pbar(attributes: &Vec<OwnedAttribute>) -> tags::ProgressBarData {
    tags::ProgressBarData {
        value: lookup_name("value", attributes)
    }
}

fn parse_repeat(attributes: &Vec<OwnedAttribute>) -> tags::RepeatData {
    tags::RepeatData {
        templateName: lookup_name("template-name", attributes),
        iter: lookup_name("iter", attributes)
    }
}

fn lookup_name<'a>(name: &'a str,
                   attributes: &Vec<OwnedAttribute>)
                   -> Option<String>
{
    attributes.iter()
        .find(|ref attribute| attribute.name.local_name.as_slice() == name)
        .map(|ref attribute| attribute.value.clone())
}
