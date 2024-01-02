use crate::semantic_scholar_api::data::{Author, Paper, PaperDetail, S2FieldsOfStudy};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Template, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[template(path = "table_row.html", ext = "html")]
pub struct TableRowTemplate {
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

impl From<Paper> for TableRowTemplate {
    fn from(x: Paper) -> Self {
        let mut external_ids_set: String = "".to_string();
        if let Some(ext_set) = x.external_ids {
            external_ids_set = format!(
                "{:?}",
                vec![
                    ext_set.doi.unwrap_or_default(),
                    ext_set.pub_med.unwrap_or_default(),
                    ext_set.pub_med_central.unwrap_or_default(),
                    ext_set.ar_xiv.unwrap_or_default(),
                    ext_set.dblp.unwrap_or_default(),
                    ext_set.mag.unwrap_or_default()
                ]
            );
        }
        Self {
            paper_id: x.paper_id.unwrap_or_default(),
            title: x.title,
            external_id: external_ids_set,
            authors: x
                .authors
                .unwrap_or_default()
                .into_iter()
                .map(|y| y.name)
                .collect::<Vec<String>>()
                .join(", "),
            keywords: "".to_string(),
            abstract_content: x.abstract_field.unwrap_or_default(),
            year: x.year.unwrap_or(0).to_string(),
            venue: x.venue.unwrap_or("".to_string()),
            is_open_access: x.is_open_access.unwrap_or(false),
            citation_count: x.citation_count.unwrap_or_default(),
            reference_count: x.reference_count.unwrap_or_default(),
        }
    }
}

#[derive(Template, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[template(path = "paper_detail.html", ext = "html")]
pub struct PaperDetailTemplate {
    pub paper_id: String,
    pub fetched: bool,
    pub paper_detail: PaperDetailTemplateDetailPrint,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PaperDetailTemplateDetailPrint {
    pub paper_id: String,
    pub corpus_id: i32,
    pub external_ids: HashMap<String, String>,
    pub url: String,
    pub title: String,
    pub abstract_field: String,
    pub venue: String,
    // pub publication_venue: Option<PublicationVenue>,
    pub year: i32,
    pub reference_count: i32,
    pub citation_count: i32,
    pub influential_citation_count: i32,
    pub is_open_access: bool,
    pub open_access_pdf: HashMap<String, String>,
    pub fields_of_study: Vec<String>,
    pub s2fields_of_study: Vec<S2FieldsOfStudy>,
    pub publication_types: Vec<String>,
    pub publication_date: String,
    // pub journal: Option<Journal>,
    // pub citation_styles: Option<CitationStyles>,
    pub authors: Vec<Author>,
    pub references_count: i32,
    pub citations_count: i32,
    // pub citations: Vec<TableRowTemplate>,
    // pub references: Vec<TableRowTemplate>,
    // pub embedding:  HashMap<String, String>,
    pub tldr: HashMap<String, String>,
}

impl From<PaperDetail> for PaperDetailTemplateDetailPrint {
    fn from(sorce: PaperDetail) -> Self {
        
        let mut external_ids_map: HashMap<String, String> = HashMap::new();
        if let Some(ext_ids) = sorce.external_ids {
            if ext_ids.doi.is_some() {
                external_ids_map.insert("doi".to_string(), ext_ids.doi.unwrap());
            }
            if ext_ids.dblp.is_some() {
                external_ids_map.insert("dblp".to_string(), ext_ids.dblp.unwrap());
            }
            if ext_ids.ar_xiv.is_some() {
                external_ids_map.insert("ar_xiv".to_string(), ext_ids.ar_xiv.unwrap());
            }
            if ext_ids.corpus_id.is_some() {
                external_ids_map.insert(
                    "corpus_id".to_string(),
                    ext_ids.corpus_id.unwrap().to_string(),
                );
            }
            if ext_ids.pub_med_central.is_some() {
                external_ids_map.insert(
                    "pub_med_central".to_string(),
                    ext_ids.pub_med_central.unwrap(),
                );
            }
            if ext_ids.pub_med.is_some() {
                external_ids_map.insert("pub_med".to_string(), ext_ids.pub_med.unwrap());
            }
            if ext_ids.mag.is_some() {
                external_ids_map.insert("mag".to_string(), ext_ids.mag.unwrap());
            }
        }
        let mut tldr_map: HashMap<String, String> = HashMap::new();
        if let Some(tldr) = sorce.tldr {
            tldr_map.insert("model".to_string(), tldr.model);
            tldr_map.insert("text".to_string(), tldr.text);
        }

        let mut open_pdf_map: HashMap<String, String> = HashMap::new();
        if let Some(open_pdf) = sorce.open_access_pdf {
            open_pdf_map.insert("url".to_string(), open_pdf.url.unwrap());
            open_pdf_map.insert("status".to_string(), open_pdf.status.unwrap());
        }

        Self {
            paper_id: sorce.paper_id,
            corpus_id: sorce.corpus_id.unwrap_or(0),
            external_ids: external_ids_map,
            url: sorce.url.unwrap_or("".to_string()),
            title: sorce.title,
            abstract_field: sorce.abstract_field.unwrap_or_default(),
            venue: sorce.venue.unwrap_or("".to_string()),
            year: sorce.year,
            reference_count: sorce.reference_count,
            citation_count: sorce.citation_count,
            influential_citation_count: sorce.influential_citation_count,
            is_open_access: sorce.is_open_access,
            open_access_pdf: open_pdf_map,
            fields_of_study: sorce.fields_of_study.unwrap_or_default(),
            publication_types: sorce.publication_types.unwrap_or_default(),
            publication_date: sorce.publication_date.unwrap_or_default(),
            authors: sorce.authors.unwrap_or_default(),
            references_count: sorce.reference_count,
            citations_count: sorce.citation_count,
            s2fields_of_study: sorce.s2fields_of_study.unwrap_or_default(),
            // references: sorce
            //     .references
            //     .unwrap_or(Vec::new())
            //     .into_iter()
            //     .map(|x| TableRowTemplate::from(x))
            //     .collect::<Vec<TableRowTemplate>>(),
            // citations: sorce
            //     .citations
            //     .unwrap_or(Vec::new())
            //     .into_iter()
            //     .map(|x| TableRowTemplate::from(x))
            //     .collect::<Vec<TableRowTemplate>>(),
            tldr: tldr_map,
        }
    }
}
