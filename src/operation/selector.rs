use crate::Element;

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
    /// Check if the `element` matches the `selector`.
    ///
    /// ```
    /// use html_editor::{Node, Element};
    /// use html_editor::operation::*;
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

        if !self.tag.is_empty() && element.name != self.tag {
            matches = false;
        }

        if !self.class.is_empty() {
            let element_class = element.attrs.iter().find(|(key, _)| key == "class");
            match element_class {
                Some(class) => {
                    let class = &class.1;
                    if !class
                        .split(' ')
                        .map(|name| name.trim().to_string())
                        .any(|x| x == self.class)
                    {
                        matches = false;
                    }
                }
                None => matches = false,
            }
        }

        if !self.id.is_empty() {
            let element_id = element.attrs.iter().find(|(key, _)| key == "id");
            match element_id {
                Some(id) => {
                    if self.id != id.1 {
                        matches = false;
                    }
                }
                None => {
                    if !self.id.is_empty() {
                        matches = false;
                    }
                }
            }
        }

        matches
    }
}

impl From<&str> for Selector {
    /// The `selector` only supports type selector, ID selector and class selector.
    ///
    /// For example, `div#app`, `span` would be ok, but `.container > div`,
    /// `#app *` would get unexpected results.
    ///
    /// ```
    /// use html_editor::operation::Selector;
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
    fn from(selector: &str) -> Self {
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
}
