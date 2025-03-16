use axum::{Json, Router, extract::Path, routing::get};
use serde_json::{Value, json};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/{user_id}", get(basic_handler))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn basic_handler(Path(user_id): Path<u32>) -> Json<Value> {
    Json(json!({"user_id": user_id}))
}
