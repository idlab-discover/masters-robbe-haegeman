use std::{
    collections::BTreeMap,
    fs::OpenOptions,
    io::{self, Write},
};

use k8s_openapi::{
    ByteString, Metadata, api::core::v1::Secret,
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    Api, Client, CustomResourceExt,
    api::{ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;
use serde::Serialize;
use tracing::{debug, trace};

use crate::crd::Database;

#[derive(Debug, Serialize, Default)]
pub struct Case {
    pub nr_resources: usize,
    pub nr_kinds: usize,
    pub duration_get_latest: Vec<u128>,
    pub duration_direct: Vec<u128>,
}

impl Case {
    pub fn new(nr_resources: usize, nr_kinds: usize) -> Self {
        Self {
            nr_resources,
            nr_kinds,
            ..Default::default()
        }
    }
}

pub fn append_case_to_file(case: &Case, file_path: &str) -> io::Result<()> {
    // Serialize the case to a JSON string
    let json = serde_json::to_string(case)?;

    // Open the file in append mode, create it if it doesn't exist
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    // Append the JSON string followed by a newline
    writeln!(file, "{}", json)?;

    Ok(())
}

pub async fn apply_database_crd(client: Client) -> CustomResourceDefinition {
    let crds: Api<CustomResourceDefinition> = Api::all(client);
    let crd = Database::crd();
    let name = crd.metadata.name.as_ref().unwrap();

    match crds.get_opt(name).await.unwrap() {
        Some(crds) => crds,
        None => {
            debug!("Creating CRD");
            crds.create(&PostParams::default(), &crd).await.unwrap();

            // The create command waits for the resource to be created, not until the CRD is registered.
            // Verify that the CRD is available
            // https://stackoverflow.com/questions/57115602/how-to-kubectl-wait-for-crd-creation
            loop {
                let crd = crds.get_opt(name).await.unwrap().unwrap();
                trace!("{:?}", crd);

                if let Some(conditions) = crd.status.as_ref().and_then(|s| s.conditions.as_ref()) {
                    if conditions.iter().any(|c| c.type_ == "Established") {
                        break crd;
                    }
                }
            }
        }
    }
}

pub async fn create_test_secrets(client: Client, db: &mut Database, amount: usize) {
    let data = Secret {
        metadata: ObjectMeta {
            name: Some(String::from("test-secret")),
            namespace: Some(String::from("poc-testing")),
            ..Default::default()
        },
        data: {
            let mut data_map = BTreeMap::new();
            data_map.insert(
                String::from("test_key"),
                ByteString("test_value".as_bytes().to_vec()),
            );
            Some(data_map)
        },
        ..Default::default()
    };
    for i in 0..amount {
        let mut data = data.clone();
        data.metadata_mut().name = Some(format!("test-secret-{i}"));
        db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
            .await
            .unwrap();
    }
}
