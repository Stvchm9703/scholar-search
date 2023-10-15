use crate::semantic_scholar_api::data::{Paper, SemanticScholarApiRequest};
use reqwest;
use reqwest::header::ACCEPT;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitingResponse {
    pub offset: Option<i32>,
    pub next: Option<i32>,
    pub data: Option<Vec<CitingDaum>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitingDaum {
    pub contexts: Vec<String>,
    pub intents: Vec<String>,
    pub is_influential: bool,
    pub citing_paper: Paper,
}

#[derive( Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitingRequest {
    pub paper_id: String,
    pub offset: i32,
    pub limit: i32,
}

impl SemanticScholarApiRequest for CitingRequest {
   
    ///  contexts - snippets of text where the reference is mentioned"
    ///  intents - Intents derived from the contexts in which this citation is mentioned. See this more details.
    ///  isInfluential - See the S2 FAQ on influential citations.
    ///  paperId - Always included. A unique (string) identifier for this paper
    ///  corpusId - A second unique (numeric) identifier for this paper
    ///  url - URL on the Semantic Scholar website
    ///  title - Included if no fields are specified
    ///  venue - Normalized venue name
    ///  publicationVenue - Publication venue meta-data for the paper
    ///  year - Year of publication
    ///  authors - Up to 500 will be returned. Will include: authorId & name
    ///  To get more detailed information about an author's papers, use the /author/{author_id}/papers endpoint
    ///  externalIdsIDs from external sources - Supports ArXiv, MAG, ACL, PubMed, Medline, PubMedCentral, DBLP, DOI
    ///  abstract - The paper's abstract. Note that due to legal reasons, this may be missing even if we display an abstract on the website
    ///  referenceCount - Total number of papers referenced by this paper
    ///  citationCount - Total number of citations S2 has found for this paper
    ///  influentialCitationCount - More information here
    ///  isOpenAccess - More information here
    ///  openAccessPdf - A link to the paper if it is open access, and we have a direct link to the pdf
    ///  fieldsOfStudy - A list of high-level academic categories from external sources
    ///  s2FieldsOfStudy - A list of academic categories, sourced from either external sources or our internally developed classifier
    ///  publicationTypes - Journal Article, Conference, Review, etc
    ///  publicationDate - YYYY-MM-DD, if available
    ///  journal - Journal name, volume, and pages, if available
    ///  citationStyles
    fn to_url(self) -> String {
        format!( "https://api.semanticscholar.org/graph/v1/paper/{paper_id}/citations?offset={offset}&limit={limit}&fields=contexts,intents,isInfluential,paperId,corpusId,url,title,venue,publicationVenue,year,authors,externalIds,abstract,referenceCount,citationCount,influentialCitationCount,isOpenAccess,openAccessPdf,fieldsOfStudy,s2FieldsOfStudy,publicationTypes,publicationDate,journal,citationStyles" ,
            paper_id = self.paper_id,
            offset = self.offset,
            limit = self.limit
        )
    }
}

impl Default for CitingRequest {
    fn default() -> Self {
        CitingRequest {
            paper_id: String::from(""),
            offset: 0,
            limit: 100,
        }
    }
}

pub async fn fetch_citing(
    request: CitingRequest,
) -> Result<CitingResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let usl = request.to_url();
    println!("usl: {:#?}", usl);
    let response = client
        .get(usl)
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .json::<CitingResponse>()
        .await
        .unwrap();

    Ok(response)
}

pub async fn fetch_references(
    request: CitingRequest,
) -> Result<CitingResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let usl = request.to_url();
    println!("usl: {:#?}", usl);
    let response = client
        .get(usl)
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .json::<CitingResponse>()
        .await
        .unwrap();

    Ok(response)
}
