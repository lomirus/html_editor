//! `html_editor` is a simple html parser and editor.
//!
//! Example:
//! ```
//! use html_editor::prelude::*;
//! use html_editor::{parse, Node, Selector};
//!
//! // You can create dom nodes by parsing html string.
//! let html = r#"
//!     <!doctype html>
//!     <html>
//!         <head>
//!         </head>
//!         <body>
//!         </body>
//!     </html>
//! "#;
//! let mut dom = parse(html).unwrap();
//!
//! // Or you can create a node by some built-in methods like below.
//! let app: Node = Node::new_element("div", vec![("id", "app")], vec![]);
//!
//! // Here shows how to edit the nodes and turn it back to html.
//! let html = dom
//!     .remove_by(&Selector::from("head"))
//!     .insert_to(&Selector::from("body"), app)
//!     .trim()
//!     .html();
//!
//! assert_eq!(
//!     html,
//!     r#"<!DOCTYPE html><html><body><div id="app"></div></body></html>"#
//! );
//! ```

mod data;
mod edit;
mod html;
mod parse;
mod query;

pub mod prelude;

pub use parse::parse;
pub use query::Selector;

/// Basic node of dom
#[derive(Debug, Clone)]
pub enum Node {
    Element {
        name: String,
        attrs: Vec<(String, String)>,
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

    /// Convert the node into an element. 
    /// 
    /// Note: The program will panic if it fails to convert.
    /// So take care to use this method unless you are sure.
    pub fn into_element(self) -> Element {
        match self {
            Node::Element {
                name,
                attrs,
                children,
            } => Element {
                name,
                attrs,
                children,
            },
            _ => panic!("{:?} is not an element", self),
        }
    }

    /// Create a new element node.
    pub fn new_element(name: &str, attrs: Vec<(&str, &str)>, children: Vec<Node>) -> Node {
        Node::Element {
            name: name.to_string(),
            attrs: attrs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            children,
        }
    }
}

/// HTML Element
#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<Node>,
}

impl Element {
    /// Create a new element.
    pub fn new(name: &str, attrs: Vec<(&str, &str)>, children: Vec<Node>) -> Self {
        Self {
            name: name.to_string(),
            attrs: attrs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            children,
        }
    }
}
