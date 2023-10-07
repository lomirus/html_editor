use super::Selector;
use crate::{Element, Node};

/// Insert and remove elements by [`Selector`], and trim the DOM.
pub trait Editable {
    /// Remove all empty text nodes from `self`.
    ///
    /// ```
    /// use html_editor::parse;
    /// use html_editor::operation::*;
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
    fn trim(&mut self) -> &mut Self;

    /// Insert `node` as the last child to all elements that matches the `selector`.
    ///
    /// ```
    /// use html_editor::{parse, Node};
    /// use html_editor::operation::*;
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
    ///
    /// assert_eq!(html, r#"<div><span>Ok</span><span>Cancel</span></div>"#)
    /// ```
    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self;

    /// Remove all elements that matches the `selector`.
    ///
    /// ```
    /// use html_editor::parse;
    /// use html_editor::operation::*;
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
    ///
    /// assert_eq!(html, r#"
    /// <div>
    ///     <div class="recommend"></div>
    ///     <div class="results"></div>
    ///    
    /// </div>"#)
    /// ```
    fn remove_by(&mut self, selector: &Selector) -> &mut Self;

    /// Replace all elements that matches the `selector` with new nodes.
    ///
    /// ```
    /// use html_editor::{parse, Node, operation::*};
    ///
    /// let html = r#"
    /// <div>
    ///     <p>Hello</p>
    /// </div>"#;
    ///
    /// let selector = Selector::from("p");
    /// let html = parse(html)
    ///     .unwrap()
    ///     .replace_with(&selector, |p| {
    ///         let new_text = format!("{} World!", p.children[0].html());
    ///         Node::Comment(new_text)
    ///     })
    ///     .html();
    ///
    /// assert_eq!(html, r#"
    /// <div>
    ///     <!--Hello World!-->
    /// </div>"#)
    /// ```
    fn replace_with(&mut self, selector: &Selector, f: fn(el: &Element) -> Node) -> &mut Self;
}

impl Editable for Vec<Node> {
    fn trim(&mut self) -> &mut Self {
        self.retain(|node| match node {
            Node::Doctype(..) => true,
            Node::Comment(..) => false,
            Node::Text(text) => !text.trim().is_empty(),
            Node::Element { .. } => true,
        });
        for node in self.iter_mut() {
            if let Node::Element(el) = node {
                el.children.trim();
            }
        }
        self
    }

    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self {
        for node in self.iter_mut() {
            if let Node::Element(el) = node {
                el.children.insert_to(selector, target.clone());
                if selector.matches(&Element {
                    name: el.name.clone(),
                    attrs: el.attrs.clone(),
                    children: vec![],
                }) {
                    el.children.push(target.clone());
                }
            }
        }
        self
    }

    fn remove_by(&mut self, selector: &Selector) -> &mut Self {
        self.retain(|node| {
            if let Node::Element(el) = node {
                let element = Element {
                    name: el.name.clone(),
                    attrs: el.attrs.clone(),
                    children: vec![],
                };
                return !selector.matches(&element);
            }
            true
        });
        for node in self.iter_mut() {
            if let Node::Element(el) = node {
                el.remove_by(selector);
            }
        }
        self
    }

    fn replace_with(&mut self, selector: &Selector, f: fn(el: &Element) -> Node) -> &mut Self {
        for node in self.iter_mut() {
            if let Node::Element(ref mut el) = node {
                if selector.matches(el) {
                    *node = f(el);
                } else {
                    el.replace_with(selector, f);
                }
            }
        }
        self
    }
}

impl Editable for Element {
    fn trim(&mut self) -> &mut Self {
        self.children.trim();
        self
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

    fn replace_with(&mut self, selector: &Selector, f: fn(el: &Element) -> Node) -> &mut Self {
        self.children.replace_with(selector, f);
        self
    }
}
