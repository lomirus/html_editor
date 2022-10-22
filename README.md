[![crate-badge]][crate-link]
[![downloads-badge]][crate-link]
![License](https://img.shields.io/crates/l/html_editor)
[![check-badge]][check-link]

[crate-badge]: https://img.shields.io/crates/v/html_editor
[crate-link]: https://crates.io/crates/html_editor
[check-badge]: https://github.com/lomirus/html-editor/workflows/check/badge.svg
[check-link]: https://github.com/lomirus/html-editor/actions/workflows/check.yaml
[downloads-badge]: https://img.shields.io/crates/d/html_editor

# HTML Editor

Pure and simple HTML parser and editor.

## Examples

### Parse HTML segment/document

```rust
let document: Vec<Node> = parse("<!doctype html><html><head></head><body></body></html>")?;
println!("{:#?}", document);
```

Output:

```rust
[
    Doctype(
        Html,
    ),
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

You can also use `try_parse` to parse the html which contains tolerable errors

```rust
let document: Vec<Node> = try_parse("<div><span>Ipsum</div>");
```

### Query an element / elements by selector

```rust
// let html = r#"..."#
let nodes = parse(html)?;
let selector: Selector = Selector::from(".box");
let element: Element = nodes.query(&selector).unwrap();
let elements: Vec<Element> = nodes.query_all(&selector);
```

### Edit the HTML

```rust
// let html = r#"..."#
let a: String = parse(html)?.trim().html();
let b: String = parse(html)?.insert_to(&selector, node).html();
let c: String = parse(html)?.remove_by(&selector).html();
```

You can find more examples in the [documentation](https://docs.rs/html_editor/latest/html_editor/).

## Changelog

See in [CHANGELOG.md](CHANGELOG.md)
