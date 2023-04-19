use html_editor::operation::*;
use html_editor::{parse, Node, Element};

const HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>I &lt;3 &#34;escaping&#34;</title>
    </head>
    <body>
        <div id="testee" attr="id-with-&quot;quotes&quot;-inside"></div>
    </body>
    </html>"#;

#[test]
fn test_parse() {
    let html = parse(HTML).unwrap();
    let title_selector = Selector::from("title");

    let Some(title) = html.query(&title_selector) else {
        assert!(false, "Invalid title");
        return;
    };

    assert_eq!(title.name, "title");
    let Some(Node::Text(title_content)) = title.children.get(0) else {
        assert!(false, "Invalid title contents");
        return;
    };
    assert_eq!(title_content, "I <3 \"escaping\"");

    let div_selector = Selector::from("#testee");

    let Some(div) = html.query(&div_selector) else {
        assert!(false, "Invalid div");
        return;
    };

    assert_eq!(
        div.attrs,
        vec![
            ("attr".into(), "id-with-\"quotes\"-inside".into()),
            ("id".into(), "testee".into()),
        ]);
}

#[test]
fn test_generate() {
    let element = Element::new(
        "dummy-tag",
        vec![("attr-1".into(), "attribute containing < and \" and &".into())],
        vec![Node::Text("fake <tag>".into())],
    );

    let generated = element.html();
    assert_eq!(generated, r#"<dummy-tag attr-1="attribute containing &lt; and &quot; and &amp;">fake &lt;tag&gt;</dummy-tag>"#);
}

