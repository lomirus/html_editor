use html_editor::{parse, Editable, Htmlifiable, Node, Selector};

const HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
    </head>
    <body>
    </body>
    </html>"#;

const INSERTED_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
    </head>
    <body>
    <script>console.log("Hello World")</script></body>
    </html>"#;

const REMOVED_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        
        <title>Document</title>
    </head>
    <body>
    </body>
    </html>"#;

#[test]
fn insert() {
    let body_selector = Selector::from("body");
    let script = Node::new_element(
        "script",
        Vec::new(),
        vec![Node::Text(r#"console.log("Hello World")"#.to_string())],
    );
    let html = parse(HTML).insert_to(&body_selector, script).html();
    assert_eq!(html, INSERTED_HTML);
}

#[test]
fn remove() {
    let meta_selector = Selector::from("meta");
    let html = parse(HTML).remove_by(&meta_selector).html();
    assert_eq!(html, REMOVED_HTML);
}
