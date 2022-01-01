use html_editor::{parse, Editable};

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
    ).trim();
    let b = parse(
        r#"
        <span></span>
        <span></span>
        <span></span>"#,
    ).trim();
    let c = parse(
        r#"
        <span>
            <span></span>
        </span>
        <span></span>"#,
    ).trim();

    println!("{:#?}", a);
    println!("{:#?}", b);
    println!("{:#?}", c);
}

#[test]
fn complex() {
    let a = parse(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>
        <body>
            <header>Example</header>
            <div>
                <input value="<p value='haha'></p>" disable placeholder=input>
                <input value="\"\"''/>">
                <!-- Nothing is true -->
                <!-- Everything is permitted -->
                <!-- <p></p> -->
                <!------------->
                <a b="" c="d"></a>
            </div>
            <footer></footer>
        </body>
        </html>"#,
    ).trim();

    println!("{:#?}", a);
}
