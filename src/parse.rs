// TODO:
//
// Maybe we could have more elegant and concise code?
// I will try to figure out a better way to parse when
// am availible. Busy now for me :(
//
// If you have any idea, feel free to post an issue.

/// Represents the offset in the source file provided to the [parse] function.
/// The number represents a number of *characters* into the source string.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SourceLocation(pub usize);

impl std::ops::Add<isize> for SourceLocation {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        SourceLocation(self.0.checked_add_signed(rhs).expect("SourceLocation out of bounds"))
    }
}

impl std::ops::Sub<isize> for SourceLocation {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self::Output {
        SourceLocation(self.0.checked_add_signed(-rhs).expect("SourceLocation out of bounds"))
    }
}

mod attrs;
mod token;

use crate::{data::VOID_TAGS, Element, Node};
use token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct HTMLParseError {
    pub source_location: SourceLocation,
    pub inner: InnerHTMLParseError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InnerHTMLParseError {
    MismatchedTags { start_tag: String, start_location: SourceLocation, end_tag: String },
    UnopenedTag { tag: String },
    UnclosedTag { tag: String },
    InvalidTag { tag: String, reason: &'static str },
}

impl InnerHTMLParseError {
    fn with_location(self, source_location: SourceLocation) -> HTMLParseError {
        HTMLParseError { source_location, inner: self }
    }
}

fn html_to_stack(html: &str) -> Result<Vec<(Token, SourceLocation)>, HTMLParseError> {
    let mut chars_stack = Vec::<char>::new();
    let mut token_stack = Vec::<(Token, SourceLocation)>::new();
    let mut in_quotes: Option<char> = None;
    // More precisely: is in angle brackets
    let mut in_brackets = false;
    let mut in_comment = false;
    let mut in_script = false;
    let mut in_style = false;
    for (i, ch) in html.chars().enumerate() {
        let loc = SourceLocation(i);
        let next_loc = loc + 1;

        if let Some(quote) = in_quotes {
            if ch == quote {
                let previous_char = *chars_stack
                    .last()
                    .expect("cannot get the last char in chars stack"); // This should be unreachable as in_quotes should not be enterable without any chars in the stack
                if previous_char != '\\' {
                    in_quotes = None;
                }
            }
            chars_stack.push(ch);
        } else if in_comment {
            chars_stack.push(ch);

            if ends_with(&chars_stack, &['-', '-', '>']) {
                let comment_len = chars_stack.len();
                let comment = String::from_iter(chars_stack);
                chars_stack = Vec::new();
                let start_loc = next_loc - comment_len as isize;
                token_stack.push((Token::from_comment(comment), start_loc));
                in_comment = false;
                in_brackets = false;
            }
        } else if in_script {
            chars_stack.push(ch);
            let len = chars_stack.len();

            if ends_with(&chars_stack, &['<', '/', 's', 'c', 'r', 'i', 'p', 't', '>']) {
                let script_len = chars_stack.len() - 9;
                let script = String::from_iter(chars_stack[..len - 9].to_vec());
                chars_stack = Vec::new();
                let script_start_loc = next_loc - 9 - script_len as isize;
                let script_end_tag_start_loc = next_loc - 9;
                token_stack.push((Token::Text(script), script_start_loc));
                token_stack.push((Token::End("script".to_string()), script_end_tag_start_loc));
                in_script = false;
            }
        } else if in_style {
            chars_stack.push(ch);
            let len = chars_stack.len();

            if ends_with(&chars_stack, &['<', '/', 's', 't', 'y', 'l', 'e', '>']) {
                let style_len = chars_stack.len() - 8;
                let style = String::from_iter(chars_stack[..len - 8].to_vec());
                chars_stack = Vec::new();
                let style_start_loc = next_loc - 8 - style_len as isize;
                let style_end_tag_start_loc = next_loc - 8;
                token_stack.push((Token::Text(style), style_start_loc));
                token_stack.push((Token::End("style".to_string()), style_end_tag_start_loc));
                in_style = false;
            }
        } else {
            match ch {
                '<' => {
                    in_brackets = true;
                    // In case of pushing empty text tokens to the stack
                    if !chars_stack.is_empty() {
                        // Turn the chars in `chars_stack` into `String`
                        // and clean the chars stack.
                        let txt_len = chars_stack.len();
                        let txt_text = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        let text_start_loc = next_loc - txt_len as isize;
                        // Push the text we just got to the token stack.
                        token_stack.push((Token::Text(txt_text), text_start_loc));
                    }
                    chars_stack.push(ch);
                }
                '>' => {
                    in_brackets = false;
                    chars_stack.push(ch);
                    // Turn the chars in `chars_stack` in to `String`
                    // and clean the chars stack.
                    let tag_text_len = chars_stack.len();
                    let tag_text = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    // Push the tag with the text we just got to the token stack.
                    let start_loc = next_loc - tag_text_len as isize;
                    let tag = Token::from(tag_text.clone()).map_err(|e| e.with_location(start_loc))?;
                    token_stack.push((tag.clone(), start_loc));
                    // Handle special tags
                    if let Token::Start(tag_name, _) = tag {
                        let tag_name = tag_name.as_str();
                        match tag_name {
                            "script" => in_script = true,
                            "style" => in_style = true,
                            _ => {}
                        }
                    }
                }
                '-' => {
                    chars_stack.push(ch);
                    if chars_stack == ['<', '!', '-', '-'] {
                        in_comment = true;
                    }
                }
                _ => {
                    if in_brackets {
                        match ch {
                            '\'' => in_quotes = Some('\''),
                            '\"' => in_quotes = Some('\"'),
                            _ => {}
                        }
                    }
                    chars_stack.push(ch)
                }
            }
        }
    }
    if !chars_stack.is_empty() {
        let text_len = chars_stack.len();
        let text = String::from_iter(chars_stack);
        let text_start_loc = SourceLocation(html.chars().count() - text_len);
        token_stack.push((Token::Text(text), text_start_loc));
    }
    Ok(token_stack)
}

fn stack_to_dom(token_stack: Vec<(Token, SourceLocation)>) -> Result<Vec<Node>, HTMLParseError> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut start_tags_stack: Vec<(Token, SourceLocation)> = Vec::new();
    let mut start_tag_index = 0;
    for (i, (token, location)) in token_stack.iter().enumerate() {
        let location = *location;
        match token {
            Token::Start(tag, attrs) => {
                let is_void_tag = VOID_TAGS.contains(&tag.as_str());
                if start_tags_stack.is_empty() {
                    if is_void_tag {
                        nodes.push(
                            Element {
                                name: tag.clone(),
                                attrs: attrs.clone(),
                                children: Vec::new(),
                            }
                            .into_node(),
                        );
                    } else {
                        start_tag_index = i;
                        start_tags_stack.push((Token::Start(tag.clone(), attrs.clone()), location));
                    }
                } else if is_void_tag {
                    // You do not need to push the void tag to the stack
                    // like above, because it must be inside the the
                    // element of the first start tag, and this element
                    // will then be pushed to the stack recursively.
                } else {
                    start_tags_stack.push((Token::Start(tag.clone(), attrs.clone()), location));
                }
            }
            Token::End(tag) => {
                let (start_tag, start_location) = match start_tags_stack.pop() {
                    Some((token, location)) => (token.into_element(), location),
                    None => return Err(
                        InnerHTMLParseError::UnopenedTag { tag: tag.to_string() }
                            .with_location(location)
                    ),
                };
                if tag != &start_tag.name {
                    return Err(
                        InnerHTMLParseError::MismatchedTags { start_tag: start_tag.name, start_location, end_tag: tag.to_string() }
                            .with_location(location)
                    );
                }
                if start_tags_stack.is_empty() {
                    nodes.push(
                        Element {
                            name: start_tag.name,
                            attrs: start_tag.attrs,
                            children: stack_to_dom(token_stack[start_tag_index + 1..i].to_vec())?,
                        }
                        .into_node(),
                    )
                }
            }
            _ => {
                if start_tags_stack.is_empty() {
                    nodes.push(token.node());
                }
            }
        }
    }

    match start_tags_stack.pop() {
        Some((token, location)) => {
            let start_tag_name = token.into_element().name;
            Err(
                InnerHTMLParseError::UnclosedTag { tag: start_tag_name.to_string() }
                    .with_location(location)
            )
        }
        None => Ok(nodes),
    }
}

fn try_stack_to_dom(token_stack: Vec<(Token, SourceLocation)>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut start_tags_stack: Vec<Token> = Vec::new();
    let mut start_tag_index = 0;

    for (i, (token, location)) in token_stack.iter().enumerate() {
        let _location = *location;
        match token {
            Token::Start(tag, attrs) => {
                let is_void_tag = VOID_TAGS.contains(&tag.as_str());
                if start_tags_stack.is_empty() {
                    if is_void_tag {
                        nodes.push(
                            Element {
                                name: tag.clone(),
                                attrs: attrs.clone(),
                                children: Vec::new(),
                            }
                            .into_node(),
                        );
                    } else {
                        start_tag_index = i;
                        start_tags_stack.push(Token::Start(tag.clone(), attrs.clone()));
                    }
                } else if is_void_tag {
                    // You do not need to push the void tag to the stack
                    // like above, because it must be inside the the
                    // element of the first start tag, and this element
                    // will then be pushed to the stack recursively.
                } else {
                    start_tags_stack.push(Token::Start(tag.clone(), attrs.clone()));
                }
            }
            Token::End(tag) => {
                let start_tag = match start_tags_stack.pop() {
                    Some(token) => token.into_element(),
                    // It means the end tag is redundant, so we will omit
                    // it and just start the next loop.
                    None => continue,
                };

                if tag != &start_tag.name {
                    // The tags do not match, so let's put it back to
                    // pretend we never come here and then continue
                    // the next loop.
                    start_tags_stack.push(Token::Start(start_tag.name, start_tag.attrs));
                    continue;
                }

                if start_tags_stack.is_empty() {
                    nodes.push(
                        Element {
                            name: start_tag.name,
                            attrs: start_tag.attrs,
                            children: try_stack_to_dom(
                                token_stack[start_tag_index + 1..i].to_vec(),
                            ),
                        }
                        .into_node(),
                    )
                }
            }
            _ => {
                if start_tags_stack.is_empty() {
                    nodes.push(token.node());
                }
            }
        }
    }

    while let Some(token) = start_tags_stack.pop() {
        let node = match token {
            Token::Start(name, attrs) => Element {
                name,
                attrs,
                children: try_stack_to_dom(token_stack[start_tag_index + 1..].to_vec()),
            }
            .into_node(),
            _ => unreachable!(),
        };
        nodes = vec![node];
    }
    nodes
}

/// Parse the html string and return a `Vector` of `Node`.
///
/// Example:
/// ```
/// use html_editor::parse;
///
/// // Parse a segment.
/// let segment = parse(r#"<p class="content">Hello, world!</p>"#);
/// println!("{:#?}", segment);
///
/// // Or you can parse a whole html file.
/// let document = parse("<!doctype html><html><head></head><body></body></html>");
/// println!("{:#?}", document);
/// ```
///
/// Output:
/// ```log
/// [
///     Element {
///         name: "p",
///         attrs: {
///             "class": "content",
///         },
///         children: [
///             Text(
///                 "Hello, world!",
///             ),
///         ],
///     },
/// ]
/// [
///     Doctype(Html),
///     Element {
///         name: "html",
///         attrs: {},
///         children: [
///             Element {
///                 name: "head",
///                 attrs: {},
///                 children: [],
///             },
///             Element {
///                 name: "body",
///                 attrs: {},
///                 children: [],
///             },
///         ],
///     },
/// ]
/// ```
pub fn parse(html: &str) -> Result<Vec<Node>, HTMLParseError> {
    let stack = html_to_stack(html)?;

    stack_to_dom(stack)
}

/// Alternative for [`parse()`](parse) with fault tolerance
/// feature.
///
/// Whatever the input is, it will try to return a vector of nodes
/// without errors. It can parse some illegal html code like `<div><a>Ipsum`
/// or `<div>Ipsum</a>`.
///
/// But we still suggest you to use [`parse()`](parse) unless neccessary for better
/// error handling.
///
/// ```
/// use html_editor::{try_parse, operation::Htmlifiable};
///
/// let result: String = try_parse("<div><a>Ipsum</div>").html();
/// assert_eq!(result, "<div><a>Ipsum</a></div>");
/// ```
pub fn try_parse(html: &str) -> Vec<Node> {
    let stack = html_to_stack(html).unwrap_or_default();

    try_stack_to_dom(stack)
}

// Use `&[char]` instead of `&str` to improve performance.
fn ends_with(chars: &Vec<char>, end: &[char]) -> bool {
    let chars_len = chars.len();
    let end_len = end.len();
    for i in 1..(end_len + 1) {
        if chars[chars_len - i] != end[end.len() - i] {
            return false;
        }
    }
    true
}
