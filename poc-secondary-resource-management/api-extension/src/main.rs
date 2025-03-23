use axum::{Json, Router, response::IntoResponse, routing::get};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{APIResource, APIResourceList};
use kube::Resource;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use tower_http::trace::{DefaultOnRequest, DefaultOnResponse};
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod resources;

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
        .route("/apis/farm.example.com/v1alpha", get(get_api_resources))
        .route(
            "/apis/farm.example.com/v1alpha/namespaces/{namespace}/llamas",
            get(resources::list_llamas),
        )
        .route(
            "/apis/farm.example.com/v1alpha/namespaces/{namespace}/llamas/{name}",
            get(resources::get_llama),
        )
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_api_resources() -> impl IntoResponse {
    Json(APIResourceList {
        group_version: String::from("farm.example.com/v1alpha"),
        resources: vec![APIResource {
            group: Some(resources::Llama::group(&()).into()),
            kind: resources::Llama::kind(&()).into(),
            name: resources::Llama::plural(&()).into(),
            namespaced: true,
            verbs: vec![String::from("list"), String::from("get")],
            ..Default::default()
        }],
    })
}
