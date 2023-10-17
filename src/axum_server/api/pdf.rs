use anyhow::Error;
use reqwest;
use std::io::Cursor;
use tracing::warn;

use convert_case::{Case, Casing};
use lopdf;

use std::fs;
pub async fn pdf_download(doi: &str, url: &str) -> Result<(), Error> {
    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_static("Mozilla/5.0"),
    );
    let client = reqwest::Client::builder()
        .default_headers(header)
        .cookie_store(true)
        .build()?;
    let resp = client.get(url).send().await?;

    // check path is exist
    fs::create_dir_all("data/pdf_temp")?;
    let doi_path = doi
        .to_case(Case::Snake)
        .replace('/', "__")
        .replace('.', "__");
    let mut out = std::fs::File::create(format!("data/pdf_temp/{}.pdf", doi_path))?;
    let content_bytes = resp.bytes().await?;
    let mut content = Cursor::new(content_bytes.to_owned());
    std::io::copy(&mut content, &mut out)?;

    if let Ok(content_str) = String::from_utf8(content_bytes.to_vec()) {
        println!("content_str: {:#?}", content_str);
        if content_str.contains(r#"<head>"#) {
            warn!("not a pdf format: {:?}", doi);
            return Err(anyhow::anyhow!("content_str: {:#?}", content_str));
        }
    }
    Ok(())
}

pub async fn convert_pdf_to_text(doi: &str) -> Result<(), Error> {
    let doi_path = doi
        .to_case(Case::Snake)
        .replace('/', "__")
        .replace('.', "__");
    let doc = lopdf::Document::load(format!("data/pdf_temp/{}.pdf", doi_path))?;
    let toc = doc.get_toc().unwrap();
    println!("toc: {:#?}", toc);
    let _pages = doc.get_pages();
    // for page in pages {
    //     let content = doc.get_content().unwrap();
    //     println!("content: {:#?}", content);
    // }
    Ok(())
}

mod test {
    
    #[tokio::test]
    async fn test_pdf_download() {
        println!(
            "test_pdf_download {}",
            "10.1145/3292500.3330648".to_case(Case::Snake)
        );
        pdf_download(
            "10.1145/3292500.3330648",
            "https://dl.acm.org/doi/pdf/10.1145/3292500.3330648",
        )
        .await
        .unwrap();
        // convert_pdf_to_text("10.1145/3292500.3330648").await.unwrap();
    }
}
