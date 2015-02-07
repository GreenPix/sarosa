// Dependencies
use xml::attribute::OwnedAttribute;
use std::collections::HashSet;

use super::ErrorType;
use super::ErrorStatus;

// Name for the "main" view.
pub const MAIN_VIEW_NAME: &'static str = "main";

#[derive(PartialEq, Debug)]
pub enum NodeType {
    Text(String),
    Group,
    Button(ButtonData),
    LineInput(LineInputData),
    ProgressBar(ProgressBarData),
    Template(TemplateData),
    Repeat(RepeatData)
}

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    classes: Option<String>,
    node_type: NodeType,
}

#[derive(Debug)]
pub struct Template {
    children: Vec<Node>,
}

#[derive(Debug)]
pub struct View {
    children: Vec<Node>,
}

impl Node {

    pub fn new(classes: Option<String>, nt: NodeType) -> Node {
        Node {
            children: Vec::new(),
            node_type: nt,
            classes: classes,
        }
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.classes {
            Some(ref classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}

impl Template {

    pub fn new() -> Template {
        Template {
            children: Vec::new()
        }
    }
}

impl View {

    pub fn new() -> View {
        View {
            children: Vec::new()
        }
    }
}


pub trait HasNode {
    fn add(&mut self, maybe_child: Option<Node>);
}

impl HasNode for Node {
    fn add(&mut self, maybe_child: Option<Node>) {
        match maybe_child {
            Some(child) => self.children.push(child),
            None => ()
        }
    }
}

impl HasNode for Template {
    fn add(&mut self, maybe_child: Option<Node>) {
        match maybe_child {
            Some(child) => self.children.push(child),
            None => ()
        }
    }
}

impl HasNode for View {
    fn add(&mut self, maybe_child: Option<Node>) {
        match maybe_child {
            Some(child) => self.children.push(child),
            None => ()
        }
    }
}

// To help readability:
type ResOrError = Result<NodeType, super::ParseError>;

// ------------------------------------------------- Button tag
#[derive(PartialEq, Debug)]
pub struct ButtonData {
    pub gotoview: Option<String>,
    pub action: Option<String>,
    pub key: Option<String>,
}

pub fn parse_button(attributes: &Vec<OwnedAttribute>) -> ResOrError {
    Ok(NodeType::Button(ButtonData {
        gotoview: super::lookup_name("goto-view", attributes),
        action: super::lookup_name("action", attributes),
        key: super::lookup_name("key", attributes),
    }))
}

// ------------------------------------------------- Line input tag
#[derive(PartialEq, Debug)]
pub struct LineInputData {
    pub value: Option<String>,
    pub key: Option<String>,
}

pub fn parse_linput(attributes: &Vec<OwnedAttribute>) -> ResOrError {
    Ok(NodeType::LineInput(LineInputData {
        value: super::lookup_name("value", attributes),
        key: super::lookup_name("key", attributes),
    }))
}

// ------------------------------------------------- Progress bar tag
#[derive(PartialEq, Debug)]
pub struct ProgressBarData {
    pub value: Option<String>
}

pub fn parse_pbar(attributes: &Vec<OwnedAttribute>) -> ResOrError {
    Ok(NodeType::ProgressBar(ProgressBarData {
        value: super::lookup_name("value", attributes)
    }))
}

// ------------------------------------------------- Template tag
#[derive(PartialEq, Debug)]
pub struct TemplateData {
    pub path: String,
}

pub fn parse_template(attributes: &Vec<OwnedAttribute>) -> ResOrError {
    match super::lookup_name("path", attributes) {
        Some(path) => {
            Ok(NodeType::Template(TemplateData {
                path: path
            }))
        }
        None => {
            Err((
                ErrorType::Warning,
                ErrorStatus::NotReported(
                    "`path` attribute in `template` is missing")
            ))
        }
    }
}

// ------------------------------------------------- Repeat tag
#[derive(PartialEq, Debug)]
pub struct RepeatData {
    pub template_name: String,
    pub iter: String,
}

pub fn parse_repeat(attributes: &Vec<OwnedAttribute>) -> ResOrError {

    match (super::lookup_name("template-name", attributes),
           super::lookup_name("iter", attributes))
    {
        (Some(name), Some(iter)) => {
            Ok(NodeType::Repeat(RepeatData {
                template_name: name,
                iter: iter
            }))
        }
        (None, _) => {
            Err((
                ErrorType::Warning,
                ErrorStatus::NotReported(
                    "`template-name` attribute in `repeat` is missing")
            ))
        }
        (_, None) => {
            Err((
                ErrorType::Warning,
                ErrorStatus::NotReported(
                    "`iter` attribute in `repeat` is missing")
            ))
        }
    }
}
