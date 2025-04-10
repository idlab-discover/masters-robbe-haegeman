#![allow(clippy::derivable_impls)]

use kube::{CustomResource, ResourceExt, api::DynamicObject, core::object::HasStatus};
use lib::PrimaryResource;
use lib::error::{Error, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::info;

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
    fn initialize_status(&mut self) {
        info!("Initializing status");
        self.status = Some(DatabaseStatus {
            ..Default::default()
        })
    }

    fn cache_secondary(&self) -> Result<&Vec<DynamicObject>> {
        Ok(&self
            .status()
            .ok_or(Error::MissingStatusError(self.name_any()))?
            .sec_recs)
    }

    fn cache_secondary_mut(&mut self) -> Result<&mut Vec<DynamicObject>> {
        info!("Requesting secondary resources");
        let name = self.name_any().clone();
        if let Some(status) = self.status_mut() {
            return Ok(&mut status.sec_recs);
        }
        Err(Error::MissingStatusError(name))
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
