
use std::collections::HashMap;
use ui::markup::Node;
use ui::markup::NodeType;
use super::Value;
use super::Stylesheet;

// TODO: FIXME
// Remember to have a property layout to either
// render right to left (rtl) or left to right (ltr)
// ```
//      layout: rtl; // ltr is default
// ```
// That does not affect the layout algorithm
// instead, the x property is inversed as parent.width - x
//
//
type PropertyName = &'static str;

// Acceptable properties:
//const BORDER_SIZE_WIDTH: &'static str = "border-width";
//const BORDER_SIZE_HEIGHT: &'static str = "border-height";
//const BORDER_SIZE: &'static str = "border";
//const BORDER_SIZE: &'static str = "border";

pub fn build_style_tree<'a, 'b>(node: &'a Node, stylesheet: &'b Stylesheet) -> StyledNode<'a> {
    let mut styled_node = StyledNode::<'a>::new(node);
    styled_node.set_properties(stylesheet);
    styled_node
}

pub struct StyledNode<'a> {
    node: &'a Node,
    property_values: HashMap<PropertyName, Value>,
    children: Vec<StyledNode<'a>>,
}

impl<'a> StyledNode<'a> {

    fn new(node: &'a Node) -> StyledNode<'a> {
        let mut children = Vec::with_capacity(node.children.len());
        for kid in node.children.iter() {
            children.push(StyledNode::new(kid));
        }

        StyledNode {
            node: node,
            property_values: HashMap::new(),
            children: children
        }
    }

    fn set_properties(&mut self, style: &Stylesheet) {
        let classes = self.node.classes();
        let ref mut properties = self.property_values;
        // We loop over rules because at some
        // point, we might want to sort them based
        // on specificity in the same way that it is done
        // in CSS. It would help understanding which
        // rule does define a particular property.
        // Thus the code below wouldn't change.
        for rule in style.rules.iter() {
            if classes.contains(rule.selector.as_slice()) {
                for dec in rule.declarations.iter() {
                    properties.insert(&dec.name, dec.value.clone());
                }
            }
        }

        for kid in self.children.iter_mut() {
            kid.set_properties(style);
        }
    }
}
