use crate::ast::*;
use std::collections::HashMap;

pub struct Item {
    name: String,
    attributes: Vec<String>,
    args: Vec<Box<ExprEnum>>,
    // actions: HashMap<String, &ActionNode>,
}

impl Item {
    pub fn new(node: &ItemInstanceNode, templates: &HashMap<String, &ItemTemplateNode>) -> Self {
        Item {
            name: node.name.to_string(),
            attributes: node.attribs.clone(),
            args: node.args,
            // actions: HashMap {},
        }
    }
}
