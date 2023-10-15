use crate::axum_server::template::table::TableRowTemplate;
use askama::Template;
use serde::{Deserialize, Serialize};
#[derive(Template, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[template(path = "search.html", ext = "html")]
pub struct SearchPageLayoutTemplate {
    // pub body: String,
    pub total_count: i32,
    pub rows: Vec<TableRowTemplate>,
}

#[derive(Template, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[template(
    source = r###"
 {% include "table.html" %}
 <div id="result-count" class="flex" hx-swap-oob="true">
  <h2>total : {{total_count}}</h2>
</div>
"###,
    ext = "html"
)]
pub struct SearchResultTemplate {
    pub query: String,
    pub rows: Vec<TableRowTemplate>,
    pub total_count: i32,
}
