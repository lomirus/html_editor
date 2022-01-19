use crate::Node;
use crate::parse::attrs;

#[derive(Debug, Clone)]
pub enum Token {
    // Like `<div>`, including `<img>`, `<input>`, etc.
    Start(String, Vec<(String, String)>),
    // Like `</div>`
    End(String),
    // Like `<div />`
    Closing(String, Vec<(String, String)>),
    // Like `<!doctype html>`
    Doctype,
    // Like `<!-- comment -->`
    Comment(String),
    // Any text
    Text(String),
}

impl Token {
    pub fn from(tag: String) -> Option<Self> {
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
            Some(Self::Closing(tag_name, attrs::parse(attr_str)))
        } else if tag.starts_with("</") {
            Some(Self::End(tag[2..tag.len() - 1].to_string()))
        } else if tag.starts_with("<!--") {
            Some(Self::from_comment(tag))
        } else if tag.starts_with("<!") {
            Some(Self::Doctype)
        } else if tag.starts_with("<") {
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
                None => tag.len() - 1,
            };
            let tag_name = tag[tag_name_start..tag_name_end].to_string();
            let attr_str = tag[tag_name_end..tag.len() - 1].trim().to_string();
            Some(Self::Start(tag_name, attrs::parse(attr_str)))
        } else {
            None
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
            Self::Start(name, attrs) => Node::Element {
                name,
                attrs,
                children: Vec::new(),
            },
            Self::End(name) => Node::Element {
                name,
                attrs: Vec::new(),
                children: Vec::new(),
            },
            Self::Closing(name, attrs) => Node::Element {
                name,
                attrs,
                children: Vec::new(),
            },
            Self::Doctype => Node::Doctype,
            Self::Comment(comment) => Node::Comment(comment),
            Self::Text(text) => Node::Text(text),
        }
    }
}
