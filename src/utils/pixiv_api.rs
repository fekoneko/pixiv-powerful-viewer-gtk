use ureq::http::Uri;
use ureq::{Agent, Body};

use crate::models::*;

pub fn fetch_work_metadata(
    agent: &Agent,
    session_id: &str,
    id: usize,
) -> Result<PixivWorkMetadata, ureq::Error> {
    let uri = format!("https://www.pixiv.net/ajax/illust/{}", id);
    let uri = Uri::from_maybe_shared(uri).unwrap();

    let work_metadata = agent
        .get(uri)
        .header("User-Agent", "Mozilla/5.0")
        .header("Cookie", format!("PHPSESSID={}", session_id))
        .call()?
        .into_body()
        .read_json::<PixivResponse<PixivWorkMetadata>>()?
        .body;

    Ok(work_metadata)
}

pub fn fetch_work_pages(
    agent: &Agent,
    session_id: &str,
    id: usize,
) -> Result<Vec<PixivWorkPage>, ureq::Error> {
    let uri = format!("https://www.pixiv.net/ajax/illust/{}/pages", id);
    let uri = Uri::from_maybe_shared(uri).unwrap();

    let work_pages = agent
        .get(uri)
        .header("User-Agent", "Mozilla/5.0")
        .header("Cookie", format!("PHPSESSID={}", session_id))
        .call()?
        .into_body()
        .read_json::<PixivResponse<Vec<PixivWorkPage>>>()?
        .body;

    Ok(work_pages)
}

pub fn fetch_image(agent: &Agent, uri: &str) -> Result<Body, ureq::Error> {
    let image = agent
        .get(uri)
        .header("Referer", "https://www.pixiv.net/")
        .call()?
        .into_body();

    Ok(image)
}
