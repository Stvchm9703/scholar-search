mod axum_server;
mod semantic_scholar_api;
use crate::axum_server::create_router_service;

// axum service

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        // .with_timer(tracing_subscriber::fmt::time::uptime())
        // .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // let app = Router::new().nest("/", page_service());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3600").await.unwrap();
    axum::serve(listener, create_router_service().into_make_service())
        .await
        .unwrap();
}

// let result = fetch_papers(BulkRequest {
//     query: String::from(r#"AI ML NLP"#),
//     publication_date_or_year: String::from("2019:"),
//     min_citation_count: 3,
//     ..BulkRequest::default()
// })
// .await
// .unwrap();
// let papers = result.data.unwrap();

// let y = papers[0].clone();
// println!("result: {:#?}", y.paper_id);

// let critions = fetch_citing(CitingRequest {
//     paper_id: y.paper_id.to_owned(),
//     limit: 100,
//     ..CitingRequest::default()
// })
// .await
// .unwrap();
// println!("criting: {:#?}", critions);

// let refs = fetch_citing(CitingRequest {
//     paper_id: y.paper_id.to_owned(),
//     limit: 100,
//     ..CitingRequest::default()
// }).await.unwrap();
// println!("refs: {:#?}", refs);
