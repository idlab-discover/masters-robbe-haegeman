#![allow(clippy::derivable_impls)]

use kube::{api::DynamicObject, core::object::HasStatus, CustomResource};
use kube_primary::PrimaryResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

impl Default for Database {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            spec: Default::default(),
            status: Default::default(),
        }
    }
}
