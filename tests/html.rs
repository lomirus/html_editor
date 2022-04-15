use html_editor::operation::*;
use html_editor::parse;

const HTML: &str = r#"
    <div>
        <span id="class">Hello</span>
        <span class="id">World</span>
    </div>"#;

#[test]
fn original_html() {
    let html = parse(HTML).unwrap().html();
    assert_eq!(html, HTML);
}

#[test]
fn trimmed_html() {
    let html = parse(HTML).unwrap().trim().html();
    assert_eq!(
        html,
        r#"<div><span id="class">Hello</span><span class="id">World</span></div>"#
    );
}
