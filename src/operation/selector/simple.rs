/// A selector with a single component, such as a single
/// id selector or type selector, that's not used in combination
/// with or contains any other selector component or combinator.
#[derive(Debug)]
pub enum SimpleSelector {
    Class(String),
    Id(String),
    Tag(String),
}
