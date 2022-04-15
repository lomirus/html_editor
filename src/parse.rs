// TODO:
//
// Maybe we could have more elegant and concise code?
// I will try to figure out a better way to parse when
// am availible. Busy now for me :(
//
// If you have any idea, feel free to post an issue.

mod attrs;
mod token;

use crate::{data::VOID_TAGS, Node};
use token::Token;

fn html_to_stack(html: &str) -> Result<Vec<Token>, String> {
    let mut chars_stack = Vec::<char>::new();
    let mut token_stack = Vec::<Token>::new();
    let mut in_quotes: Option<char> = None;
    // More precisely: is in angle brackets
    let mut in_brackets = false;
    let mut in_comment = false;
    let mut in_script = false;
    let mut in_style = false;
    for ch in html.chars() {
        if let Some(quote) = in_quotes {
            if ch == quote {
                let previous_char = chars_stack
                    .last()
                    .expect("cannot get the last char in chars stack")
                    .clone();
                if previous_char != '\\' {
                    in_quotes = None;
                }
            }
            chars_stack.push(ch);
        } else if in_comment {
            chars_stack.push(ch);

            if String::from_iter(&chars_stack).ends_with("-->") {
                let comment = String::from_iter(chars_stack);
                chars_stack = Vec::new();
                token_stack.push(Token::from_comment(comment));
                in_comment = false;
                in_brackets = false;
            }
        } else if in_script {
            chars_stack.push(ch);
            let len = chars_stack.len();

            if String::from_iter(&chars_stack).ends_with("</script>") {
                let script = String::from_iter(chars_stack[..len - 9].to_vec());
                chars_stack = Vec::new();
                token_stack.push(Token::Text(script));
                token_stack.push(Token::End("script".to_string()));
                in_script = false;
            }
        } else if in_style {
            chars_stack.push(ch);
            let len = chars_stack.len();

            if String::from_iter(&chars_stack).ends_with("</style>") {
                let style = String::from_iter(chars_stack[..len - 8].to_vec());
                chars_stack = Vec::new();
                token_stack.push(Token::Text(style));
                token_stack.push(Token::End("style".to_string()));
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
                    token_stack.push(tag.clone());
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
                    if String::from_iter(&chars_stack) == "<!--" {
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
        let text = String::from_iter(chars_stack);
        token_stack.push(Token::Text(text));
    }
    Ok(token_stack)
}

fn stack_to_dom(token_stack: Vec<Token>) -> Result<Vec<Node>, String> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut start_tags_stack: Vec<Token> = Vec::new();
    let mut start_tag_index = 0;
    for (i, token) in token_stack.iter().enumerate() {
        match token {
            Token::Start(tag, attrs) => {
                let is_void_tag = VOID_TAGS.contains(&tag.as_str());
                if start_tags_stack.is_empty() {
                    if is_void_tag {
                        nodes.push(Node::Element {
                            name: tag.clone(),
                            attrs: attrs.clone(),
                            children: Vec::new(),
                        });
                    } else {
                        start_tag_index = i;
                        start_tags_stack.push(Token::Start(tag.clone(), attrs.clone()));
                    }
                } else {
                    if is_void_tag {
                        // You do not need to push the void tag to the stack
                        // like above, because it must be inside the the
                        // element of the first start tag, and this element
                        // will then be pushed to the stack recursively.
                    } else {
                        start_tags_stack.push(Token::Start(tag.clone(), attrs.clone()));
                    }
                }
            }
            Token::End(tag) => {
                let start_tag = match start_tags_stack.pop() {
                    Some(token) => token.into_node().into_element(),
                    None => return Err(format!("No start tag matches </{}>", tag)),
                };
                if tag != &start_tag.name {
                    return Err(format!(
                        "<{}> does not match the </{}>",
                        start_tag.name, tag
                    ));
                }
                if start_tags_stack.is_empty() {
                    nodes.push(Node::Element {
                        name: start_tag.name,
                        attrs: start_tag.attrs,
                        children: stack_to_dom(token_stack[start_tag_index + 1..i].to_vec())?,
                    })
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
        Some(token) => {
            let start_tag_name = token.into_node().into_element().name;
            Err(format!("<{}> is not closed", start_tag_name))
        }
        None => Ok(nodes),
    }
}

fn try_stack_to_dom(token_stack: Vec<Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut start_tags_stack: Vec<Token> = Vec::new();
    let mut start_tag_index = 0;

    for (i, token) in token_stack.iter().enumerate() {
        match token {
            Token::Start(tag, attrs) => {
                let is_void_tag = VOID_TAGS.contains(&tag.as_str());
                if start_tags_stack.is_empty() {
                    if is_void_tag {
                        nodes.push(Node::Element {
                            name: tag.clone(),
                            attrs: attrs.clone(),
                            children: Vec::new(),
                        });
                    } else {
                        start_tag_index = i;
                        start_tags_stack.push(Token::Start(tag.clone(), attrs.clone()));
                    }
                } else {
                    if is_void_tag {
                        // You do not need to push the void tag to the stack
                        // like above, because it must be inside the the
                        // element of the first start tag, and this element
                        // will then be pushed to the stack recursively.
                    } else {
                        start_tags_stack.push(Token::Start(tag.clone(), attrs.clone()));
                    }
                }
            }
            Token::End(tag) => {
                let start_tag = match start_tags_stack.pop() {
                    Some(token) => token.into_node().into_element(),
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
                    nodes.push(Node::Element {
                        name: start_tag.name,
                        attrs: start_tag.attrs,
                        children: try_stack_to_dom(token_stack[start_tag_index + 1..i].to_vec()),
                    })
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
            Token::Start(name, attrs) => Node::Element {
                name,
                attrs,
                children: try_stack_to_dom(token_stack[start_tag_index + 1..].to_vec()),
            },
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
pub fn parse(html: &str) -> Result<Vec<Node>, String> {
    let stack = html_to_stack(html)?;
    // println!("{:#?}", stack);
    let dom = stack_to_dom(stack);
    dom
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
    let stack = html_to_stack(html).unwrap_or(vec![]);
    // println!("{:?}", stack);
    let dom = try_stack_to_dom(stack);
    dom
}
