mod edit;
mod html;
mod parse;
mod query;
mod data;

use std::collections::HashMap;

pub use edit::Editable;
pub use html::Htmlifiable;
pub use parse::parse;
pub use query::{Queryable, Selector};

/// Basic node of dom
#[derive(Debug, Clone)]
pub enum Node {
    Element {
        name: String,
        attrs: HashMap<String, String>,
        children: Vec<Node>,
    },
    Text(String),
    Comment(String),
    Doctype,
}

impl Node {
    /// Check if it is an element node.
    pub fn is_element(&self) -> bool {
        match self {
            Node::Element { .. } => true,
            _ => false,
        }
    }

    /// Try to convert the node into an element.
    pub fn try_into_element(self) -> Result<Element, &'static str> {
        match self {
            Node::Element {
                name,
                attrs,
                children,
            } => Ok(Element {
                name,
                attrs,
                children,
            }),
            _ => Err("not an element"),
        }
    }

    /// Create a new element node.
    pub fn new_element(name: &str, attrs: Vec<(&str, &str)>, children: Vec<Node>) -> Node {
        Node::Element {
            name: name.to_string(),
            attrs: attrs.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            children,
        }
    }
}

/// HTML Element
#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Node>,
}

impl Element {
    /// Create a new element.
    pub fn new(name: &str, attrs: Vec<(&str, &str)>, children: Vec<Node>) -> Self {
        Self {
            name: name.to_string(),
            attrs: attrs.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            children,
        }
    }
}
