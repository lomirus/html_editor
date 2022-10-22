mod compound;
mod simple;

use crate::Element;

use self::{compound::CompoundSelector, simple::SimpleSelector};

/// Basic selector. It follows the
/// [CSS selector](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors)
/// standard but not all rules are supported now. Please refer
/// to [`Selector::from`](Selector::from).
#[derive(Debug)]
pub struct Selector(Vec<CompoundSelector>);

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
            .map(|(_, v)| v.split(' ').map(|name| name.trim()).collect::<Vec<_>>());
        let element_id = element
            .attrs
            .iter()
            .find(|(key, _)| key == "id")
            .map(|(_, v)| v);

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
        Selector(selector.split(',').map(CompoundSelector::from).collect())
    }
}
