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

pub use parse::{parse, SourceLocation, HTMLParseError, InnerHTMLParseError};
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
    Element(Element),
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
        matches!(self, Node::Element { .. })
    }

    #[deprecated(note = "Please use `is_element` instead")]
    pub fn into_element(self) -> Element {
        match self {
            Node::Element(element) => element,
            _ => panic!("{:?} is not an element", self),
        }
    }

    /// Convert the node into an element.
    ///
    /// Returns `None` if the node is not an element.
    ///
    /// Example:
    /// ```
    /// use html_editor::{Node, Element};
    ///
    /// let a: Node = Node::new_element("div", vec![("id", "app")], vec![]);
    /// assert!(a.as_element().is_some());
    ///
    /// let b: Node = Node::Text("hello".to_string());
    /// assert!(b.as_element().is_none());
    /// ```
    pub fn as_element(&self) -> Option<&Element> {
        match self {
            Node::Element(element) => Some(element),
            _ => None,
        }
    }

    /// Convert the node into a mutable element.
    ///
    /// Returns `None` if the node is not an element.
    ///
    /// Example:
    /// ```
    /// use html_editor::{Node, Element};
    ///
    /// let mut a: Node = Node::new_element("div", vec![("id", "app")], vec![]);
    /// assert!(a.as_element_mut().is_some());
    ///
    /// let mut b: Node = Node::Text("hello".to_string());
    /// assert!(b.as_element_mut().is_none());
    /// ```
    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Node::Element(element) => Some(element),
            _ => None,
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
        Element {
            name: name.to_string(),
            attrs: attrs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            children,
        }
        .into_node()
    }
}

/// HTML Element
#[derive(Debug, Clone)]
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

impl Element {
    pub fn into_node(self) -> Node {
        Node::Element(self)
    }
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Node::Element(element)
    }
}
