use crate::{Element, Node};

/// Simple query selector
#[derive(Debug)]
pub struct Selector {
    class: String,
    id: String,
    tag: String,
}

enum SelectorPos {
    Class,
    Id,
    Tag,
}

impl Selector {
    /// The `selector` only supports type selector, ID selector and class selector.
    ///
    /// For example, `div#app`, `span` would be ok, but `.container > div`,
    /// `#app *` would get unexpected results.
    /// 
    /// ```
    /// use html_editor::Selector;
    /// 
    /// // Ok: Simple tag, class and ID selectors.
    /// let selector = Selector::from("span");
    /// let selector = Selector::from(".class");
    /// let selector = Selector::from("#id");
    /// 
    /// // Ok: Mixed selector
    /// let selector = Selector::from("div#app");
    /// let selector = Selector::from("span.info#first");
    /// 
    /// // Disallowed
    /// let selector = Selector::from("div span");
    /// let selector = Selector::from("a[target=_blank]");
    /// ```
    pub fn from(selector: &str) -> Self {
        let selector_chars = selector.trim().chars();
        let mut chars_stack = Vec::<char>::new();
        let mut selector_pos = SelectorPos::Tag;
        let mut selector = Selector {
            class: String::new(),
            id: String::new(),
            tag: String::new(),
        };

        for ch in selector_chars {
            match ch {
                '#' => {
                    let string = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    match selector_pos {
                        SelectorPos::Class => selector.class = string,
                        SelectorPos::Id => selector.id = string,
                        SelectorPos::Tag => selector.tag = string,
                    }
                    selector_pos = SelectorPos::Id;
                }
                '.' => {
                    let string = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    match selector_pos {
                        SelectorPos::Class => selector.class = string,
                        SelectorPos::Id => selector.id = string,
                        SelectorPos::Tag => selector.tag = string,
                    }
                    selector_pos = SelectorPos::Class;
                }
                _ => chars_stack.push(ch),
            }
        }
        let string = String::from_iter(chars_stack);
        match selector_pos {
            SelectorPos::Class => selector.class = string,
            SelectorPos::Id => selector.id = string,
            SelectorPos::Tag => selector.tag = string,
        }
        selector
    }

    /// Check if the `element` matches the `selector`.
    /// 
    /// ```
    /// use html_editor::{Node, Element, Selector};
    /// use html_editor::prelude::*;
    /// 
    /// let element: Element = Element::new(
    ///     "div",
    ///     vec![("id", "app")],
    ///     vec![Node::Text("Hello World!".to_string())],
    /// );
    /// 
    /// let selector = Selector::from("div#app");
    /// 
    /// assert_eq!(selector.matches(&element), true);
    /// ```
    pub fn matches(&self, element: &Element) -> bool {
        let mut matches = true;

        if self.tag != "" && element.name != self.tag {
            matches = false;
        }

        if self.class != "" {
            let element_class = element.attrs.iter().find(|(key,_)|key == "class");
            match element_class {
                Some(class) => {
                    if self.class != class.1 {
                        matches = false;
                    }
                }
                None => {
                    if self.class != "" {
                        matches = false;
                    }
                }
            }
        }

        if self.id != "" {
            let element_id = element.attrs.iter().find(|(key,_)|key == "id");
            match element_id {
                Some(id) => {
                    if self.id != id.1 {
                        matches = false;
                    }
                }
                None => {
                    if self.id != "" {
                        matches = false;
                    }
                }
            }
        }

        matches
    }
}

/// Used to `query()` or `query_all()` with `Selector`
pub trait Queryable {
    /// Query the node in `self` for the given selector.
    ///
    /// ```
    /// use html_editor::{parse, Element, Selector};
    /// use html_editor::prelude::*;
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
    /// use html_editor::{parse, Element, Selector};
    /// use html_editor::prelude::*;
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
                let element = node.clone().try_into_element().unwrap();

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
                let element = node.clone().try_into_element().unwrap();
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
