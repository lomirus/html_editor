use html_editor::{parse, Htmlifiable, Editable};

const HTML: &str = r#"
    <div>
        <span id="class">Hello</span>
        <span class="id">World</span>
    </div>"#;

#[test]
fn original_html() {
    let html = parse(HTML).html();
    assert_eq!(html, HTML);
}

#[test]
fn trimmed_html() {
    let html = parse(HTML).trim().html();
    assert_eq!(
        html,
        r#"<div><span id="class">Hello</span><span class="id">World</span></div>"#
    );
}
