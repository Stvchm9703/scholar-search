use crate::semantic_scholar_api::critions::CitingDaum;
use crate::semantic_scholar_api::data::{
    Author, Embedding, Journal, OpenAccessPdf, Paper, PaperDetail, PublicationVenue,
    S2FieldsOfStudy, Tldr,
};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Template, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[template(path = "table.html", ext = "html")]
pub struct CitingListResponse {
    pub table_id: String,
    pub offset: Option<i32>,
    pub next: Option<i32>,
    pub rows: Vec<CitingListRowTemplate>,
}

#[derive(Template, Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[template(path = "table_row.html", ext = "html")]
pub struct CitingListRowTemplate {
    pub contexts: Vec<String>,
    pub intents: Vec<String>,
    pub is_influential: bool,
    // pub citing_paper: TableRowTemplate,
    pub paper_id: String,
    pub external_id: String,
    pub title: String,
    pub authors: String,
    pub keywords: String,
    pub abstract_content: String,
    pub year: String,
    pub venue: String,
    pub is_open_access: bool,
    pub citation_count: i32,
    pub reference_count: i32,
}

impl From<CitingDaum> for CitingListRowTemplate {
    fn from(x: CitingDaum) -> Self {
        Self {
            contexts: x.contexts,
            intents: x.intents,
            is_influential: x.is_influential,

            paper_id: x.citing_paper.paper_id.unwrap_or_default(),
            title: x.citing_paper.title,
            external_id: x.citing_paper.external_ids.unwrap().doi.unwrap_or_default(),
            authors: x
                .citing_paper
                .authors
                .unwrap()
                .into_iter()
                .map(|y| y.name)
                .collect::<Vec<String>>()
                .join(", "),
            keywords: "".to_string(),
            abstract_content: x.citing_paper.abstract_field.unwrap_or_default(),
            year: x.citing_paper.year.unwrap_or(0).to_string(),
            venue: x.citing_paper.venue.unwrap_or("".to_string()),
            is_open_access: x.citing_paper.is_open_access.unwrap_or(false),
            citation_count: x.citing_paper.citation_count.unwrap_or_default(),
            reference_count: x.citing_paper.reference_count.unwrap_or_default(),
        }
    }
}
