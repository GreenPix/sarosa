
use std::collections::HashSet;

#[derive(PartialEq, Show)]
pub enum NodeType {
    Text(String),
    Group,
    Button(ButtonData),
    LineInput(LineInputData),
    ProgressBar(ProgressBarData),
    Template(TemplateData),
    Repeat(RepeatData),
    None
}

#[derive(Show)]
pub struct Node {
    children: Vec<Node>,
    classes: Option<String>,
    nodeType: NodeType,
}

#[derive(Show)]
pub struct Template {
    children: Vec<Node>,
}

#[derive(Show)]
pub struct View {
    children: Vec<Node>,
}

impl Node {

    pub fn new(classes: Option<String>, nt: NodeType) -> Node {
        Node {
            children: Vec::new(),
            nodeType: nt,
            classes: classes,
        }
    }

    // pub fn classes(&self) -> HashSet<&str> {
    //     match self.classes {
    //         Some(classlist) => classlist.split(' ').collect(),
    //         None => HashSet::new()
    //     }
    // }
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

#[derive(PartialEq, Show)]
pub struct ButtonData {
    pub gotoview: Option<String>,
    pub action: Option<String>,
    pub key: Option<String>,
}

#[derive(PartialEq, Show)]
pub struct LineInputData {
    pub value: Option<String>,
    pub key: Option<String>,
}

#[derive(PartialEq, Show)]
pub struct ProgressBarData {
    pub value: Option<String>
}

#[derive(PartialEq, Show)]
pub struct TemplateData {
    pub path: Option<String>,
}

#[derive(PartialEq, Show)]
pub struct RepeatData {
    pub templateName: Option<String>,
    pub iter: Option<String>,
}
