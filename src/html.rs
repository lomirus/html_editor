use crate::{data::VOID_TAGS, Element, Node};

/// Used to be converted to html string
pub trait Htmlifiable {
    /// Convert the object to html string.
    ///
    /// ```
    /// use html_editor::{Node, Element, Htmlifiable};
    ///
    /// let node: Node = Node::new_element(
    ///     "script",
    ///     vec![
    ///         ("src", "index.js"),
    ///         ("defer", "")
    ///     ],
    ///     vec![]
    /// );
    /// assert_eq!(node.html(), r#"<script src="index.js" defer></script>"#);
    ///
    /// let nodes: Vec<Node> = vec![node.clone()];
    /// assert_eq!(nodes.html(), r#"<script src="index.js" defer></script>"#);
    ///
    /// let element: Element = node.try_into_element().unwrap();
    /// assert_eq!(element.html(), r#"<script src="index.js" defer></script>"#);
    /// ```
    fn html(&self) -> String;
}

impl Htmlifiable for Element {
    fn html(&self) -> String {
        if self.attrs.len() == 0 {
            return if VOID_TAGS.contains(&self.name.as_str()) {
                format!("<{}>", self.name)
            } else {
                format!("<{}>{}</{}>", self.name, self.children.html(), self.name)
            };
        }
        let attrs = self
            .attrs
            .iter()
            .map(|(k, v)| {
                if v.is_empty() {
                    format!("{}", k)
                } else {
                    format!(r#"{}="{}""#, k, v)
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        if VOID_TAGS.contains(&self.name.as_str()) {
            format!("<{} {}>", self.name, attrs,)
        } else {
            format!(
                "<{} {}>{}</{}>",
                self.name,
                attrs,
                self.children.html(),
                self.name
            )
        }
    }
}

impl Htmlifiable for Node {
    fn html(&self) -> String {
        match self {
            Node::Element { .. } => self.clone().try_into_element().unwrap().html(),
            Node::Text(text) => text.to_string(),
            Node::Comment(comment) => format!("<!--{}-->", comment),
            Node::Doctype => "<!DOCTYPE html>".to_string(),
        }
    }
}

impl Htmlifiable for Vec<Node> {
    fn html(&self) -> String {
        let mut html = String::new();
        for node in self {
            html.push_str(node.html().as_str());
        }
        html
    }
}
