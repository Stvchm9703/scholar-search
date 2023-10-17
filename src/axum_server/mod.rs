pub mod api;
pub mod template;
use crate::axum_server::template::{
    page_detail::{CitingListResponse, CitingListRowTemplate},
    search_page::{SearchPageLayoutTemplate, SearchResultTemplate},
    table::{PaperDetailTemplate, PaperDetailTemplateDetailPrint, TableRowTemplate},
};

use axum::{
    extract,
    extract::{Form, Path, RawForm},
    routing::{get, post},
    Router,
};

use crate::semantic_scholar_api::{
    critions::{fetch_citing, fetch_references, CitingRequest},
    paper_fetch::{fetch_paper_detail, fetch_papers, BulkRequest},
};

use crate::axum_server::api::pdf::pdf_download;
use axum_htmx::HxBoosted;
use query_map::QueryMap;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug)]
pub struct PaperCloneRequest {
    doi: String,
    url: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PaperCloneResponse {
    status: String,
}

pub async fn api_paper_clone(
    Form(payload): Form<PaperCloneRequest>,
) -> axum::Json<PaperCloneResponse> {
    // println!("payload: {:#?}", payload);
    println!("here");
    // tokio::spawn(async move {
    // pdf_download("10.1145/3292500.3330648", "https://dl.acm.org/doi/pdf/10.1145/3292500.3330648").await.unwrap();
    //    match  pdf_download(&payload.doi, &payload.url).await {
    //         Ok(_) => {
    //             println!("pdf_download success");
    //         }
    //         Err(e) => {
    //             println!("pdf_download error: {:#?}", e);
    //         }
    //     }
    // match convert_pdf_to_text(&payload.doi).await {
    //     Ok(_) => {
    //         println!("convert_pdf_to_text success");
    //     }
    //     Err(e) => {
    //         println!("convert_pdf_to_text error: {:#?}", e);
    //

    // });
    axum::Json(PaperCloneResponse {
        status: "accpeted".to_string(),
    })
}

pub fn create_router_service() -> Router {
    let page_route = Router::new()
        .route("/", get(paper_index))
        .route("/x/paper_search", post(search_paper))
        .route("/x/paper/:paper_id", get(paper_detail))
        .route("/x/paper/:paper_id/references", get(paper_references))
        .route("/x/paper/:paper_id/citations", get(paper_citation));

    let api_route = Router::new().route("/paper/clone", post(api_paper_clone));

    Router::new().nest("/", page_route).nest("/api", api_route)
    // .nest(
    //     "/static",
    //     axum::service::get(axum_static_service::new(std::path::Path::new("./static"))),
    // )
}
