use std::vec;

use super::simple::SimpleSelector;
use super::simple::SelectorMark;

/// A sequence of simple selectors that are not separated by a
/// combinator. A compound selector represents a set of
/// simultaneous conditions on a single element.
#[derive(Debug)]
pub struct CompoundSelector(pub Vec<SimpleSelector>);

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
