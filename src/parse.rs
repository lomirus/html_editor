mod attrs;
mod token;

use crate::{data::VOID_TAGS, Node};
use token::Token;

fn html_to_stack(html: &str) -> Vec<Token> {
    let mut chars_stack = Vec::<char>::new();
    let mut token_stack = Vec::<Token>::new();
    let mut in_quotes: Option<char> = None;
    // More precisely: is in angle brackets
    let mut in_brackets = false;
    let mut in_comment = false;
    for ch in html.chars() {
        if let Some(quote) = in_quotes {
            if ch == quote {
                let last_char = chars_stack
                    .last()
                    .expect("cannot get the last char in chars stack")
                    .clone();
                if last_char != '\\' {
                    in_quotes = None;
                }
            }
            chars_stack.push(ch);
        } else if in_comment {
            chars_stack.push(ch);
            let len = chars_stack.len();
            if chars_stack[len - 3..len] == ['-', '-', '>'] {
                let comment = String::from_iter(chars_stack);
                chars_stack = Vec::new();
                let tag = Token::from_comment(comment);
                token_stack.push(tag);
                in_comment = false;
                in_brackets = false;
            }
        } else {
            match ch {
                '<' => {
                    in_brackets = true;
                    // In case of pushing empty text tokens to the stack
                    if chars_stack.len() != 0 {
                        // Turn the chars in `chars_stack` in to `String`
                        // and clean the chars stack.
                        let txt_text = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        // Push the text we just got to the token stack.
                        token_stack.push(Token::Text(txt_text));
                    }
                    chars_stack.push(ch);
                }
                '>' => {
                    in_brackets = false;
                    chars_stack.push(ch);
                    // Turn the chars in `chars_stack` in to `String`
                    // and clean the chars stack.
                    let tag_text = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    // Push the tag with the text we just got to the token stack.
                    let tag = Token::from(tag_text.clone())
                        .expect(format!("Invalid tag: {}", tag_text).as_str());
                    token_stack.push(tag);
                }
                '-' => {
                    chars_stack.push(ch);
                    if chars_stack.len() == 4 && chars_stack == ['<', '!', '-', '-'] {
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
    token_stack
}

fn stack_to_dom(token_stack: Vec<Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut start_tokens_stack: Vec<Token> = Vec::new();
    let mut start_token_index = 0;
    for (i, token) in token_stack.iter().enumerate() {
        match token {
            Token::Start(tag, attrs) => {
                let is_void_tag = VOID_TAGS.contains(&tag.as_str());
                if start_tokens_stack.is_empty() {
                    if is_void_tag {
                        nodes.push(Node::Element {
                            name: tag.clone(),
                            attrs: attrs.clone(),
                            children: Vec::new(),
                        });
                    } else {
                        start_token_index = i;
                        start_tokens_stack.push(Token::Start(tag.clone(), attrs.clone()));
                    }
                } else {
                    if !is_void_tag {
                        start_tokens_stack.push(Token::Start(tag.clone(), attrs.clone()));
                    }
                }
            }
            Token::End(tag) => {
                let start_tag = start_tokens_stack
                    .pop()
                    .expect(format!("unexpected end tag: {}", tag).as_str())
                    .into_node()
                    .try_into_element()
                    .unwrap();
                if start_tokens_stack.is_empty() {
                    nodes.push(Node::Element {
                        name: start_tag.name,
                        attrs: start_tag.attrs,
                        children: stack_to_dom(token_stack[start_token_index + 1..i].to_vec()),
                    })
                }
            }
            _ => {
                if start_tokens_stack.is_empty() {
                    nodes.push(token.node());
                }
            }
        }
    }
    nodes
}

/// Parse the html string and return a `Vector` of `Node`.
///
/// Example:
///
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
///     Doctype,
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
pub fn parse(html: &str) -> Vec<Node> {
    let stack = html_to_stack(html);
    let dom = stack_to_dom(stack);
    dom
}
