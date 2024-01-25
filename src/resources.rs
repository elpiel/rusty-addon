use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Resource {
    Stream,
    /// `https://v3-cinemeta.strem.io/catalog/series/last-videos/lastVideosIds=tt1254207.json`
    Catalog,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceType {
    Series,
    Movie,
}
