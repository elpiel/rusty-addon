use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use once_cell::sync::Lazy;

use stremio_core::types::resource::MetaItem;

/// For All Mankind
/// <https://www.imdb.com/title/tt7772588>
pub static FOR_ALL_MANKIND_META_ITEM: Lazy<MetaItem> = Lazy::new(|| {
    serde_json::from_str(include_str!("../assets/for_all_mankind_19_1_2024.json")).unwrap()
});

/// Demon Slayer: Kimetsu no Yaiba
/// <https://www.imdb.com/title/tt9335498>
pub static KIMETSU_META_ITEM: Lazy<MetaItem> =
    Lazy::new(|| serde_json::from_str(include_str!("../assets/kimetsu_19_1_2024.json")).unwrap());

// handle errors by converting them into something that implements
// `IntoResponse`
async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {err}"),
    )
}

pub mod catalog_resource {
    use axum::{http::StatusCode, response::IntoResponse, Json};
    use stremio_core::types::{addon::ResourceResponse, resource::MetaItem};
    use tracing::{info, warn};

    use crate::{constant::MANIFEST, manifest_ext::ManifestExt, resources::ResourceType};

    use super::{FOR_ALL_MANKIND_META_ITEM, KIMETSU_META_ITEM};

    /// # Example
    ///
    /// `https://v3-cinemeta.strem.io/catalog/series/last-videos/lastVideosIds=tt1254207.json`
    /// Where `path == "lastVideosIds=tt1254207.json"`
    pub async fn handle_catalog_resource_last_videos(
        // resource_type: ResourceType,
        path: &str,
    ) -> Result<Json<ResourceResponse>, StatusCode> {
        info!("Handling LastVideos: {path}");
        let ids = path
            .strip_prefix("lastVideosIds=")
            .and_then(|stripped_prefix| stripped_prefix.strip_suffix(".json"))
            .ok_or(StatusCode::NOT_FOUND)?;

        // on empty ids `split` will return empty string for the first element!
        let split_ids = ids.split(',').collect::<Vec<_>>();
        // only continue if the first id is not empty
        if split_ids
            .get(0)
            .map(|first_id| !first_id.is_empty())
            .unwrap_or_default()
        {
            // filter by supported prefix
            let ids: Vec<_> = split_ids
                .into_iter()
                .filter(|id| MANIFEST.is_id_supported(id))
                .collect();

            let metas_detailed = ids
                .iter()
                .filter_map(|id| {
                    match *id {
                        // For All Mankind
                        // https://www.imdb.com/title/tt7772588/
                        "tt7772588" => {
                            info!("Matched id: {id}");
                            Some(FOR_ALL_MANKIND_META_ITEM.clone())
                        }
                        // Demon Slayer: Kimetsu no Yaiba
                        // https://www.imdb.com/title/tt9335498/
                        "kitsu:44081" => {
                            info!("Matched id: {id}");
                            Some(KIMETSU_META_ITEM.clone())
                        }
                        unmatched => {
                            warn!("Unmatched id: `{unmatched}`");
                            None
                        }
                    }
                })
                .collect();

            Ok(Json(ResourceResponse::MetasDetailed { metas_detailed }))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
        // .filter_map(|ids| {
        //     ids.
        // });

        // Ok(Json(ResourceResponse::MetasDetailed { metas_detailed: vec![] }))
        // match resource_type {
        //     ResourceType::Series => {
        //     },
        //     ResourceType::Movie => Err(StatusCode::NOT_FOUND),
        // }
    }
    pub async fn handle_catalog_resource(
        resource_type: ResourceType,
        endpoint_id: &str,
    ) -> Json<ResourceResponse> {
        match resource_type {
            ResourceType::Series => todo!(),
            ResourceType::Movie => todo!("Error"),
        }
    }
}

pub mod stream_resource {
    use axum::Json;
    use hex::FromHex;

    use stremio_core::{
        constants::STREAM_RESOURCE_NAME,
        types::{
            addon::{Manifest, ResourcePath, ResourceResponse},
            resource::{Stream, StreamBehaviorHints, StreamProxyHeaders, StreamSource},
        },
    };
    use tracing::info;

    use crate::resources::ResourceType;

    const BIG_BUCK_BUNNY_720P: &str =
        "https://download.blender.org/peach/bigbuckbunny_movies/big_buck_bunny_720p_h264.mov";

    pub async fn handle_stream_resource(
        resource_type: ResourceType,
        endpoint_id: &str,
    ) -> Json<ResourceResponse> {
        let stream = match (resource_type, endpoint_id) {
            (ResourceType::Movie, "tt1254207.json") => Stream {
                source: StreamSource::Url {
                    url: BIG_BUCK_BUNNY_720P.parse().unwrap(),
                },
                name: Some("720p".into()),
                description: Some(format!("720p source for {endpoint_id}")),
                thumbnail: None,
                subtitles: vec![],
                behavior_hints: StreamBehaviorHints {
                    not_web_ready: false,
                    binge_group: Some("1".into()),
                    country_whitelist: None,
                    proxy_headers: Some(StreamProxyHeaders {
                        request: vec![("req-header-1".to_string(), "value-1".to_string())]
                            .into_iter()
                            .collect(),
                        response: vec![("resp-header-1".to_string(), "value-1".to_string())]
                            .into_iter()
                            .collect(),
                    }),
                    other: Default::default(),
                },
            },
            // Big hero 6
            (ResourceType::Series, "tt13622776:1:5.json") => Stream {
                source: StreamSource::Torrent {
                    info_hash: <[u8; 20]>::from_hex("ba44b8864cfb3ee13a7a20f8d2687baa1b9d5351")
                        .unwrap(),
                    file_idx: Some(0),
                    announce: vec![],
                },
                name: Some("Torrentio (fake) \n480p".into()),
                description: Some(
                    "Ahsoka.S01E05.480p.x264-RUBiK\n👤 7 💾 293.24 MB ⚙️ ThePirateBay".into(),
                ),
                thumbnail: None,
                subtitles: vec![],
                behavior_hints: StreamBehaviorHints {
                    not_web_ready: false,
                    binge_group: Some("torrentio|480p|RUBiK".into()),
                    ..Default::default()
                },
            },
            _ => return Json(ResourceResponse::Streams { streams: vec![] }),
        };
        // let resource_path = ResourcePath {
        //     resource: STREAM_RESOURCE_NAME.into(),
        //     r#type: "movie".into(),
        //     id: endpoint_id.clone(),
        //     extra: vec![],
        // };
        let response = ResourceResponse::Streams {
            streams: vec![stream],
        };
        info!("Big Buck Bunny stream should be returned: {response:?}");

        Json(response)
    }
}
