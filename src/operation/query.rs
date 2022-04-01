use crate::{Element, Node};
use super::Selector;

/// Query one/all element(s) by [`Selector`].
pub trait Queryable {
    /// Query the node in `self` for the given selector.
    ///
    /// ```
    /// use html_editor::{parse, Element};
    /// use html_editor::operation::*;
    ///
    /// let html = r#"
    ///     <!DOCTYPE html>
    ///     <html lang="en">
    ///     <head>
    ///         <meta charset="UTF-8">
    ///         <title>App</title>
    ///     </head>
    ///     <body>
    ///         <div id="app"></div>
    ///     </body>
    ///     </html>"#;
    /// 
    /// let selector: Selector = Selector::from("#app");
    /// let app: Element = parse(html).unwrap().query(&selector).unwrap();
    /// ```
    fn query(&self, selector: &Selector) -> Option<Element>;

    /// Query all the nodes in `self` for the given selector.
    /// 
    /// ```
    /// use html_editor::{parse, Element};
    /// use html_editor::operation::*;
    ///
    /// let html = r#"
    ///     <!DOCTYPE html>
    ///     <html lang="en">
    ///     <head>
    ///         <meta charset="UTF-8">
    ///         <title>App</title>
    ///     </head>
    ///     <body>
    ///         <span class="btn">Ok</span>
    ///         <span class="btn">Cancel</span>
    ///         <span class="btn">Remind Me Later</span>
    ///     </body>
    ///     </html>"#;
    /// 
    /// let selector: Selector = Selector::from(".btn");
    /// let app: Vec<Element> = parse(html).unwrap().query_all(&selector);
    /// ```
    fn query_all(&self, selector: &Selector) -> Vec<Element>;
}

impl Queryable for Vec<Node> {
    fn query(&self, selector: &Selector) -> Option<Element> {
        for node in self {
            if node.is_element() {
                let element = node.clone().into_element();

                if selector.matches(&element) {
                    return Some(element);
                } else {
                    if let Some(elem) = element.query(selector) {
                        return Some(elem);
                    }
                }
            }
        }
        None
    }

    fn query_all(&self, selector: &Selector) -> Vec<Element> {
        let mut elements = Vec::new();
        for node in self {
            if node.is_element() {
                let element = node.clone().into_element();
                // Recursively traverse the descendants nodes
                let sub_elements = element.query_all(selector);
                elements.extend(sub_elements);
                // Check if this element matches. If so, push it to the `elements`
                if selector.matches(&element) {
                    elements.push(element);
                }
            }
        }
        elements
    }
}

impl Queryable for Element {
    fn query(&self, selector: &Selector) -> Option<Element> {
        self.children.query(selector)
    }
    
    fn query_all(&self, selector: &Selector) -> Vec<Element> {
        self.children.query_all(selector)
    }
}
