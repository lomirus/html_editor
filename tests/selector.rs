use html_editor::operation::*;

#[test]
fn simple() {
    assert_eq!(
        format!("{:?}", Selector::from("div")),
        r#"Selector([CompoundSelector([Tag("div")])])"#
    );
    assert_eq!(
        format!("{:?}", Selector::from(".class")),
        r#"Selector([CompoundSelector([Class("class")])])"#
    );
    assert_eq!(
        format!("{:?}", Selector::from("#id")),
        r#"Selector([CompoundSelector([Id("id")])])"#
    );
}

#[test]
fn compound() {
    assert_eq!(
        format!("{:?}", Selector::from("button.round")),
        r#"Selector([CompoundSelector([Tag("button"), Class("round")])])"#
    );
    assert_eq!(
        format!("{:?}", Selector::from("div#app")),
        r#"Selector([CompoundSelector([Tag("div"), Id("app")])])"#
    );
    assert_eq!(
        format!("{:?}", Selector::from("a.o#e")),
        r#"Selector([CompoundSelector([Tag("a"), Class("o"), Id("e")])])"#
    );
}

#[test]
fn complex() {
    assert_eq!(
        format!("{:?}", Selector::from("h1, h2")),
        r#"Selector([CompoundSelector([Tag("h1")]), CompoundSelector([Tag("h2")])])"#
    );
    assert_eq!(
        format!("{:?}", Selector::from(" h1,h2  ")),
        r#"Selector([CompoundSelector([Tag("h1")]), CompoundSelector([Tag("h2")])])"#
    );
}
