pub mod api;
pub mod template;
use crate::axum_server::template::{
    page_detail::{CitingListResponse, CitingListRowTemplate},
    search_page::{SearchPageLayoutTemplate, SearchResultTemplate},
    table::{PaperDetailTemplate, PaperDetailTemplateDetailPrint, TableRowTemplate},
};

use axum::{
    extract::{Path, RawForm},
    http::{HeaderValue, Method},
    routing::IntoMakeService,
    routing::{get, post},
    Router,
};

use axum_htmx::HxBoosted;
use query_map::QueryMap;
use tower_http::cors::CorsLayer;

use crate::semantic_scholar_api::{
    critions::{fetch_citing, fetch_references, CitingRequest},
    paper_fetch::{fetch_paper_detail, fetch_papers, BulkRequest},
};


pub async fn paper_index() -> SearchPageLayoutTemplate {
    let result = fetch_papers(BulkRequest {
        query: String::from(r#"AI ML NLP"#),
        publication_date_or_year: String::from("2019:"),
        min_citation_count: 3,
        // ..BulkRequest::default()
    })
    .await
    .unwrap();
    let papers = result.data.unwrap();
    SearchPageLayoutTemplate {
        total_count: result.total,
        rows: papers
            .into_iter()
            .map(TableRowTemplate::from)
            .collect::<Vec<TableRowTemplate>>(),
    }
}

pub async fn search_paper(
    hx_boosted: HxBoosted,
    RawForm(form_set): RawForm,
) -> SearchResultTemplate {
    println!("hx_boosted : {:#?}", hx_boosted);
    println!("form_set: {:#?}", form_set);
    let form_set_extract = String::from_utf8_lossy(&form_set)
        .parse::<QueryMap>()
        .unwrap();
    let result = fetch_papers(BulkRequest {
        query: form_set_extract.first("query").unwrap().to_string(),
        publication_date_or_year: form_set_extract
            .first("publication_date_or_year")
            .unwrap()
            .to_string(),
        // min_citation_count: form_set.min_citation_count,
        ..BulkRequest::default()
    })
    .await
    .unwrap();
    // println!("result: {:#?}", result);
    SearchResultTemplate {
        query: form_set_extract.to_query_string(),
        total_count: result.total,
        rows: result
            .data
            .unwrap()
            .into_iter()
            .map(TableRowTemplate::from)
            .collect::<Vec<TableRowTemplate>>(),
    }
}

// async fn path(Path(user_id): Path<u32>) {}
pub async fn paper_detail(Path(paper_id): Path<String>) -> PaperDetailTemplate {
    // println!("paper_id: {:#?}", paper_id);
    let paper = fetch_paper_detail(paper_id.to_owned()).await.unwrap();
    PaperDetailTemplate {
        paper_id: paper_id.to_owned(),
        fetched: false,
        paper_detail: PaperDetailTemplateDetailPrint::from(paper),
        // body: "Hello, world!".to_string(),
    }
}

pub async fn paper_references(Path(paper_id): Path<String>) -> CitingListResponse {
    // println!("paper_id: {:#?}", paper_id);
    let paper = fetch_references(CitingRequest {
        paper_id: paper_id.to_owned(),
        // limit: 100,
        ..CitingRequest::default()
    })
    .await
    .unwrap();
    CitingListResponse {
        table_id: "reference-accordion-collapse-body".to_string(),
        offset: paper.offset,
        next: paper.next,
        rows: paper
            .data
            .unwrap()
            .into_iter()
            .map(CitingListRowTemplate::from)
            .collect::<Vec<CitingListRowTemplate>>(),
    }
}

pub async fn paper_citation(Path(paper_id): Path<String>) -> CitingListResponse {
    // println!("paper_id: {:#?}", paper_id);
    let paper = fetch_citing(CitingRequest {
        paper_id: paper_id.to_owned(),
        // limit: 100,
        ..CitingRequest::default()
    })
    .await
    .unwrap();
    CitingListResponse {
        table_id: "citation-accordion-collapse-body".to_string(),
        offset: paper.offset,
        next: paper.next,
        rows: paper
            .data
            .unwrap()
            .into_iter()
            .map(CitingListRowTemplate::from)
            .collect::<Vec<CitingListRowTemplate>>(),
    }
}

pub fn create_router_service() -> IntoMakeService<Router> {
    Router::new()
        .route("/", get(paper_index))
        .route("/x/paper_search", post(search_paper))
        .route("/x/paper/:paper_id", get(paper_detail))
        .route("/x/paper/:paper_id/references", get(paper_references))
        .route("/x/paper/:paper_id/citations", get(paper_citation))
        // .route("/api", get(json))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT]),
        )
        .into_make_service()
}
