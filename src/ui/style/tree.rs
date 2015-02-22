
use std::collections::HashMap;
use ui::markup::Node;
use ui::markup::NodeType;
use super::Value;

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
const BORDER_SIZE_WIDTH: &'static str = "border-width";
const BORDER_SIZE_HEIGHT: &'static str = "border-height";
const BORDER_SIZE: &'static str = "border";
const BORDER_SIZE: &'static str = "border";



pub struct StyledNode<'a> {
    node: &'a Node,
    property_values: HashMap<PropertyName, Value>,
    children: Vec<StyledNode<'a>>,
}

// fn do_stuff(node: &Node, ss: &Stylesheet) {
//     let mut p
//     ss.rules.
// }
