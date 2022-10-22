//! Traits about editing, querying and stringifying the [`Element`](struct.Element.html) / [`Node`](enum.Node.html).

mod edit;
mod html;
mod query;
mod selector;

pub use edit::Editable;
pub use html::Htmlifiable;
pub use query::Queryable;
pub use selector::Selector;
