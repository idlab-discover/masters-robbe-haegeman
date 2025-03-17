use axum::{Json, Router, response::IntoResponse, routing::get};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{APIResource, APIResourceList};
use kube::Resource;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod resources;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("apis/farm.example.com/v1alpha", get(get_api_resources))
        .route(
            "/apis/farm.example.com/v1alpha/namespaces/:namespace/llamas",
            get(resources::list_llamas),
        )
        .route(
            "/apis/farm.example.com/v1alpha/namespaces/:namespace/llamas/:name",
            get(resources::get_llama),
        )
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // run our app with hyper, listening globally on port 3000
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
