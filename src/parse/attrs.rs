// Let's take `<img src="example.png" alt=image>` for example.
enum AttrPos {
    // This includes `src`, `alt`
    Key,
    // This includes `=`
    Equal,
    // This includes `example.png`, `image`
    Value(Option<char>),
    // This includes ` `
    Space,
}

// Valid `attr_str` like: `src="example.png" alt=example disabled`
pub fn parse(attr_str: String) -> Vec<(String, String)> {
    let mut chars_stack: Vec<char> = Vec::new();
    let mut key_stack: Vec<String> = Vec::new();
    let mut value_stack: Vec<String> = Vec::new();
    let mut attr_pos = AttrPos::Key;
    for ch in attr_str.chars() {
        match attr_pos {
            AttrPos::Key => match ch {
                '=' => {
                    attr_pos = AttrPos::Equal;
                    let key = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    key_stack.push(key)
                }
                ' ' => {
                    attr_pos = AttrPos::Space;
                    let key = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    key_stack.push(key);
                    value_stack.push(String::new())
                }
                _ => chars_stack.push(ch),
            },
            AttrPos::Equal => match ch {
                '\'' => attr_pos = AttrPos::Value(Some('\'')),
                '\"' => attr_pos = AttrPos::Value(Some('\"')),
                _ => {
                    attr_pos = AttrPos::Value(None);
                    chars_stack.push(ch)
                }
            },
            AttrPos::Value(delimiter) => match delimiter {
                None => {
                    if ch == ' ' {
                        attr_pos = AttrPos::Space;
                        let value = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        value_stack.push(value)
                    } else {
                        chars_stack.push(ch);
                    }
                }
                Some(quote) => {
                    if ch == quote {
                        if chars_stack.len() == 0 {
                            value_stack.push(String::new());
                            attr_pos = AttrPos::Space;
                            continue;
                        }
                        let last_char = chars_stack
                            .last()
                            .expect("cannot accesss the last char in `chars_stack`");
                        if last_char == &'\\' {
                            chars_stack.push(ch);
                            continue;
                        } else {
                            attr_pos = AttrPos::Space;
                            let value = String::from_iter(chars_stack);
                            chars_stack = Vec::new();
                            value_stack.push(value)
                        }
                    } else {
                        chars_stack.push(ch)
                    }
                }
            },
            AttrPos::Space => {
                if ch != ' ' {
                    attr_pos = AttrPos::Key;
                    chars_stack.push(ch);
                }
            }
        }
    }

    let err_info = format!("cannot parse the attributes: {}", attr_str);
    let err_info = err_info.as_str();

    if !chars_stack.is_empty() {
        let str = String::from_iter(chars_stack);
        match attr_pos {
            AttrPos::Key => {
                key_stack.push(str);
                value_stack.push(String::new());
            }
            AttrPos::Value(delimiter) => {
                if let None = delimiter {
                    value_stack.push(str);
                } else {
                    panic!("{}", err_info)
                }
            }
            _ => {}
        }
    }

    if key_stack.len() != value_stack.len() {
        panic!(
            "{}\nkey:\t{:?}\nvalue:\t{:?}",
            err_info, key_stack, value_stack
        )
    }

    let mut attrs = Vec::new();
    let len = key_stack.len();
    for _ in 0..len {
        attrs.push((
            key_stack.pop().expect(err_info),
            value_stack.pop().expect(err_info),
        ));
    }
    attrs
}
