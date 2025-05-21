use std::time::SystemTime;

use case::{Case, Measurement, append_case_to_file};
use crd::Database;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    Api, Client, ResourceExt,
    api::{ListParams, ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;

pub mod case;
pub mod crd;

#[tokio::main]
async fn main() {
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

    let mut case = Case::default();

    for _ in 0..100 {
        let mut measurement = Measurement::default();
        let start = SystemTime::now();

        let result = db.get_latest_with_secondaries(client.clone()).await;

        assert!(result.is_ok());

        let end = SystemTime::now();
        if let Ok(duration) = end.duration_since(start) {
            measurement.duration_get_latest = duration.as_micros();
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
            measurement.duration_direct = duration.as_micros();
        }

        case.measurements.push(measurement);
    }
    append_case_to_file(&case, "./result.jsonl").unwrap();
}
