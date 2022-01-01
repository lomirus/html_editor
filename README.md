[![Crates.io](https://img.shields.io/crates/v/html_editor)](https://crates.io/crates/html_editor)
![License](https://img.shields.io/crates/l/html_editor)

# HTML Editor

Pure, simple and elegant HTML parser and editor.

## Examples

### Parse HTML segment/document

```rust
let document = parse("<!doctype html><html><head></head><body></body></html>");
println!("{:#?}", document);
```

Output:

```rust
[
    Doctype,
    Element {
        name: "html",
        attrs: {},
        children: [
            Element {
                name: "head",
                attrs: {},
                children: [],
            },
            Element {
                name: "body",
                attrs: {},
                children: [],
            },
        ],
    },
]
```

### Query an element by classname

```rust
// let html = r#"..."#
let nodes = parse(html);
let selector: Selector = Selector::from(".last");
let element: Element = nodes.query(&selector).unwrap();
```

### Query all elements by tag

```rust
// let html = r#"..."#
let nodes = parse(html);
let selector: Selector = Selector::from("span");
let elements: Vec<Element> = nodes.query_all(&selector);
```

### Edit the HTML

```rust
// let html = r#"..."#
let a: String = parse(html).trim().html();
let b: String = parse(html).insert_to(&selector, node).html();
let c: String = parse(html).remove_by(&selector).html();
```

You can find more examples in the [documentation](https://docs.rs/html_editor/latest/html_editor/).

## Changelog

See in [CHANGELOG.md](CHANGELOG.md)
