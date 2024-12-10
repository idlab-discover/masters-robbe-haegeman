use crate::error::{Error, Result};
use kube::{
    api::{DynamicObject, PostParams},
    Api, Client,
};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

impl PrimaryResource for Database {}

pub(crate) trait PrimaryResource: kube::ResourceExt {
    async fn create_secondary<
        T: kube::Resource<Scope = k8s_openapi::NamespaceResourceScope>
            + Clone
            + Debug
            + Serialize
            + DeserializeOwned,
    >(
        &self,
        client: Client,
        pp: &mut PostParams,
        data: &T,
    ) -> Result<T>
    where
        <T as kube::Resource>::DynamicType: Default,
    {
        let secondary_api: Api<T> =
            Api::namespaced(client, &self.namespace().unwrap_or(String::from("default")));

        secondary_api
            .create(pp, data)
            .await
            .map_err(Error::KubeError)
    }
}
