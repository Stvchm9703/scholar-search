use reqwest;
use reqwest::header::ACCEPT;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;
use serde_json::Value;

// use query_map::QueryMap;
use crate::semantic_scholar_api::data::{Paper, PaperDetail, SemanticScholarApiRequest};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkResponse {
    pub total: i32,
    pub token: Value,
    pub data: Option<Vec<Paper>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BulkRequest {
    pub query: String,
    // pub fields: String,
    // pub sort: HashMap<String, String>,
    // pub publication_types: Option<Vec<String>>,
    pub min_citation_count: i32,
    pub publication_date_or_year: String,
}

impl SemanticScholarApiRequest for BulkRequest {
    fn to_url(self) -> String {
        format!(
            "https://api.semanticscholar.org/graph/v1/paper/search/bulk?query={query}&fields=paperId,corpusId,url,title,venue,year,authors,externalIds,abstract,referenceCount,citationCount,influentialCitationCount,isOpenAccess,openAccessPdf,fieldsOfStudy,s2FieldsOfStudy,publicationTypes,publicationDate,journal,citationStyles,authors&sort={sort}&minCitationCount={min_citation_count}&minCitationCount={min_citation_count}&publicationDateOrYear={publication_date_or_year}"
            ,
            query = self.query,
            sort = "citationCount:desc",
            // publication_types = self.publication_types,
            min_citation_count = self.min_citation_count,
            publication_date_or_year = self.publication_date_or_year
        )
    }
}
impl Default for BulkRequest {
    fn default() -> BulkRequest {
        BulkRequest {
            query: String::from(""),
            min_citation_count: 3,
            publication_date_or_year: String::from("2023"),
        }
    }
}
// #[warn(dead_code)]
pub async fn fetch_papers(
    request: BulkRequest,
) -> Result<BulkResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(request.to_url())
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .json::<BulkResponse>()
        .await
        .unwrap();
    // let bulk_response: BulkResponse = response.()?;
    Ok(response)
}

pub async fn fetch_paper_detail(
    paper_id: String,
) -> Result<PaperDetail, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request_url = format!( "https://api.semanticscholar.org/graph/v1/paper/{paper_id}?fields=paperId,corpusId,url,title,venue,publicationVenue,year,authors,externalIds,abstract,referenceCount,citationCount,influentialCitationCount,isOpenAccess,openAccessPdf,fieldsOfStudy,s2FieldsOfStudy,publicationTypes,publicationDate,journal,citationStyles,embedding,tldr", paper_id = paper_id );
    let response = client
        .get(request_url)
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        // .text()
        .json::<PaperDetail>()
        .await
        .unwrap();

    // println!("response: {:#?}", response);
    // let deserializer: &mut serde_json::Deserializer<serde_json::de::StrRead<'_>> = &mut serde_json::Deserializer::from_str(&response);
    // let result: Result<PaperDetail, _> = serde_path_to_error::deserialize(deserializer);
    // match result {
    //     Ok(_) => println!("Expected an error"),
    //     Err(err) => {
    //         panic!("{}", err);
    //     }
    // }
    // Ok(PaperDetail::default())
    // Ok(serde_json::from_str::<PaperDetail>(&response).unwrap())
    // let bulk_response: BulkResponse = response.()?;
    Ok(response)
}
