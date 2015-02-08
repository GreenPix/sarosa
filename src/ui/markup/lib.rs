
use std::collections::HashMap;
use std::mem;

use ui::markup::tags::{Node, NodeType, TemplateData, View, Template};
use ui::ErrorReporter;
use util;

// Library
pub struct Library<E> {
    pub views: HashMap<String, View>,
    pub templates: HashMap<String, Template>,
    err: E,
}

impl<E> Library<E>
    where E: ErrorReporter
{

    pub fn new(reporter: E,
               views: HashMap<String, View>,
               templates: HashMap<String, Template>) -> Library<E>
    {
        Library {
            err: reporter,
            views: views,
            templates: templates
        }
    }

    pub fn merge(&mut self, other: Library<E>) {
        for (key, val) in other.views.into_iter() {
            self.views.insert(key, val);
        }

        for (key, val) in other.templates.into_iter() {
            self.templates.insert(key, val);
        }
    }

    /// # Resolve the dependencies
    ///
    /// Convert all templates that match a template definition
    /// with a group containing the templates childs.
    ///
    /// Note: this does not resolve data-bindings dependencies.
    pub fn resolve_templates(&mut self) {
        let ref mut views = self.views;
        let ref templates = self.templates;
        for (_, view) in views.iter_mut() {
            for node in view.children.iter_mut() {
                Library::<E>::resolve_templates_for_node(
                    &templates,
                    node
                );
            }
        }
    }

    fn resolve_templates_for_node(templates: &HashMap<String, Template>,
                                  node: &mut Node)
    {
        let mut child_index = 0;
        for child in node.children.iter_mut() {

            let test = match child.node_type {
                NodeType::Template(TemplateData { ref path }) => {
                    templates.get(path)
                }
                _ => None
            };

            match test {
                Some(found) => {
                    mem::swap(
                        child,
                        &mut Node::from_template(
                            found,
                            NodeType::Group
                        )
                    );
                }
                None => ()
            }
        }

        for child in node.children.iter_mut() {
            Library::<E>::resolve_templates_for_node(templates, child);
        }
    }
}
