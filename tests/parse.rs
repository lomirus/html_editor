use html_editor::{parse, try_parse};
use html_editor::prelude::*;

#[test]
fn paired_tag() {
    let a = parse("<p></p>");
    let b = parse("<div>Hello, world!</div>");

    println!("{:#?}", a);
    println!("{:#?}", b);
}

#[test]
fn void_tag() {
    let a = parse("<div />");
    let b = parse("<div/>");

    println!("{:#?}", a);
    println!("{:#?}", b);
}

#[test]
fn self_closing_tag() {
    let a = parse("<img>");

    println!("{:#?}", a);
}

#[test]
fn comment_tag() {
    let a = parse("<!-- comment -->");
    let b = parse("<!--comment-->");

    println!("{:#?}", a);
    println!("{:#?}", b);
}

#[test]
fn attributes() {
    let a = parse("<img src=\"example.png\" alt=example>");
    let b = parse("<input disabled type=\"button\">");

    println!("{:#?}", a);
    println!("{:#?}", b);
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
        "#,
    )
    .unwrap();
    dom.trim();

    println!("{:#?}", dom);
}

#[test]
fn fault_tolerance() {
    assert_eq!(try_parse(r#"<div><a>Ipsum"#).html(), "<div><a>Ipsum</a></div>");
    assert_eq!(try_parse(r#"<div>Ipsum</a>"#).html(), "<div>Ipsum</div>");
    assert_eq!(
        try_parse(r#"<span><span>Ipsum</span>"#).html(),
        "<span><span>Ipsum</span></span>"
    );
}
