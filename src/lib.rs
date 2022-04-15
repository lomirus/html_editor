//! `html_editor` is a simple html parser and editor.
//!
//! Quick Start:
//! ```
//! use html_editor::operation::*;
//! use html_editor::{parse, Node};
//!
//! // You can create DOM nodes by parsing html string.
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
mod parse;

pub mod operation;

pub use parse::parse;
pub use parse::try_parse;

/// Doctype of Html or Xml
#[derive(Clone, Debug)]
pub enum Doctype {
    Html,
    Xml { version: String, encoding: String },
}

/// Node of DOM
#[derive(Debug, Clone)]
pub enum Node {
    Element {
        name: String,
        attrs: Vec<(String, String)>,
        children: Vec<Node>,
    },
    Text(String),
    Comment(String),
    Doctype(Doctype),
}

impl Node {
    /// Check if it is an element node.
    ///
    /// ```
    /// use html_editor::Node;
    ///
    /// assert_eq!(Node::new_element("div", vec![("id", "app")], vec![]).is_element(), true);
    /// assert_eq!(Node::Text("Lorem Ipsum".to_string()).is_element(), false);
    /// ```
    pub fn is_element(&self) -> bool {
        match self {
            Node::Element { .. } => true,
            _ => false,
        }
    }

    /// Convert the node into an element.
    ///
    /// Warning: The program will panic if it fails to convert.
    /// So take care to use this method unless you are sure.
    ///
    /// Example:
    /// ```
    /// use html_editor::{Node, Element};
    ///
    /// let a: Node = Node::new_element("div", vec![("id", "app")], vec![]);
    /// let a: Element = a.into_element();
    ///
    /// let b: Node = Node::Text("hello".to_string());
    /// // The next line will panic at 'Text("hello") is not an element'
    /// // let b: Element = a.into_element();
    /// ```
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
    ///
    /// ```
    /// use html_editor::Node;
    ///
    /// let node: Node = Node::new_element(
    ///     "h1",
    ///     vec![("class", "title")],
    ///     vec![
    ///         Node::Text("Hello, world!".to_string()),
    ///     ]
    /// );
    /// ```
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
