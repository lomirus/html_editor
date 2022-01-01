use html_editor::{parse, Queryable, Selector};

const HTML: &str = r#"
    <div>
        <span>Hello</span>
        <span>World</span>
        <div class="last">Last Element</div>
    </div>"#;

#[test]
fn nodes_query() {
    let nodes = parse(HTML);
    let element = nodes.query(&Selector::from("span")).unwrap();
    println!("{:?}", element);
}

#[test]
fn nodes_query_all() {
    let nodes = parse(HTML);
    let elements = nodes.query_all(&Selector::from("span"));
    println!("{:?}", elements);
}

#[test]
fn element_query() {
    let nodes = parse(HTML);
    let node = nodes.into_iter().nth(1).unwrap();
    let element = node
        .try_into_element()
        .unwrap()
        .query(&Selector::from("span"))
        .unwrap();
    println!("{:?}", element);
}

#[test]
fn element_query_all() {
    let nodes = parse(HTML);
    let node = nodes.into_iter().nth(1).unwrap();
    let elements = node
        .try_into_element()
        .unwrap()
        .query_all(&Selector::from("span"));
    println!("{:?}", elements);
}

#[test]
fn class_query() {
    let nodes = parse(HTML);
    let element = nodes.query(&Selector::from(".last")).unwrap();
    println!("{:?}", element);
}
