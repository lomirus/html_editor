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
    let element = nodes.query(&Selector::from("span")).unwrap();
    println!("{:?}", element);
}

#[test]
fn nodes_query_all() {
    let nodes = parse(HTML).unwrap();
    let elements = nodes.query_all(&Selector::from("span"));
    println!("{:?}", elements);
}

#[test]
fn element_query() {
    let nodes = parse(HTML).unwrap();
    let node = nodes.into_iter().nth(1).unwrap();
    let element = node.into_element().query(&Selector::from("span")).unwrap();
    println!("{:?}", element);
}

#[test]
fn element_query_all() {
    let nodes = parse(HTML).unwrap();
    let node = nodes.into_iter().nth(1).unwrap();
    let elements = node.into_element().query_all(&Selector::from("span"));
    println!("{:?}", elements);
}

#[test]
fn class_query() {
    let nodes = parse(HTML).unwrap();
    let element = nodes.query(&Selector::from(".last")).unwrap();
    println!("{:?}", element);
}
