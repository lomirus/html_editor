use html_editor::operation::*;
use html_editor::{parse, Node, Element};

#[test]
fn test_parse() {
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

// Nothing inside script and style tags should be escaped
#[test]
fn no_unescapes_in_script_and_style() {
    const HTML: &str = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <script>let text = "this tag shouldn't be escaped -> <p> hi </p>"</script>
            <style>main:before { content: "fake <b>tag</b>"; }</style>
        </head>
        </html>"#;

    let html = parse(HTML).unwrap();

    let script_selector = Selector::from("script");

    let Some(script) = html.query(&script_selector) else {
        assert!(false, "Couldn't find script");
        return;
    };

    assert_eq!(script.name, "script");
    let Some(Node::Text(script_content)) = script.children.get(0) else {
        assert!(false, "Invalid script contents");
        return;
    };

    assert_eq!(script_content, r#"let text = "this tag shouldn't be escaped -> <p> hi </p>""#);

    let style_selector = Selector::from("style");

    let Some(style) = html.query(&style_selector) else {
        assert!(false, "Couldn't find style");
        return;
    };

    assert_eq!(style.name, "style");
    let Some(Node::Text(style_content)) = style.children.get(0) else {
        assert!(false, "Invalid script contents");
        return;
    };

    assert_eq!(style_content, r#"main:before { content: "fake <b>tag</b>"; }"#);
}

#[test]
fn no_escapes_in_script_and_style() {
    let element = Element::new(
        "head",
        vec![],
        vec![
            Node::Element(Element::new(
                "script",
                vec![],
                vec![Node::Text(r#"let text = "this tag shouldn't be escaped -> <p> hi </p>""#.into())],
            )),
            Node::Element(Element::new(
                "style",
                vec![],
                vec![Node::Text(r#"main:before { content: "fake <b>tag</b>"; }"#.into())],
            )),
        ],
    );

    let generated = element.html();
    assert_eq!(generated, r#"<head><script>let text = "this tag shouldn't be escaped -> <p> hi </p>"</script><style>main:before { content: "fake <b>tag</b>"; }</style></head>"#);
}
