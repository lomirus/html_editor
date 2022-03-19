use crate::{Element, Node, Selector};

/// Used to insert or remove elements by `Selector`, and trim the dom.
pub trait Editable {
    /// Remove all empty text nodes from `self`.
    /// 
    /// ```
    /// use html_editor::parse;
    /// use html_editor::prelude::*;
    ///
    /// let html = r#"
    ///     <!DOCTYPE html>
    ///     <html>
    ///         <head></head>
    ///         <body></body>
    ///     </html>"#;
    /// 
    /// let html = parse(html).unwrap().trim().html();
    /// assert_eq!(html, r#"<!DOCTYPE html><html><head></head><body></body></html>"#)
    /// ```
    fn trim(self) -> Self;

    /// Insert `node` as the last child to all elements that matches the `selector`.
    /// 
    /// ```
    /// use html_editor::{parse, Node, Selector};
    /// use html_editor::prelude::*;
    ///
    /// let html = r#"<div><span>Ok</span></div>"#;
    /// 
    /// let selector = Selector::from("div");
    /// let html = parse(html)
    ///     .unwrap()
    ///     .insert_to(&selector, Node::new_element(
    ///         "span",
    ///         vec![],
    ///         vec![Node::Text("Cancel".to_string())]
    ///     ))
    ///     .html();
    /// assert_eq!(html, r#"<div><span>Ok</span><span>Cancel</span></div>"#)
    /// ```
    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self;
    
    /// Remove all elements that matches the `selector`.
    /// 
    /// ```
    /// use html_editor::{parse, Selector};
    /// use html_editor::prelude::*;
    ///
    /// let html = r#"
    /// <div>
    ///     <div class="recommend"></div>
    ///     <div class="results"></div>
    ///     <div class="ad"></div>
    /// </div>"#;
    /// 
    /// let selector = Selector::from(".ad");
    /// let html = parse(html).unwrap().remove_by(&selector).html();
    /// assert_eq!(html, r#"
    /// <div>
    ///     <div class="recommend"></div>
    ///     <div class="results"></div>
    ///    
    /// </div>"#)
    /// ```
    fn remove_by(&mut self, selector: &Selector) -> &mut Self;
}

impl Editable for Vec<Node> {
    fn trim(self) -> Self {
        let mut nodes: Vec<Node> = Vec::new();
        for node in self {
            match node {
                Node::Element {
                    name,
                    attrs,
                    children,
                } => nodes.push(Node::Element {
                    name,
                    attrs,
                    children: children.trim(),
                }),
                Node::Text(text) => {
                    if text.trim() != "" {
                        nodes.push(Node::Text(text));
                    }
                }
                Node::Comment(_) => {}
                Node::Doctype => nodes.push(node),
            }
        }
        nodes
    }
    
    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self {
        for node in self.iter_mut() {
            if let Node::Element {
                name,
                attrs,
                children,
            } = node
            {
                children.insert_to(selector, target.clone());
                if selector.matches(&Element {
                    name: name.clone(),
                    attrs: attrs.clone(),
                    children: vec![],
                }) {
                    children.push(target.clone());
                }
            }
        }
        self
    }

    fn remove_by(&mut self, selector: &Selector) -> &mut Self {
        self.retain(|node| {
            if let Node::Element { name, attrs, .. } = node {
                let element = Element {
                    name: name.clone(),
                    attrs: attrs.clone(),
                    children: vec![],
                };
                return !selector.matches(&element);
            }
            true
        });
        for node in self.iter_mut() {
            if let Node::Element { children, .. } = node {
                children.remove_by(selector);
            }
        }
        self
    }
}

impl Editable for Element {
    fn trim(self) -> Self {
        Element {
            name: self.name,
            attrs: self.attrs,
            children: self.children.trim(),
        }
    }

    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self {
        self.children.insert_to(selector, target.clone());
        if selector.matches(self) {
            self.children.push(target);
        }
        self
    }

    fn remove_by(&mut self, selector: &Selector) -> &mut Self {
        self.children.remove_by(selector);
        self
    }
}
