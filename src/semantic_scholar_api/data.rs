use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Display;
use std::str::FromStr;
// use serde::de::{self, Deserialize, Deserializer};
use serde_json as json;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paper {
    pub paper_id: Option<String>,
    pub external_ids: Option<ExternalIds>,
    #[serde(default, deserialize_with = "from_str_optional")]
    pub corpus_id: Option<i32>,
    pub publication_venue: Option<PublicationVenue>,
    pub url: Option<String>,
    pub title: String,
    #[serde(rename = "abstract")]
    pub abstract_field: Option<String>,
    pub venue: Option<String>,
    pub year: Option<i32>,
    pub reference_count: Option<i32>,
    pub citation_count: Option<i32>,
    pub influential_citation_count: Option<i32>,
    pub is_open_access: Option<bool>,
    pub open_access_pdf: Option<OpenAccessPdf>,
    #[serde(default)]
    pub fields_of_study: Option<Vec<String>>,
    #[serde(rename = "s2FieldsOfStudy")]
    pub s2fields_of_study: Option<Vec<S2FieldsOfStudy>>,
    pub publication_types: Option<Vec<String>>,
    pub publication_date: Option<String>,
    pub journal: Option<Journal>,
    pub citation_styles: Option<CitationStyles>,
    pub authors: Option<Vec<Author>>,
}

fn from_str_optional<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: serde::Deserializer<'de>,
{
    let deser_res: Result<json::Value, _> = serde::Deserialize::deserialize(deserializer);
    match deser_res {
        Ok(json::Value::String(s)) => T::from_str(&s)
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(v) => T::from_str(&v.to_string())
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Err(_) => Ok(None),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    #[serde(rename = "ArXiv")]
    pub ar_xiv: Option<String>,
    #[serde(rename = "DBLP")]
    pub dblp: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
    #[serde(rename = "CorpusId")]
    pub corpus_id: Option<i32>,
    #[serde(rename = "PubMedCentral")]
    pub pub_med_central: Option<String>,
    #[serde(rename = "PubMed")]
    pub pub_med: Option<String>,
    #[serde(rename = "MAG")]
    pub mag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAccessPdf {
    pub url: Option<String>,
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S2FieldsOfStudy {
    pub category: Option<String>,
    pub source: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Journal {
    pub name: Option<String>,
    pub volume: Option<String>,
    pub pages: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationStyles {
    pub bibtex: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub author_id: Option<String>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicationVenue {
    pub id: String,
    pub name: String,
    #[serde(rename = "alternate_names")]
    pub alternate_names: Option<Vec<String>>,
    pub issn: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "alternate_urls")]
    pub alternate_urls: Option<Vec<String>>,
}

pub trait SemanticScholarApiRequest {
    fn to_url(self) -> String;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaperDetail {
    pub paper_id: String,
    pub corpus_id: Option<i32>,
    pub external_ids: Option<ExternalIds>,
    pub url: Option<String>,
    pub title: String,
    #[serde(rename = "abstract")]
    pub abstract_field: Option<String>,
    pub venue: Option<String>,
    pub publication_venue: Option<PublicationVenue>,
    pub year: i32,
    pub reference_count: i32,
    pub citation_count: i32,
    pub influential_citation_count: i32,
    pub is_open_access: bool,
    pub open_access_pdf: Option<OpenAccessPdf>,
    pub fields_of_study: Vec<String>,
    #[serde(rename = "s2FieldsOfStudy")]
    pub s2fields_of_study: Option<Vec<S2FieldsOfStudy>>,
    pub publication_types: Option<Vec<String>>,
    pub publication_date: Option<String>,
    pub journal: Option<Journal>,
    pub citation_styles: Option<CitationStyles>,
    pub authors: Vec<Author>,
    pub citations: Option<Vec<CitationShort>>,
    pub references: Option<Vec<CitationShort>>,
    pub embedding: Option<Embedding>,
    pub tldr: Option<Tldr>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embedding {
    pub model: String,
    pub vector: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tldr {
    pub model: String,
    pub text: String,
}

// {
//   "paperId": "5ba7f36b4815e93b51e5cd93400c0eb5512d2902",
//   "title": "Can editors save peer review from peer reviewers?"
// },

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationShort {
    pub paper_id: String,
    pub title: String,
}
