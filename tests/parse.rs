use html_editor::operation::*;
use html_editor::{parse, try_parse};

#[test]
fn paired_tag() {
    parse("<p></p>").unwrap();
    parse("<div>Hello, world!</div>").unwrap();
}

#[test]
fn void_tag() {
    parse("<div />").unwrap();
    parse("<div/>").unwrap();
}

#[test]
fn self_closing_tag() {
    parse("<img>").unwrap();
}

#[test]
fn comment_tag() {
    parse("<!-- comment -->").unwrap();
    parse("<!--comment-->").unwrap();
}

#[test]
fn attributes() {
    parse("<img src=\"example.png\" alt=example>").unwrap();
    parse("<input disabled type=\"button\">").unwrap();
}

#[test]
fn matched() {
    let a = parse(
        r#"
        <span>
            <span>
                <span></span>
            </span>
        </span>"#,
    )
    .unwrap()
    .trim()
    .html();
    let b = parse(
        r#"
        <span></span>
        <span></span>
        <span></span>"#,
    )
    .unwrap()
    .trim()
    .html();
    let c = parse(
        r#"
        <span>
            <span></span>
        </span>
        <span></span>"#,
    )
    .unwrap()
    .trim()
    .html();

    assert_eq!("<span><span><span></span></span></span>", a);
    assert_eq!("<span></span><span></span><span></span>", b);
    assert_eq!("<span><span></span></span><span></span>", c);
}

#[test]
fn complex() {
    let mut dom = parse(
        r#"
            <input value="<p value='haha'></p>" disable placeholder=input>
            <input value="\"\"''/>">
            <!-- <p></p> -->
            <!------------->
            <a b="" c="d"></a>
            <div>
                <script></script>
                <script>'<'</script>
                <script>"</div>"</script>
                <style>div::after{ content: "</div>" }</style>
            </div>
            <div 
            ></div>
        "#,
    )
    .unwrap();
    dom.trim();
}

#[test]
fn fault_tolerance() {
    assert_eq!(
        try_parse(r#"<div><a>Ipsum"#).html(),
        "<div><a>Ipsum</a></div>"
    );
    assert_eq!(try_parse(r#"<div>Ipsum</a>"#).html(), "<div>Ipsum</div>");
    assert_eq!(
        try_parse(r#"<span><span>Ipsum</span>"#).html(),
        "<span><span>Ipsum</span></span>"
    );
}

#[test]
fn autocomplete_multiple_unclosed_tags() {
    assert_eq!(
        try_parse(r#"<img><h1><h2><h3><img><h4><h5><h6><h7><h8><h9><img>"#).html(),
        "<img><h1><h2><h3><img><h4><h5><h6><h7><h8><h9><img></h9></h8></h7></h6></h5></h4></h3></h2></h1>"
    );
}

#[test]
fn xml() {
    let mut html = parse(
        r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <message>
                <log:warning>
                    Hello World
                </log:warning>
            </message>
        "#,
    )
    .unwrap();
    html.trim();
}
