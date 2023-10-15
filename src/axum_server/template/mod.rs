use askama::Template;
use serde::{Deserialize, Serialize};

pub mod table;
pub mod search_page;

#[derive(Template, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[template(path = "hello.html", ext = "html")]
pub struct LayoutTemplate {
    // pub body: String,
}


