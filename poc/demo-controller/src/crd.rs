use kube::{api::DynamicObject, core::object::HasStatus, CustomResource};
use kube_primary::PrimaryResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(CustomResource, Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[kube(group = "poc.sec.res", version = "v1", kind = "Database", namespaced)]
#[kube(status = "DatabaseStatus", shortname = "db")]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSpec {
    #[serde(rename = "CRVersion")]
    pub cr_version: String,
    pub backup_enabled: bool, // Normally part of another BackupSpec
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    // Note: these will have an OwnerReference as well, but there is no way in Kubernetes to query them
    // apart from getting all resources and filtering on the value
    #[serde(skip)]
    pub sec_recs: Vec<DynamicObject>,
}

impl PrimaryResource for Database {
    fn cache_secondary_status_dependent(&self) -> Option<&Vec<DynamicObject>> {
        self.status().map(|status| &status.sec_recs)
    }
    fn cache_secondary_mut_status_dependent(&mut self) -> Option<&mut Vec<DynamicObject>> {
        self.status_mut()
            .as_mut()
            .map(|status| &mut status.sec_recs)
    }
}
