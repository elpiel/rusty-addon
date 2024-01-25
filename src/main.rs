use axum::{extract::Path, http::Method, response::IntoResponse, routing::get, Json, Router};

use stremio_core::types::addon::{Manifest, ResourceResponse};

use rusty_addon::{
    constant::MANIFEST,
    resources::{Resource, ResourceType},
    routes::{
        catalog_resource::{handle_catalog_resource, handle_catalog_resource_last_videos},
        stream_resource::handle_stream_resource,
    },
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // let resources_router = Router::new().route("/users/:id", get(users_get));

    // build our application with a single route
    let app = Router::new()
        .route("/manifest.json", get(get_manifest))
        // .nest("/:resource")
        .route("/catalog/series/last-videos/*path", get(get_series_last_videos))
        .route("/:resource/:type/:id", get(get_resource))
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin(Any)
                // .allow_origin([
                //     // local stremio-web
                //     "http://localhost:8080".parse::<HeaderValue>().unwrap(),
                //     "https://localhost:8080".parse::<HeaderValue>().unwrap(),
                //     "https://web.stremio.com".parse::<HeaderValue>().unwrap(),
                //     "https://stremio.github.io/stremio-web"
                //         .parse::<HeaderValue>()
                //         .unwrap(),
                // ])
                .allow_methods([Method::GET]),
        );

    info!("Starting addon on localhost:3000");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_manifest() -> Json<Manifest> {
    Json(MANIFEST.clone())
}

///
/// # Examples
///
/// - `https://v3-cinemeta.strem.io/catalog/series/last-videos/lastVideosIds=tt1254207.json`
/// Resource: [`Resource::Catalog`]
/// Resource Type: [`ResourceType::Series`]
async fn get_series_last_videos(Path(path): Path<String>) -> impl IntoResponse {
    info!("Get /catalog/series/last-videos; Path {path}");
    handle_catalog_resource_last_videos(&path).await
}

async fn get_resource(
    Path((resource, resource_type, endpoint_id)): Path<(Resource, ResourceType, String)>,
) -> Json<ResourceResponse> {
    info!("{resource:?} {resource_type:?} {endpoint_id}");

    match resource {
        Resource::Stream => handle_stream_resource(resource_type, &endpoint_id).await,
        Resource::Catalog => handle_catalog_resource(resource_type, &endpoint_id).await,
    }
}
