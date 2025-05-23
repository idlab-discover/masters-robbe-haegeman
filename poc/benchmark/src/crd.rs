#![allow(clippy::derivable_impls)]

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    Api, Client, CustomResource, CustomResourceExt,
    api::{DynamicObject, PostParams},
    core::object::HasStatus,
};
use kube_primary::PrimaryResourceExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::{debug, trace};

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "poc.sec.res", version = "v1", kind = "Database", namespaced)]
#[kube(status = "DatabaseStatus", shortname = "db")]
pub struct DatabaseSpec {}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    #[serde(skip)]
    pub sec_recs: Vec<DynamicObject>,
}

impl PrimaryResourceExt for Database {
    fn cache_secondary_status_dependent(&self) -> Option<&Vec<DynamicObject>> {
        self.status().map(|status| &status.sec_recs)
    }

    fn cache_secondary_mut_status_dependent(&mut self) -> Option<&mut Vec<DynamicObject>> {
        self.status_mut()
            .as_mut()
            .map(|status| &mut status.sec_recs)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            spec: Default::default(),
            status: Default::default(),
        }
    }
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
