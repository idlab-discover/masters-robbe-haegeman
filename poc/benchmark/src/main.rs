use std::time::SystemTime;

use case::{Case, append_case_to_file, apply_database_crd, create_test_secrets};
use crd::Database;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    Api, Client, ResourceExt,
    api::{ListParams, ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub mod case;
pub mod crd;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::ERROR.into())
                .from_env_lossy(),
        )
        .init();

    let client = Client::try_default().await.unwrap();

    apply_database_crd(client.clone()).await;

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
    let mut db = match db_api.get_opt(&db.name_any()).await.unwrap() {
        Some(db) => db,
        None => db_api.create(&PostParams::default(), &db).await.unwrap(),
    };

    create_test_secrets(client.clone(), &mut db, 5).await;

    let mut case = Case::new(5, 1);

    for _ in 0..100 {
        let start = SystemTime::now();

        let result = db.get_latest_with_secondaries(client.clone()).await;

        assert!(result.is_ok());

        let end = SystemTime::now();
        if let Ok(duration) = end.duration_since(start) {
            case.duration_get_latest.push(duration.as_micros());
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
            case.duration_direct.push(duration.as_micros());
        }
    }
    append_case_to_file(&case, "./result.jsonl").unwrap();
}
