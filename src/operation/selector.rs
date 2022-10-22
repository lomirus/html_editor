use std::vec;

use crate::Element;

/// Basic selector. It follows the
/// [CSS selector](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors)
/// standard but not all rules are supported now. Please refer
/// to [`Selector::from`](Selector::from).
#[derive(Debug)]
pub struct Selector(Vec<CompoundSelector>);

/// A sequence of simple selectors that are not separated by a
/// combinator. A compound selector represents a set of
/// simultaneous conditions on a single element.
#[derive(Debug)]
struct CompoundSelector(Vec<SimpleSelector>);

/// A selector with a single component, such as a single
/// id selector or type selector, that's not used in combination
/// with or contains any other selector component or combinator.
#[derive(Debug)]
enum SimpleSelector {
    Class(String),
    Id(String),
    Tag(String),
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
        let element_classes = element
            .attrs
            .iter()
            .find(|(key, _)| key == "class")
            .and_then(|(_, v)| Some(v.split(' ').map(|name| name.trim()).collect::<Vec<_>>()));
        let element_id = element
            .attrs
            .iter()
            .find(|(key, _)| key == "id")
            .and_then(|(_, v)| Some(v));

        self.0.iter().any(|compound_selector| {
            compound_selector
                .0
                .iter()
                .all(|simple_selector| match simple_selector {
                    SimpleSelector::Class(selector_class) => match &element_classes {
                        Some(element_classes) => element_classes
                            .iter()
                            .any(|element_class| element_class == selector_class),
                        None => false,
                    },
                    SimpleSelector::Id(selector_id) => match element_id {
                        Some(element_id) => element_id == selector_id,
                        None => false,
                    },
                    SimpleSelector::Tag(tag) => tag == &element.name,
                })
        })
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
        Selector(
            selector
                .split(',')
                .map(|s| CompoundSelector::from(s))
                .collect(),
        )
    }
}

enum SelectorMark {
    Class,
    Id,
    Tag,
}

impl From<&str> for CompoundSelector {
    fn from(selector: &str) -> Self {
        let selector_chars = selector.trim().chars();
        let mut chars_stack = Vec::<char>::new();
        let mut selector_mark = SelectorMark::Tag;
        let mut simple_selectors = vec![];

        for ch in selector_chars {
            match ch {
                '#' => {
                    if !chars_stack.is_empty() {
                        let string = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        match selector_mark {
                            SelectorMark::Class => simple_selectors.push(SimpleSelector::Class(string)),
                            SelectorMark::Id => simple_selectors.push(SimpleSelector::Id(string)),
                            SelectorMark::Tag => simple_selectors.push(SimpleSelector::Tag(string)),
                        }
                    }
                    
                    selector_mark = SelectorMark::Id;
                }
                '.' => {
                    if !chars_stack.is_empty() {
                        let string = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        match selector_mark {
                            SelectorMark::Class => simple_selectors.push(SimpleSelector::Class(string)),
                            SelectorMark::Id => simple_selectors.push(SimpleSelector::Id(string)),
                            SelectorMark::Tag => simple_selectors.push(SimpleSelector::Tag(string)),
                        }
                    }
                    selector_mark = SelectorMark::Class;
                }
                _ => chars_stack.push(ch),
            }
        }
        let string = String::from_iter(chars_stack);
        match selector_mark {
            SelectorMark::Class => simple_selectors.push(SimpleSelector::Class(string)),
            SelectorMark::Id => simple_selectors.push(SimpleSelector::Id(string)),
            SelectorMark::Tag => simple_selectors.push(SimpleSelector::Tag(string)),
        }
        CompoundSelector(simple_selectors)
    }
}
