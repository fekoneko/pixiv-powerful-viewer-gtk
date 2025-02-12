use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixivResponse<TBody> {
    pub body: TBody,
}
