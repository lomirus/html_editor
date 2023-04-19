use html_editor::{parse, InnerHTMLParseError, SourceLocation};

// This test suite only tests the "shape" of the errors, aka that the variants of the errors returned are correct
// It never tests the contents of the error messages, as those may change in the future. It does however test the error location

#[test]
fn mismatched_tags() {
    fn ensure_mismatched_at(html: &str, at: usize, start_tag: &'static str, start_location: usize, end_tag: &'static str) {
        let Err(err) = parse(html) else {
            panic!("parse should fail");
        };

        match err.inner {
            InnerHTMLParseError::MismatchedTags { start_tag: est, start_location: esl, end_tag: eend } => {
                assert_eq!(est, start_tag);
                assert_eq!(esl.0, start_location);
                assert_eq!(eend, end_tag);
            }
            _ => { panic!("Expected parse to give MismatchedTags") }
        }

        assert_eq!(err.source_location, SourceLocation(at));
    }

    ensure_mismatched_at("<p></b>", 3, "p", 0, "b");
    //                    0123456

    ensure_mismatched_at("<span><p></p></div>", 13, "span", 0, "div");
    //                    0123456789012345678

    ensure_mismatched_at("<span><p></b></div>", 9, "p", 6, "b");
    //                    0123456789012345678
}

#[test]
fn unopened_tags() {
    fn ensure_unopened_at(html: &str, at: usize, tag: &'static str) {
        let Err(err) = parse(html) else {
            panic!("parse should fail");
        };

        match err.inner {
            InnerHTMLParseError::UnopenedTag { tag: etag } => {
                assert_eq!(etag, tag);
            }
            _ => { panic!("Expected parse to give UnopenedTag") }
        }

        assert_eq!(err.source_location, SourceLocation(at));
    }

    ensure_unopened_at("</b>", 0, "b");
    //                  0123

    ensure_unopened_at("hi <span>hello</span></div>", 21, "div");
    //                  012345678901234567890123456
}

#[test]
fn unclosed_tags() {
    fn ensure_unclosed_at(html: &str, at: usize, tag: &'static str) {
        let Err(err) = parse(html) else {
            panic!("parse should fail");
        };

        match err.inner {
            InnerHTMLParseError::UnclosedTag { tag: etag } => {
                assert_eq!(etag, tag);
            }
            _ => { panic!("Expected parse to give UnclosedTag") }
        }

        assert_eq!(err.source_location, SourceLocation(at));
    }

    ensure_unclosed_at("<b>", 0, "b");
    //                  012

    ensure_unclosed_at("<div><p>", 5, "p");
    //                  01234567
}

#[test]
fn invalid_tags() {
    fn ensure_invalid_at(html: &str, at: usize, tag: &'static str) {
        let Err(err) = parse(html) else {
            panic!("parse should fail");
        };

        match err.inner {
            InnerHTMLParseError::InvalidTag { tag: etag, .. } => {
                assert_eq!(etag, tag);
            }
            _ => { panic!("Expected parse to give InvalidTag, gave {:?}", err) }
        }

        assert_eq!(err.source_location, SourceLocation(at));
    }

    // tag name cannot be all spaces
    ensure_invalid_at("<   >", 0, "<   >");
    ensure_invalid_at("<   />", 0, "<   />");
    //                  01234

    // no version in xml declaration
    ensure_invalid_at("<?xml encoding=\"UTF-8\"?>", 0, "<?xml encoding=\"UTF-8\"?>");
    //                  01234567

    // no version in xml declaration
    ensure_invalid_at("<?xml encoding=\"UTF-8\"?>", 0, "<?xml encoding=\"UTF-8\"?>");
    //                  01234567
}


