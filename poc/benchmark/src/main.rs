use std::{mem::ManuallyDrop, time::SystemTime};

use case::{Case, append_case_to_file, create_test_secrets};
use crd::Database;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    Api, Client, ResourceExt,
    api::{ListParams, ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;
use sysinfo::System;

pub mod case;
pub mod crd;

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
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
    let mut db = match db_api.get_opt(&db.name_any()).await.unwrap() {
        Some(db) => db,
        None => db_api.create(&PostParams::default(), &db).await.unwrap(),
    };

    // create_test_secrets(client.clone(), &mut db, 5).await;

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

        print!("Before");
        get_memory_usage(&mut sys);
        let db_api: Api<Database> = Api::namespaced(client.clone(), "poc-testing");
        let db = db_api.get(&db.name_any()).await;
        let mut db = ManuallyDrop::new(db);
        print!("After");
        get_memory_usage(&mut sys);
        assert!(db.is_ok());
        unsafe {
            ManuallyDrop::drop(&mut db);
        }

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

fn get_memory_usage(sys: &mut System) {
    sys.refresh_all();
    if let Some(process) = sys.process(sysinfo::get_current_pid().unwrap()) {
        println!("Memory usage: {} KB", process.memory());
    } else {
        println!("Process not found");
    }
}
