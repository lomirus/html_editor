use html_editor::operation::*;
use html_editor::parse;

const HTML: &str = r#"
    <div>
        <span>Hello</span>
        <span>World</span>
        <div class="last">Last Element</div>
    </div>"#;

#[test]
fn nodes_query() {
    let nodes = parse(HTML).unwrap();
    let _element = nodes.query(&Selector::from("span")).unwrap();
}

#[test]
fn nodes_query_all() {
    let nodes = parse(HTML).unwrap();
    let _elements = nodes.query_all(&Selector::from("span"));
}

#[test]
fn element_query() {
    let nodes = parse(HTML).unwrap();
    let node = nodes.into_iter().nth(1).unwrap();
    let _element = node.into_element().query(&Selector::from("span")).unwrap();
}

#[test]
fn element_query_all() {
    let nodes = parse(HTML).unwrap();
    let node = nodes.into_iter().nth(1).unwrap();
    let _elements = node.into_element().query_all(&Selector::from("span"));
}

#[test]
fn class_query() {
    let nodes = parse(HTML).unwrap();
    let _element = nodes.query(&Selector::from(".last")).unwrap();
}

#[test]
fn html_editor_multiple_class_parsing() {
    let snippet = r#"<div class="a b"></div>"#;
    let nodes = parse(snippet).unwrap();
    let selector = Selector::from(".a");
    nodes.query(&selector).unwrap();
}
