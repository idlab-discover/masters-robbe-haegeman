use std::net::SocketAddr;

use axum::Json;
use axum::{Router, response::IntoResponse, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::level_filters::LevelFilter;
use tracing::{Level, span, trace};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const GROUP: &str = "poc.sec.res.kinds";
const VERSION: &str = "v1";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let app = Router::new()
        .route("/apis/poc.sec.res.kinds/v1/health", get(get_health))
        .route("/apis/poc.sec.res.kinds/v1", get(get_api_resources))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        );

    let span = span!(Level::TRACE, "Key management");
    let handle = span.enter();
    let tls_cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
    trace!("Generated certificate: {}", tls_cert.cert.pem());
    trace!(
        "Generated private key: {}",
        tls_cert.key_pair.serialize_pem()
    );
    let tls_config = RustlsConfig::from_pem(
        tls_cert.cert.pem().into_bytes(),
        tls_cert.key_pair.serialize_pem().into_bytes(),
    )
    .await
    .unwrap();
    drop(handle);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_health() -> impl IntoResponse {
    "OK"
}

async fn get_api_resources() -> impl IntoResponse {
    Json(APIResourceList {
        group_version: std::format!("{}/{}", GROUP, VERSION),
        resources: vec![],
    })
}
