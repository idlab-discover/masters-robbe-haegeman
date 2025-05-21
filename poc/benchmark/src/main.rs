use std::{sync::Once, time::SystemTime};

use crd::Database;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    Api, Client, ResourceExt,
    api::{ListParams, ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub mod crd;

static INIT: Once = Once::new();

fn create_tracing_subscriber() {
    INIT.call_once(|| {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .init();
    });
}

#[tokio::main]
async fn main() {
    create_tracing_subscriber();
    let client = Client::try_default().await.unwrap();
    let db = Database {
        metadata: ObjectMeta {
            name: Some(String::from("test")),
            namespace: Some(String::from("poc-testing")),
            ..Default::default()
        },
        ..Default::default()
    };

    // Create an Api client for the `Database` CRD
    let db_api: Api<Database> = Api::namespaced(client.clone(), "poc-testing");
    let db = match db_api.get_opt(&db.name_any()).await.unwrap() {
        Some(db) => db,
        None => db_api.create(&PostParams::default(), &db).await.unwrap(),
    };

    let start = SystemTime::now();

    let result = db.get_latest_with_secondaries(client.clone()).await;

    assert!(result.is_ok());

    let end = SystemTime::now();
    if let Ok(duration) = end.duration_since(start) {
        info!(
            benchmark = true,
            time = duration.as_micros(),
            "get_latest_with_secondaries"
        );
    }

    let start = SystemTime::now();

    let db_api: Api<Database> = Api::namespaced(client.clone(), "poc-testing");
    let db = db_api.get(&db.name_any()).await;

    assert!(db.is_ok());

    let secret_api: Api<Secret> = Api::namespaced(client.clone(), "poc-testing");
    let secrets = secret_api.list(&ListParams::default()).await;

    assert!(secrets.is_ok());

    let end = SystemTime::now();
    if let Ok(duration) = end.duration_since(start) {
        info!(
            benchmark = true,
            time = duration.as_micros(),
            "direct listing"
        );
    }
}
