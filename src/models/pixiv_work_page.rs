use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixivWorkPage {
    pub urls: PixivWorkPageUrls,
}

#[derive(Deserialize)]
// NOTE: In Pixiv API snake_case is used here
pub struct PixivWorkPageUrls {
    pub thumb_mini: String,
    pub small: String,
    pub regular: String,
    pub original: String,
}
