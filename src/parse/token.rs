use crate::parse::attrs;
use crate::{Doctype, Element, Node};

#[derive(Debug, Clone)]
pub enum Token {
    /// Like `<div>`, including `<img>`, `<input>`, etc.
    Start(String, Vec<(String, String)>),
    /// Like `</div>`
    End(String),
    /// Like `<div />`
    Closing(String, Vec<(String, String)>),
    /// Like `<!doctype html>`
    Doctype(Doctype),
    /// Like `<!-- comment -->`
    Comment(String),
    /// Any text
    Text(String),
}

impl Token {
    pub fn from(tag: String) -> Result<Self, String> {
        if tag.ends_with("/>") {
            let tag_name_start = tag[1..tag.len()]
                .chars()
                .position(|x| x != ' ')
                .expect("tag name cannot be all spaces after \"<\"")
                + 1;
            let tag_name_end_option = tag[tag_name_start..tag.len()]
                .chars()
                .position(|x| x == ' ');
            let tag_name_end = match tag_name_end_option {
                Some(end) => end + tag_name_start,
                None => tag.len() - 2,
            };
            let tag_name = tag[tag_name_start..tag_name_end].to_string();
            let attr_str = tag[tag_name_end..tag.len() - 2].trim().to_string();
            Ok(Self::Closing(tag_name, attrs::parse(attr_str)))
        } else if tag.starts_with("</") {
            Ok(Self::End(tag[2..tag.len() - 1].trim().to_string()))
        } else if tag.starts_with("<!--") {
            Ok(Self::from_comment(tag))
        } else if tag.starts_with("<!") {
            Ok(Self::Doctype(Doctype::Html))
        } else if tag.starts_with("<?") {
            let attr = tag[2..tag.len() - 2].to_string();
            let attr = attrs::parse(attr);
            let version = attr
                .iter()
                .find(|(name, _)| name == "version")
                .expect("cannot find version attribute in xml declaration")
                .1
                .to_string();
            let encoding = attr
                .iter()
                .find(|(name, _)| name == "encoding")
                .expect("cannot find encoding attribute in xml declaration")
                .1
                .to_string();
            Ok(Self::Doctype(Doctype::Xml { version, encoding }))
        } else if tag.starts_with('<') {
            let tag_name_start = tag[1..tag.len()]
                .chars()
                .position(|x| !x.is_ascii_whitespace())
                .expect("tag name cannot be all spaces after \"<\"")
                + 1;
            let tag_name_end_option = tag[tag_name_start..tag.len()]
                .chars()
                .position(|x| x.is_ascii_whitespace());
            let tag_name_end = match tag_name_end_option {
                Some(end) => end + tag_name_start,
                None => tag.len() - 1,
            };
            let tag_name = tag[tag_name_start..tag_name_end].to_string();
            let attr_str = tag[tag_name_end..tag.len() - 1].trim().to_string();
            Ok(Self::Start(tag_name, attrs::parse(attr_str)))
        } else {
            Err(format!("Invalid tag: {}", tag))
        }
    }

    #[inline]
    pub fn from_comment(comment: String) -> Self {
        Self::Comment(comment[4..comment.len() - 3].to_string())
    }

    pub fn node(&self) -> Node {
        self.clone().into_node()
    }

    pub fn into_node(self) -> Node {
        match self {
            Self::Start(name, attrs) => Element {
                name,
                attrs,
                children: Vec::new(),
            }
            .into_node(),

            Self::End(name) => Element {
                name,
                attrs: Vec::new(),
                children: Vec::new(),
            }
            .into_node(),

            Self::Closing(name, attrs) => Element {
                name,
                attrs,
                children: Vec::new(),
            }
            .into_node(),

            Self::Doctype(doctype) => Node::Doctype(doctype),
            Self::Comment(comment) => Node::Comment(comment),
            Self::Text(text) => Node::Text(text),
        }
    }

    pub fn into_element(self) -> Element {
        match self {
            Self::Start(name, attrs) => Element {
                name,
                attrs,
                children: Vec::new(),
            },
            Self::End(name) => Element {
                name,
                attrs: Vec::new(),
                children: Vec::new(),
            },
            Self::Closing(name, attrs) => Element {
                name,
                attrs,
                children: Vec::new(),
            },
            _ => panic!("Cannot convert token to element"),
        }
    }
}
