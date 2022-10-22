use std::vec;

use super::simple::SimpleSelector;

/// A sequence of simple selectors that are not separated by a
/// combinator. A compound selector represents a set of
/// simultaneous conditions on a single element.
#[derive(Debug)]
pub struct CompoundSelector(pub Vec<SimpleSelector>);

impl From<&str> for CompoundSelector {
    fn from(selector: &str) -> Self {
        let mut simple_selectors = vec![];

        let mut start = 0;
        let mut end;

        while start < selector.len() {
            end = selector[start + 1..]
                .find(|c| c == '.' || c == '#')
                .map(|n| n + start + 1)
                .unwrap_or(selector.len());
            let start_char = selector.chars().nth(start).unwrap();

            use SimpleSelector::*;
            simple_selectors.push(match start_char {
                '.' => Class((&selector[start + 1..end]).to_string()),
                '#' => Id((&selector[start + 1..end]).to_string()),
                _ => Tag((&selector[start..end]).to_string()),
            });

            start = end;
        }

        CompoundSelector(simple_selectors)
    }
}
