use super::Selector;
use crate::{Element, Node};

/// Query the specific element(s) by [`Selector`].
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
    /// let app: &Element = parse(html).unwrap().query(&selector).unwrap();
    /// ```
    fn query(&self, selector: &Selector) -> Option<&Element>;

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
    /// let buttons: Vec<&Element> = parse(html).unwrap().query_all(&selector);
    /// ```
    fn query_all(&self, selector: &Selector) -> Vec<&Element>;

    /// Query the node in `self` as mutable for the given selector.
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
    /// let app: &mut Element = parse(html).unwrap().query_mut(&selector).unwrap();
    /// ```
    fn query_mut(&mut self, selector: &Selector) -> Option<&mut Element>;

    /// Executes a given function for the node in `self` for the given selector.
    ///
    /// ```
    /// use html_editor::{parse, Element, Node};
    /// use html_editor::operation::*;
    ///
    /// let html = r#"
    ///    <!DOCTYPE html>
    ///    <html lang="en">
    ///        <head>
    ///           <meta charset="UTF-8">
    ///           <title>App</title>
    ///        </head>
    ///        <body>
    ///           <input type="text" />
    ///           <input type="text" />
    ///           <input type="text" />
    ///        </body>
    ///    </html>"#;
    ///
    /// // Add a class to all the input elements
    /// let selector: Selector = Selector::from("input");
    /// let mut doc: Vec<Node> = parse(html).unwrap();
    /// doc.execute_for(&selector, |elem| {
    ///    elem.attrs.push(("class".to_string(), "input".to_string()));
    /// });
    /// ```
    fn execute_for(&mut self, selector: &Selector, f: impl FnMut(&mut Element));
}

// We meed this function to allow the trait interface to use `impl FnMut(&mut Element)` instead of `&mut impl FnMut(&mut Element)`
fn nodes_execute_for_internal(
    nodes: &mut Vec<Node>,
    selector: &Selector,
    f: &mut impl FnMut(&mut Element),
) {
    for node in nodes {
        if let Some(element) = node.as_element_mut() {
            // Recursively traverse the descendants nodes
            element_execute_for_internal(element, selector, f);
        }
    }
}

// We meed this function to allow the trait interface to use `impl FnMut(&mut Element)` instead of `&mut impl FnMut(&mut Element)`
fn element_execute_for_internal(
    element: &mut Element,
    selector: &Selector,
    f: &mut impl FnMut(&mut Element),
) {
    if selector.matches(element) {
        f(element);
    }
    nodes_execute_for_internal(&mut element.children, selector, f);
}

impl Queryable for Vec<Node> {
    fn query(&self, selector: &Selector) -> Option<&Element> {
        for node in self {
            if let Some(element) = node.as_element() {
                if let Some(elem) = element.query(selector) {
                    return Some(elem);
                }
            }
        }
        None
    }

    fn query_all(&self, selector: &Selector) -> Vec<&Element> {
        let mut elements = Vec::new();
        for node in self {
            if let Some(element) = node.as_element() {
                // Recursively traverse the descendants nodes
                let sub_elements = element.query_all(selector);
                elements.extend(sub_elements);
            }
        }
        elements
    }

    fn query_mut(&mut self, selector: &Selector) -> Option<&mut Element> {
        for node in self {
            if let Some(element) = node.as_element_mut() {
                if let Some(elem) = element.query_mut(selector) {
                    return Some(elem);
                }
            }
        }
        None
    }

    fn execute_for(&mut self, selector: &Selector, mut f: impl FnMut(&mut Element)) {
        nodes_execute_for_internal(self, selector, &mut f);
    }
}

impl Queryable for Element {
    fn query(&self, selector: &Selector) -> Option<&Element> {
        if selector.matches(self) {
            Some(self)
        } else {
            self.children.query(selector)
        }
    }

    fn query_all(&self, selector: &Selector) -> Vec<&Element> {
        let mut elements = self.children.query_all(selector);
        if selector.matches(self) {
            elements.push(self);
        }
        elements
    }

    fn query_mut(&mut self, selector: &Selector) -> Option<&mut Element> {
        if selector.matches(self) {
            Some(self)
        } else {
            self.children.query_mut(selector)
        }
    }

    fn execute_for(&mut self, selector: &Selector, mut f: impl FnMut(&mut Element)) {
        element_execute_for_internal(self, selector, &mut f);
    }
}

impl Queryable for Node {
    fn query(&self, selector: &Selector) -> Option<&Element> {
        if let Some(element) = self.as_element() {
            element.query(selector)
        } else {
            None
        }
    }

    fn query_all(&self, selector: &Selector) -> Vec<&Element> {
        if let Some(element) = self.as_element() {
            element.query_all(selector)
        } else {
            Vec::new()
        }
    }

    fn query_mut(&mut self, selector: &Selector) -> Option<&mut Element> {
        if let Some(element) = self.as_element_mut() {
            element.query_mut(selector)
        } else {
            None
        }
    }

    fn execute_for(&mut self, selector: &Selector, f: impl FnMut(&mut Element)) {
        if let Some(element) = self.as_element_mut() {
            element.execute_for(selector, f);
        }
    }
}
